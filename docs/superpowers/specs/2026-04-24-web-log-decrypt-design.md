# Web-based Encrypted Log Decryption

**Date:** 2026-04-24
**Status:** Approved

## Problem

Client and server binaries produce envelope-encrypted runtime log files (`.log.enc`, format `ATLG`+RSA-wrapped AES key+AES-GCM frames, introduced in commit `0de569a8`). Today the only way to read them is to copy the file off the machine and run `cloud/backend/tools/decrypt_log.py` with the RSA private key. Operators want to do this from the admin web UI instead.

## Goals

- Admins upload a `.log.enc` file through the admin UI and immediately see plaintext lines.
- No shell / script access required.
- Plaintext never persists in the cloud DB or on disk.
- Single source of truth for the decryption logic (reused by the existing CLI script).

## Non-goals (YAGNI)

- Persisting decrypted logs for later browsing.
- Bulk / multi-file uploads.
- Log parsing, level highlighting, timestamp extraction — raw text only in v1.
- Streaming responses (the 20 MB size cap makes one-shot fine).

## Architecture

```
[Admin browser]
   |
   |  POST /api/admin/logs/decrypt  (multipart file, cookie session)
   v
[FastAPI admin router]
   |  require_admin dependency
   |  size check (<=20 MB)
   v
[app.log_decrypt.decrypt_log_bytes(data, private_key)]
   |  validates ATLG magic + version
   |  unwraps AES key with cloud RSA private key
   |  yields UTF-8 lines (lossy on invalid bytes)
   v
[JSON response: {filename, total_lines, truncated, lines[]}]
   |
   v
[ClientLogs.vue: renders / searches / downloads]
```

Private key never leaves the backend. The browser only sees plaintext already in its possession (it uploaded the ciphertext, now it gets plaintext back).

## Backend

### New shared helper — `cloud/backend/app/log_decrypt.py`

```python
from typing import Iterator
from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey

MAGIC = b"ATLG"
VERSION = 1

class LogDecryptError(ValueError):
    pass

def decrypt_log_bytes(data: bytes, private_key: RSAPrivateKey) -> Iterator[str]:
    """Yield plaintext lines from a .log.enc blob.

    Raises LogDecryptError on bad magic, unknown version, or truncated frames.
    """
```

- Validates `data[:4] == MAGIC`, `version == 1`.
- Unwraps the AES-256 key using `private_key.decrypt(..., OAEP/SHA-256)`.
- Iterates `[4B len][nonce+ct+tag]` frames; each frame decrypted via `AESGCM`; decoded `utf-8` with `errors="replace"`.
- Raises `LogDecryptError` (not SystemExit) on any format error.

### Refactor `cloud/backend/tools/decrypt_log.py`

Replace the inlined loop with `from app.log_decrypt import decrypt_log_bytes, LogDecryptError`. CLI behavior unchanged.

### New endpoint — `cloud/backend/app/api/admin.py`

```python
@router.post("/logs/decrypt")
async def decrypt_client_log(
    file: UploadFile = File(...),
    _admin: User = Depends(require_admin),
):
    ...
```

Rules:
- `file.size > MAX_BYTES (20 * 1024 * 1024)` → `HTTPException(400, "文件过大")`.
- Read bytes, call `decrypt_log_bytes(data, crypto.private_key)`.
- `LogDecryptError` → `HTTPException(400, str(e))`.
- Collect up to `MAX_LINES = 50_000` lines; if the iterator yields more, set `truncated = True` and stop consuming.
- Response schema (new in `app/schemas.py`):

```python
class LogDecryptResponse(BaseModel):
    filename: str
    total_lines: int     # number of lines returned (<= MAX_LINES)
    truncated: bool
    lines: list[str]
```

Filename echoed back is `file.filename or "uploaded.log.enc"`.

### Wiring

`app/main.py` needs access to `CryptoManager` in the handler. It's instantiated as `crypto = CryptoManager(RSA_KEY_DIR)` at module scope. The admin router can import it (`from app.main import crypto`) — existing `verify.py` follows the same pattern; verify before copying.

If circular-import risk, expose via a dependency:
```python
def get_crypto() -> CryptoManager: ...
```
in a new `app/deps.py` (only if needed).

### Tests — `cloud/backend/tests/test_log_decrypt.py`

Uses the existing pytest/`TestClient` fixtures. Cases:

1. **Roundtrip:** Generate fixture bytes in Python that match the Rust format (`ATLG`+RSA wrap of random AES key+two AES-GCM frames), POST as multipart, expect 200 + both lines back.
2. **Bad magic:** Post `b"XXXX" + ...` → 400.
3. **Truncated frame:** Post a file cut mid-frame → 400.
4. **Size limit:** Post 21 MB dummy → 400.
5. **Non-admin:** Regular user session → 403.
6. **Truncation flag:** Post a file with > 50k lines (skip or use a small `MAX_LINES` override via monkeypatch) → `truncated: true`, `len(lines) == MAX_LINES`.

## Frontend

### New view — `cloud/frontend/src/views/ClientLogs.vue`

Layout:
```
+--------------------------------------------------+
|  Card header: "客户端日志解密"                   |
|                                                  |
|  +--- el-upload drag-zone ---+                   |
|  | drop .log.enc here or click|                   |
|  +-----------------------------+                   |
|                                                  |
|  ── after upload ──                              |
|  Filename: at-client-...log.enc | 3421 lines     |
|  [el-alert warning] "Truncated at 50000 lines"   |
|  [ search input ]  [ Download .txt btn ]         |
|  +-----------------------------+                 |
|  |  1 | 2026-04-24 10:00 INFO ...  |             |
|  |  2 | ...                        |             |
|  +-----------------------------+                 |
+--------------------------------------------------+
```

- `<el-upload :auto-upload="false" :on-change="handleFile" accept=".enc">` (a plain file button works too; drag-drop is optional polish if `el-upload` supports it trivially).
- Uploads via a new `decryptClientLog(file)` in `api/admin.ts` using `FormData`.
- Store response in local refs; render lines in a plain scrollable `<pre>` with `max-height` + `overflow: auto` (50k lines * ~100 chars ≈ 5 MB string; browsers handle that, no virtual scroll needed).
- Search: `computed(() => lines.value.filter(l => l.toLowerCase().includes(query.value.toLowerCase())))`.
- Download: `new Blob([lines.join('\n')], {type: 'text/plain'})` → `URL.createObjectURL` → anchor click.

### Router

`cloud/frontend/src/router/index.ts`: add `{ path: '/client-logs', component: () => import('../views/ClientLogs.vue'), meta: { admin: true } }`. Match the admin-guard pattern used by existing admin routes.

### Sidebar nav — `cloud/frontend/src/layouts/AdminLayout.vue`

Add a menu item with an icon (reuse `Document` or similar) matching the style of existing items.

### API binding — `cloud/frontend/src/api/admin.ts`

```ts
export interface DecryptedLogResponse {
  filename: string
  total_lines: number
  truncated: boolean
  lines: string[]
}

export function decryptClientLog(file: File): Promise<{ data: DecryptedLogResponse }> {
  const form = new FormData()
  form.append('file', file)
  return request.post('/api/admin/logs/decrypt', form, {
    headers: { 'Content-Type': 'multipart/form-data' },
  })
}
```

### i18n strings — `cloud/frontend/src/locales/{en,zh}.json`

Namespace `clientLogs`:
- `title` — "Client Logs" / "客户端日志"
- `menu` — nav label
- `upload.hint` — "Drag a .log.enc file here or click to upload"
- `upload.accept` — ".log.enc files only"
- `summary.filename` / `summary.lines`
- `truncatedWarning` — "Truncated at {n} lines"
- `searchPlaceholder` — "Filter lines…"
- `download` — "Download as .txt"
- `errors.badMagic` / `errors.tooLarge` / `errors.corrupted` / `errors.unauthorized`

## Error handling

| Layer | Condition | Behavior |
|---|---|---|
| Backend | File > 20 MB | 400 `{detail: "..."}` |
| Backend | Bad magic / unknown version | 400 |
| Backend | Truncated / decrypt failure | 400 |
| Backend | Non-admin | 403 (existing auth) |
| Frontend | Backend 400 | Element Plus error message, keep drop zone visible |
| Frontend | Network failure | Generic error message |

## Security considerations

- Admin auth reused; no new credential surface.
- In-memory only on backend; no temp file.
- Size cap prevents RAM exhaustion.
- Plaintext only lives in the response body and browser memory; not logged, not persisted.
- RSA private key never leaves the cloud container.

## Test plan (manual, after unit tests pass)

1. Build a real client run locally → grab its `.log.enc` → upload to staging web UI → lines match `tools/decrypt_log.py`.
2. Upload a non-encrypted random file → clean error, no crash.
3. Log in as a non-admin user → page/API is blocked.
4. Download button produces a `.txt` identical to the displayed lines.
5. EN/CN toggle renders both languages correctly.

## Files touched

- **New:** `cloud/backend/app/log_decrypt.py`
- **New:** `cloud/backend/tests/test_log_decrypt.py`
- **New:** `cloud/frontend/src/views/ClientLogs.vue`
- **Edit:** `cloud/backend/app/api/admin.py` (+ endpoint)
- **Edit:** `cloud/backend/app/schemas.py` (+ response model)
- **Edit:** `cloud/backend/tools/decrypt_log.py` (refactor to import helper)
- **Edit:** `cloud/frontend/src/api/admin.ts` (+ API fn)
- **Edit:** `cloud/frontend/src/router/index.ts` (+ route)
- **Edit:** `cloud/frontend/src/layouts/AdminLayout.vue` (+ nav item)
- **Edit:** `cloud/frontend/src/locales/en.json`, `zh.json` (+ strings)
