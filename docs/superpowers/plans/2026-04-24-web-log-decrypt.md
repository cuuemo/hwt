# Web-based Encrypted Log Decryption — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let admins decrypt `.log.enc` runtime-log files through the cloud admin web UI instead of running `tools/decrypt_log.py`.

**Architecture:** New FastAPI admin endpoint `POST /api/admin/logs/decrypt` accepts a multipart file, decrypts it in-memory using the cloud RSA private key (already held by `CryptoManager`), and returns plaintext lines as JSON. A new Vue view `ClientLogs.vue` uploads the file, renders lines, supports search + `.txt` download. Plaintext is never persisted.

**Tech Stack:** Python / FastAPI / cryptography (backend), Vue 3 / TypeScript / Element Plus / axios / vue-i18n (frontend).

Spec: `docs/superpowers/specs/2026-04-24-web-log-decrypt-design.md`.

---

## File Structure

**Create:**
- `cloud/backend/app/log_decrypt.py` — shared pure-function decryptor
- `cloud/backend/tests/test_log_decrypt.py` — endpoint + helper tests
- `cloud/frontend/src/views/ClientLogs.vue` — upload + view UI

**Modify:**
- `cloud/backend/app/schemas.py` — add `LogDecryptResponse`
- `cloud/backend/app/api/admin.py` — add endpoint + import crypto
- `cloud/backend/tools/decrypt_log.py` — refactor to call the shared helper
- `cloud/frontend/src/api/admin.ts` — add `decryptClientLog()` API
- `cloud/frontend/src/router/index.ts` — add `/client-logs` route
- `cloud/frontend/src/layouts/AdminLayout.vue` — add sidebar menu item
- `cloud/frontend/src/locales/en.json` — add `clientLogs` namespace
- `cloud/frontend/src/locales/zh.json` — add `clientLogs` namespace

---

## Task 1: Backend helper — happy path (roundtrip)

**Files:**
- Create: `cloud/backend/app/log_decrypt.py`
- Test: `cloud/backend/tests/test_log_decrypt.py`

- [ ] **Step 1: Write the failing test**

Create `cloud/backend/tests/test_log_decrypt.py`:

```python
"""Tests for app.log_decrypt + /api/admin/logs/decrypt."""
import os
import struct

from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

from tests.conftest import crypto  # CryptoManager singleton bound to test keys


MAGIC = b"ATLG"
VERSION = 1


def _build_log_bytes(lines):
    """Build an AT .log.enc blob using the test cloud public key."""
    aes_key = os.urandom(32)
    wrapped = crypto.public_key.encrypt(
        aes_key,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None,
        ),
    )
    out = bytearray()
    out += MAGIC
    out += struct.pack(">H", VERSION)
    out += struct.pack(">H", len(wrapped))
    out += wrapped
    aes = AESGCM(aes_key)
    for line in lines:
        nonce = os.urandom(12)
        ct = aes.encrypt(nonce, line.encode("utf-8"), None)  # ct || tag (16B)
        frame = nonce + ct
        out += struct.pack(">I", len(frame))
        out += frame
    return bytes(out)


def test_decrypt_log_bytes_roundtrip():
    from app.log_decrypt import decrypt_log_bytes

    data = _build_log_bytes(["hello world", "second line 中文"])
    lines = list(decrypt_log_bytes(data, crypto.private_key))
    assert lines == ["hello world", "second line 中文"]
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py::test_decrypt_log_bytes_roundtrip -v`
Expected: FAIL with `ModuleNotFoundError: No module named 'app.log_decrypt'`.

- [ ] **Step 3: Implement `app/log_decrypt.py`**

Create `cloud/backend/app/log_decrypt.py`:

```python
"""Decrypt .log.enc envelope-encrypted runtime log files.

File format (matches protocol/src/encrypted_log.rs):
    [4B magic "ATLG"][2B version][2B rsa_key_len][rsa_key_len bytes wrapped AES-256 key]
    repeated: [4B frame_len][frame_len bytes: 12B nonce + ciphertext + 16B tag]
"""
import struct
from typing import Iterator

from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

MAGIC = b"ATLG"
VERSION = 1


class LogDecryptError(ValueError):
    """Raised when a .log.enc file is malformed or cannot be decrypted."""


def decrypt_log_bytes(data: bytes, private_key) -> Iterator[str]:
    """Yield UTF-8 decoded plaintext lines from a .log.enc blob.

    Raises LogDecryptError on bad magic, unsupported version, truncated input,
    or failed AES-GCM authentication.
    """
    if len(data) < 8:
        raise LogDecryptError("file too short")
    if data[:4] != MAGIC:
        raise LogDecryptError(f"bad magic: expected {MAGIC!r}, got {data[:4]!r}")
    (version,) = struct.unpack(">H", data[4:6])
    if version != VERSION:
        raise LogDecryptError(f"unsupported version: {version}")
    (key_len,) = struct.unpack(">H", data[6:8])
    if len(data) < 8 + key_len:
        raise LogDecryptError("truncated RSA-wrapped key")
    wrapped = data[8 : 8 + key_len]

    try:
        aes_key = private_key.decrypt(
            wrapped,
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA256()),
                algorithm=hashes.SHA256(),
                label=None,
            ),
        )
    except Exception as exc:
        raise LogDecryptError(f"failed to unwrap AES key: {exc}") from exc

    aes = AESGCM(aes_key)
    pos = 8 + key_len
    while pos < len(data):
        if pos + 4 > len(data):
            raise LogDecryptError("truncated frame length")
        (flen,) = struct.unpack(">I", data[pos : pos + 4])
        pos += 4
        if pos + flen > len(data):
            raise LogDecryptError("truncated frame body")
        frame = data[pos : pos + flen]
        pos += flen
        if len(frame) < 12 + 16:
            raise LogDecryptError("frame shorter than nonce+tag")
        nonce = frame[:12]
        ct_and_tag = frame[12:]
        try:
            pt = aes.decrypt(nonce, ct_and_tag, None)
        except Exception as exc:
            raise LogDecryptError(f"AES-GCM decrypt failed: {exc}") from exc
        yield pt.decode("utf-8", errors="replace")
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py::test_decrypt_log_bytes_roundtrip -v`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add cloud/backend/app/log_decrypt.py cloud/backend/tests/test_log_decrypt.py
git commit -m "feat(backend): add shared .log.enc decrypt helper"
```

---

## Task 2: Backend helper — error cases

**Files:**
- Test: `cloud/backend/tests/test_log_decrypt.py`

- [ ] **Step 1: Add failing tests**

Append to `cloud/backend/tests/test_log_decrypt.py`:

```python
import pytest


def test_decrypt_log_bytes_bad_magic():
    from app.log_decrypt import decrypt_log_bytes, LogDecryptError

    with pytest.raises(LogDecryptError, match="bad magic"):
        list(decrypt_log_bytes(b"XXXX" + b"\x00" * 100, crypto.private_key))


def test_decrypt_log_bytes_unsupported_version():
    from app.log_decrypt import decrypt_log_bytes, LogDecryptError

    blob = MAGIC + struct.pack(">H", 99) + struct.pack(">H", 0)
    with pytest.raises(LogDecryptError, match="unsupported version"):
        list(decrypt_log_bytes(blob, crypto.private_key))


def test_decrypt_log_bytes_truncated_frame():
    from app.log_decrypt import decrypt_log_bytes, LogDecryptError

    # Valid header + claim 999 byte frame but provide nothing
    data = _build_log_bytes(["ok"])
    # Append a frame length that overruns the buffer
    data += struct.pack(">I", 999)
    with pytest.raises(LogDecryptError, match="truncated frame body"):
        list(decrypt_log_bytes(data, crypto.private_key))


def test_decrypt_log_bytes_too_short():
    from app.log_decrypt import decrypt_log_bytes, LogDecryptError

    with pytest.raises(LogDecryptError, match="file too short"):
        list(decrypt_log_bytes(b"A", crypto.private_key))
```

- [ ] **Step 2: Run tests**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py -v`
Expected: All four new tests PASS (the implementation from Task 1 already handles them).

- [ ] **Step 3: Commit**

```bash
git add cloud/backend/tests/test_log_decrypt.py
git commit -m "test(backend): cover .log.enc error paths"
```

---

## Task 3: Refactor `tools/decrypt_log.py` to use the shared helper

**Files:**
- Modify: `cloud/backend/tools/decrypt_log.py`

- [ ] **Step 1: Rewrite the script**

Replace the entire contents of `cloud/backend/tools/decrypt_log.py` with:

```python
#!/usr/bin/env python3
"""Decrypt an AT encrypted log file (.log.enc).

Usage:
    python decrypt_log.py <log.enc> [--key rsa_private.pem] [--out out.txt]
"""
import argparse
import sys
from pathlib import Path

# Make `app` importable when running the script directly
_BACKEND_DIR = Path(__file__).resolve().parents[1]
if str(_BACKEND_DIR) not in sys.path:
    sys.path.insert(0, str(_BACKEND_DIR))

from cryptography.hazmat.primitives import serialization

from app.log_decrypt import decrypt_log_bytes, LogDecryptError


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("log", type=Path, help="encrypted log file (.log.enc)")
    ap.add_argument(
        "--key",
        type=Path,
        default=_BACKEND_DIR / "keys" / "rsa_private.pem",
        help="RSA private key PEM (default: ../keys/rsa_private.pem)",
    )
    ap.add_argument("--out", type=Path, help="output file (default: stdout)")
    args = ap.parse_args()

    priv = serialization.load_pem_private_key(args.key.read_bytes(), password=None)
    data = args.log.read_bytes()

    out = open(args.out, "w", encoding="utf-8") if args.out else sys.stdout
    try:
        try:
            for line in decrypt_log_bytes(data, priv):
                out.write(line + "\n")
        except LogDecryptError as exc:
            raise SystemExit(f"decrypt failed: {exc}")
    finally:
        if args.out:
            out.close()


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: Verify the script still imports**

Run: `cd cloud/backend && python -c "import tools.decrypt_log"`
Expected: No output, exit 0.

- [ ] **Step 3: Quick smoke test via pytest**

The existing unit tests exercise `decrypt_log_bytes`, so the helper is covered. Manually confirm the CLI help works:

Run: `cd cloud/backend && python tools/decrypt_log.py --help`
Expected: Usage printed, exit 0.

- [ ] **Step 4: Commit**

```bash
git add cloud/backend/tools/decrypt_log.py
git commit -m "refactor(backend): reuse app.log_decrypt in decrypt_log.py"
```

---

## Task 4: Add `LogDecryptResponse` schema

**Files:**
- Modify: `cloud/backend/app/schemas.py`

- [ ] **Step 1: Add the schema**

Append to `cloud/backend/app/schemas.py` (after `PaginatedResponse`):

```python
class LogDecryptResponse(BaseModel):
    filename: str
    total_lines: int
    truncated: bool
    lines: List[str]
```

- [ ] **Step 2: Verify it imports**

Run: `cd cloud/backend && python -c "from app.schemas import LogDecryptResponse; print(LogDecryptResponse.model_fields)"`
Expected: Prints the four field definitions, no error.

- [ ] **Step 3: Commit**

```bash
git add cloud/backend/app/schemas.py
git commit -m "feat(backend): add LogDecryptResponse schema"
```

---

## Task 5: Endpoint — happy path

**Files:**
- Modify: `cloud/backend/app/api/admin.py`
- Test: `cloud/backend/tests/test_log_decrypt.py`

- [ ] **Step 1: Add the failing endpoint test**

Append to `cloud/backend/tests/test_log_decrypt.py`:

```python
from tests.conftest import admin_login


def test_endpoint_roundtrip(client):
    token = admin_login(client)
    blob = _build_log_bytes(["alpha", "beta 中文"])
    resp = client.post(
        "/api/admin/logs/decrypt",
        files={"file": ("demo.log.enc", blob, "application/octet-stream")},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200, resp.text
    data = resp.json()
    assert data["filename"] == "demo.log.enc"
    assert data["total_lines"] == 2
    assert data["truncated"] is False
    assert data["lines"] == ["alpha", "beta 中文"]
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py::test_endpoint_roundtrip -v`
Expected: FAIL with 404 (endpoint does not exist).

- [ ] **Step 3: Implement the endpoint**

Edit `cloud/backend/app/api/admin.py`. Add these imports near the top (merge with existing `from fastapi import ...`):

```python
from fastapi import APIRouter, Depends, File, HTTPException, Query, UploadFile, status
```

Add these module-level imports (next to the other `app.*` imports):

```python
from app.log_decrypt import decrypt_log_bytes, LogDecryptError
from app.schemas import LogDecryptResponse
```

Add these constants near the top of the file (below the existing imports):

```python
MAX_LOG_BYTES = 20 * 1024 * 1024  # 20 MB
MAX_LOG_LINES = 50_000
```

Add the endpoint at the bottom of the file:

```python
# ---------- 客户端日志解密 ----------

@router.post("/logs/decrypt", response_model=LogDecryptResponse)
async def decrypt_client_log(
    file: UploadFile = File(...),
    _admin: User = Depends(require_admin),
):
    """Decrypt an uploaded .log.enc file using the cloud RSA private key."""
    from app.main import crypto

    data = await file.read()
    if len(data) > MAX_LOG_BYTES:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"文件过大 (>{MAX_LOG_BYTES // (1024 * 1024)} MB)",
        )

    lines: list[str] = []
    truncated = False
    try:
        for line in decrypt_log_bytes(data, crypto.private_key):
            if len(lines) >= MAX_LOG_LINES:
                truncated = True
                break
            lines.append(line)
    except LogDecryptError as exc:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(exc),
        )

    return LogDecryptResponse(
        filename=file.filename or "uploaded.log.enc",
        total_lines=len(lines),
        truncated=truncated,
        lines=lines,
    )
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py::test_endpoint_roundtrip -v`
Expected: PASS.

- [ ] **Step 5: Run full decrypt test file**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py -v`
Expected: All tests PASS.

- [ ] **Step 6: Commit**

```bash
git add cloud/backend/app/api/admin.py cloud/backend/tests/test_log_decrypt.py
git commit -m "feat(backend): add /api/admin/logs/decrypt endpoint"
```

---

## Task 6: Endpoint — error and auth cases

**Files:**
- Modify: `cloud/backend/tests/test_log_decrypt.py`
- Modify: `cloud/backend/app/api/admin.py` (only if needed)

- [ ] **Step 1: Add failing tests**

Append to `cloud/backend/tests/test_log_decrypt.py`:

```python
from tests.conftest import create_approved_user, rsa_encrypt_b64


def _login_as(client, username, password):
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, password.encode("utf-8"))
    resp = client.post("/api/auth/login", json={
        "username": username,
        "password_encrypted": enc_pw,
    })
    assert resp.status_code == 200, resp.text
    return resp.json()["access_token"]


def test_endpoint_bad_magic_returns_400(client):
    token = admin_login(client)
    resp = client.post(
        "/api/admin/logs/decrypt",
        files={"file": ("junk.log.enc", b"XXXX" + b"\x00" * 100,
                        "application/octet-stream")},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 400
    assert "bad magic" in resp.json()["detail"]


def test_endpoint_size_limit_returns_400(client):
    token = admin_login(client)
    big = b"A" * (20 * 1024 * 1024 + 1)
    resp = client.post(
        "/api/admin/logs/decrypt",
        files={"file": ("big.log.enc", big, "application/octet-stream")},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 400
    assert "过大" in resp.json()["detail"]


def test_endpoint_requires_admin(client):
    create_approved_user(client, "loguser1", "logpass1")
    token = _login_as(client, "loguser1", "logpass1")
    blob = _build_log_bytes(["hi"])
    resp = client.post(
        "/api/admin/logs/decrypt",
        files={"file": ("x.log.enc", blob, "application/octet-stream")},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 403


def test_endpoint_truncation_flag(client, monkeypatch):
    import app.api.admin as admin_mod
    monkeypatch.setattr(admin_mod, "MAX_LOG_LINES", 3)

    token = admin_login(client)
    blob = _build_log_bytes([f"line {i}" for i in range(10)])
    resp = client.post(
        "/api/admin/logs/decrypt",
        files={"file": ("many.log.enc", blob, "application/octet-stream")},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200
    data = resp.json()
    assert data["truncated"] is True
    assert data["total_lines"] == 3
    assert data["lines"] == ["line 0", "line 1", "line 2"]
```

- [ ] **Step 2: Run tests**

Run: `cd cloud/backend && pytest tests/test_log_decrypt.py -v`
Expected: All tests PASS. If `test_endpoint_requires_admin` fails with 401 instead of 403, check `require_admin` — a non-admin logged-in user should get 403.

- [ ] **Step 3: Commit**

```bash
git add cloud/backend/tests/test_log_decrypt.py
git commit -m "test(backend): cover decrypt endpoint error and auth paths"
```

---

## Task 7: Frontend — API binding

**Files:**
- Modify: `cloud/frontend/src/api/admin.ts`

- [ ] **Step 1: Append the new interface and function**

Append to `cloud/frontend/src/api/admin.ts`:

```ts
export interface DecryptedLogResponse {
  filename: string
  total_lines: number
  truncated: boolean
  lines: string[]
}

export function decryptClientLog(
  file: File,
): Promise<{ data: DecryptedLogResponse }> {
  const form = new FormData()
  form.append('file', file)
  return request.post('/api/admin/logs/decrypt', form, {
    headers: { 'Content-Type': 'multipart/form-data' },
    timeout: 60_000,
  })
}
```

- [ ] **Step 2: Type-check**

Run: `cd cloud/frontend && npm run build`
Expected: Build succeeds (the function is currently unused; TypeScript does not error on exported-but-unused functions).

- [ ] **Step 3: Commit**

```bash
git add cloud/frontend/src/api/admin.ts
git commit -m "feat(frontend): add decryptClientLog API binding"
```

---

## Task 8: Frontend — ClientLogs view

**Files:**
- Create: `cloud/frontend/src/views/ClientLogs.vue`

- [ ] **Step 1: Create the component**

Create `cloud/frontend/src/views/ClientLogs.vue`:

```vue
<template>
  <div class="page-container">
    <el-card shadow="never" class="table-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ $t('clientLogs.title') }}</span>
        </div>
      </template>

      <!-- Upload zone -->
      <el-upload
        drag
        :auto-upload="false"
        :show-file-list="false"
        :on-change="handleFileChange"
        accept=".enc"
        class="upload-zone"
      >
        <el-icon class="el-icon--upload"><UploadFilled /></el-icon>
        <div class="el-upload__text">
          {{ $t('clientLogs.upload.hint') }}
        </div>
        <template #tip>
          <div class="el-upload__tip">{{ $t('clientLogs.upload.accept') }}</div>
        </template>
      </el-upload>

      <el-skeleton v-if="loading" :rows="5" animated class="loading-skeleton" />

      <!-- Result pane -->
      <div v-if="result && !loading" class="result-pane">
        <div class="summary">
          <span class="filename">{{ result.filename }}</span>
          <el-tag size="small" type="info">
            {{ $t('clientLogs.summary.lines', { n: result.total_lines }) }}
          </el-tag>
          <el-tag v-if="result.truncated" size="small" type="warning">
            {{ $t('clientLogs.truncatedWarning', { n: result.total_lines }) }}
          </el-tag>
          <div class="spacer" />
          <el-input
            v-model="query"
            :placeholder="$t('clientLogs.searchPlaceholder')"
            clearable
            class="search-input"
          >
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
          <el-button type="primary" @click="downloadTxt">
            <el-icon><Download /></el-icon>
            <span>{{ $t('clientLogs.download') }}</span>
          </el-button>
        </div>

        <pre class="log-view">{{ filteredText }}</pre>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { UploadFile } from 'element-plus'
import { ElMessage } from 'element-plus'
import { UploadFilled, Search, Download } from '@element-plus/icons-vue'
import { decryptClientLog, type DecryptedLogResponse } from '../api/admin'

const loading = ref(false)
const result = ref<DecryptedLogResponse | null>(null)
const query = ref('')

async function handleFileChange(uploadFile: UploadFile) {
  if (!uploadFile.raw) return
  loading.value = true
  result.value = null
  query.value = ''
  try {
    const resp = await decryptClientLog(uploadFile.raw)
    result.value = resp.data
  } catch (err) {
    // request.ts interceptor already shows an error toast
    console.error('decrypt failed:', err)
  } finally {
    loading.value = false
  }
}

const filteredLines = computed(() => {
  if (!result.value) return []
  const q = query.value.trim().toLowerCase()
  if (!q) return result.value.lines
  return result.value.lines.filter((l) => l.toLowerCase().includes(q))
})

const filteredText = computed(() =>
  filteredLines.value
    .map((l, i) => String(i + 1).padStart(5, ' ') + '  ' + l)
    .join('\n'),
)

function downloadTxt() {
  if (!result.value) return
  const blob = new Blob([result.value.lines.join('\n')], {
    type: 'text/plain;charset=utf-8',
  })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = result.value.filename.replace(/\.enc$/, '') + '.txt'
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  ElMessage.success('OK')
}
</script>

<style scoped>
.page-container { padding: 0; }
.table-card {
  border: none;
  background: var(--at-bg-card);
}
.card-header .title {
  font-size: 18px;
  font-weight: 700;
  color: var(--at-text-primary);
}
.upload-zone { margin-top: 8px; }
.loading-skeleton { margin-top: 16px; }
.result-pane { margin-top: 20px; }
.summary {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}
.filename {
  font-family: monospace;
  font-weight: 600;
  color: var(--at-text-primary);
}
.spacer { flex: 1; }
.search-input { width: 240px; }
.log-view {
  background: var(--at-bg-main, #0f1420);
  color: var(--at-text-primary, #e6e8f0);
  border: 1px solid var(--at-border, #2a3142);
  border-radius: 6px;
  padding: 12px 16px;
  max-height: 60vh;
  overflow: auto;
  font-family: Menlo, Consolas, monospace;
  font-size: 12px;
  line-height: 1.55;
  white-space: pre;
  margin: 0;
}
@media (max-width: 768px) {
  .search-input { width: 100%; }
}
</style>
```

- [ ] **Step 2: Type-check build**

Run: `cd cloud/frontend && npm run build`
Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add cloud/frontend/src/views/ClientLogs.vue
git commit -m "feat(frontend): add ClientLogs view with upload + search"
```

---

## Task 9: Frontend — router route

**Files:**
- Modify: `cloud/frontend/src/router/index.ts`

- [ ] **Step 1: Add the route**

Edit `cloud/frontend/src/router/index.ts`. In the `children` array, after the `logs` child, add:

```ts
      {
        path: 'client-logs',
        name: 'ClientLogs',
        component: () => import('../views/ClientLogs.vue'),
      },
```

The surrounding block after edit:

```ts
      {
        path: 'logs',
        name: 'Logs',
        component: () => import('../views/Logs.vue'),
      },
      {
        path: 'client-logs',
        name: 'ClientLogs',
        component: () => import('../views/ClientLogs.vue'),
      },
    ],
```

- [ ] **Step 2: Type-check build**

Run: `cd cloud/frontend && npm run build`
Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add cloud/frontend/src/router/index.ts
git commit -m "feat(frontend): register /client-logs route"
```

---

## Task 10: Frontend — sidebar menu item

**Files:**
- Modify: `cloud/frontend/src/layouts/AdminLayout.vue`

- [ ] **Step 1: Add the menu item**

Edit `cloud/frontend/src/layouts/AdminLayout.vue`. After the existing `/logs` `<el-menu-item>` block, insert:

```vue
        <el-menu-item index="/client-logs">
          <el-icon><Files /></el-icon>
          <template #title>{{ $t('common.clientLogs') }}</template>
        </el-menu-item>
```

Then update the icon import on line 90 to include `Files`:

```ts
import { Odometer, User, Link, Document, Files, SwitchButton, Fold, Expand, Monitor, Menu } from '@element-plus/icons-vue'
```

- [ ] **Step 2: Type-check build**

Run: `cd cloud/frontend && npm run build`
Expected: Build succeeds.

- [ ] **Step 3: Commit**

```bash
git add cloud/frontend/src/layouts/AdminLayout.vue
git commit -m "feat(frontend): add Client Logs sidebar entry"
```

---

## Task 11: Frontend — i18n strings

**Files:**
- Modify: `cloud/frontend/src/locales/en.json`
- Modify: `cloud/frontend/src/locales/zh.json`

- [ ] **Step 1: Add English strings**

Edit `cloud/frontend/src/locales/en.json`.

In the `common` object, add a `clientLogs` key (place it after `"logs": "Logs",`):

```json
    "clientLogs": "Client Logs",
```

At the top level, after the `logs` object's closing `}`, insert a new top-level `clientLogs` block (before `"auth":`):

```json
  "clientLogs": {
    "title": "Client Log Decrypt",
    "upload": {
      "hint": "Drop a .log.enc file here, or click to upload",
      "accept": "Only .log.enc files produced by AT client/server are supported"
    },
    "summary": {
      "lines": "{n} lines"
    },
    "truncatedWarning": "Truncated at {n} lines — the file was larger",
    "searchPlaceholder": "Filter lines…",
    "download": "Download as .txt"
  },
```

- [ ] **Step 2: Add Chinese strings**

Edit `cloud/frontend/src/locales/zh.json`.

In the `common` object, add after `"logs": "验证日志",`:

```json
    "clientLogs": "客户端日志",
```

At the top level, before `"auth":`, insert:

```json
  "clientLogs": {
    "title": "客户端日志解密",
    "upload": {
      "hint": "拖拽 .log.enc 文件到这里，或点击上传",
      "accept": "仅支持 AT 客户端/服务端产生的 .log.enc 文件"
    },
    "summary": {
      "lines": "共 {n} 行"
    },
    "truncatedWarning": "已截断到 {n} 行 — 原文件更大",
    "searchPlaceholder": "筛选日志…",
    "download": "下载为 .txt"
  },
```

- [ ] **Step 3: Validate JSON**

Run: `cd cloud/frontend && python -c "import json; json.load(open('src/locales/en.json')); json.load(open('src/locales/zh.json')); print('ok')"`
Expected: `ok`.

- [ ] **Step 4: Type-check build**

Run: `cd cloud/frontend && npm run build`
Expected: Build succeeds.

- [ ] **Step 5: Commit**

```bash
git add cloud/frontend/src/locales/en.json cloud/frontend/src/locales/zh.json
git commit -m "feat(frontend): i18n strings for client log decrypt"
```

---

## Task 12: End-to-end verification

**Files:** None modified. Manual verification only.

- [ ] **Step 1: Start the backend**

Run: `cd cloud/backend && bash start.sh --reload` (port 10000)

- [ ] **Step 2: Start the frontend dev server in another terminal**

Run: `cd cloud/frontend && npm run dev` (default port 5173)

- [ ] **Step 3: Produce a real `.log.enc` fixture**

On a dev box with the Rust workspace built (or using an existing client/server `.log.enc` from `dist/` / a test run), copy one file onto your local machine. Alternatively, generate a synthetic one with the same Python helper the tests use — save it as `/tmp/demo.log.enc`:

```bash
cd cloud/backend && python - <<'PY'
import os, struct
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

pub = serialization.load_pem_public_key(open("keys/rsa_public.pem","rb").read())
aes_key = os.urandom(32)
wrapped = pub.encrypt(aes_key, padding.OAEP(mgf=padding.MGF1(hashes.SHA256()), algorithm=hashes.SHA256(), label=None))
aes = AESGCM(aes_key)
blob = b"ATLG" + struct.pack(">H", 1) + struct.pack(">H", len(wrapped)) + wrapped
for line in [f"[INFO] line {i}" for i in range(20)]:
    nonce = os.urandom(12)
    frame = nonce + aes.encrypt(nonce, line.encode(), None)
    blob += struct.pack(">I", len(frame)) + frame
open("/tmp/demo.log.enc","wb").write(blob)
print("wrote /tmp/demo.log.enc", len(blob), "bytes")
PY
```

- [ ] **Step 4: Verify the UI**

In a browser:
1. Open http://localhost:5173, log in as admin.
2. Click the new **Client Logs / 客户端日志** menu item.
3. Drag `/tmp/demo.log.enc` onto the drop zone (or click to upload).
4. Confirm all 20 lines appear with line numbers.
5. Type "line 1" in the search box — confirm only matching lines display.
6. Click **Download as .txt** — confirm a `demo.log.txt` file downloads with the plaintext.
7. Toggle the language (top-right) — confirm EN/CN strings both work.

- [ ] **Step 5: Negative path**

1. Create a junk file: `echo "not a log" > /tmp/junk.log.enc`.
2. Upload it — confirm an error toast appears and no lines are displayed.

- [ ] **Step 6: Non-admin path**

1. Register a regular user via the Register page; do not approve.
2. Attempt to access `/client-logs` — the page should still render, but the upload API call should return 403 and the error toast should show.

(If regular users should not see the menu item at all, that is out of scope for v1 since the existing `/logs`, `/users`, `/bindings` pages are similarly visible to all authenticated users in the current codebase.)

- [ ] **Step 7: Stop dev servers — no commit needed**

This task records verification results only.

---

## Closing: verify full test suite

- [ ] **Step 1: Backend test suite**

Run: `cd cloud/backend && pytest`
Expected: All tests pass.

- [ ] **Step 2: Frontend production build**

Run: `cd cloud/frontend && npm run build`
Expected: Build succeeds with no type errors.

- [ ] **Step 3: Final commit (if any uncommitted changes)**

```bash
git status
```

If clean, done. Otherwise commit any stragglers with a descriptive message.
