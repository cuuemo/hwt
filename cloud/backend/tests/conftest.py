"""
Shared fixtures and helpers for the HWT cloud backend test suite.
"""
import os
import json
import base64
import tempfile

# --- Set environment BEFORE any app import ---
os.environ["DATABASE_URL"] = "sqlite://"
_tmpdir = tempfile.mkdtemp()
os.environ["RSA_KEY_DIR"] = _tmpdir

import pytest
from datetime import datetime, timezone, timedelta

from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

from sqlalchemy import create_engine, event
from sqlalchemy.orm import sessionmaker
from sqlalchemy.pool import StaticPool
from fastapi.testclient import TestClient

# Import database module to patch engine/SessionLocal BEFORE app import
import app.database as _db
from app.database import Base

# Replace the engine with a StaticPool in-memory SQLite so that all
# connections share the same underlying database.
_test_engine = create_engine(
    "sqlite://",
    connect_args={"check_same_thread": False},
    poolclass=StaticPool,
)
_db.engine = _test_engine
_db.SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=_test_engine)

# Re-export for convenience
engine = _db.engine
SessionLocal = _db.SessionLocal

# Create tables before importing app.main (which also calls create_all and
# registers the startup event that queries the users table).
from app.models import User, UserRole, UserStatus  # noqa: E402
Base.metadata.create_all(bind=engine)

from app.main import app, crypto  # noqa: E402
from app.auth import hash_password  # noqa: E402


# ---------------------------------------------------------------------------
# Crypto helpers
# ---------------------------------------------------------------------------

def rsa_encrypt_b64(pem: str, plaintext: bytes) -> str:
    """RSA OAEP-SHA256 encrypt *plaintext* with the given PEM public key,
    return the ciphertext as a base64-encoded string."""
    public_key = serialization.load_pem_public_key(pem.encode("utf-8"))
    ct = public_key.encrypt(
        plaintext,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None,
        ),
    )
    return base64.b64encode(ct).decode("utf-8")


def aes_encrypt(key: bytes, data: bytes):
    """AES-256-GCM encrypt. Returns (iv_b64, payload_b64, tag_b64)."""
    iv = os.urandom(12)
    aesgcm = AESGCM(key)
    ct_with_tag = aesgcm.encrypt(iv, data, None)
    ct = ct_with_tag[:-16]
    tag = ct_with_tag[-16:]
    return (
        base64.b64encode(iv).decode(),
        base64.b64encode(ct).decode(),
        base64.b64encode(tag).decode(),
    )


def aes_decrypt(key: bytes, iv_b64: str, payload_b64: str, tag_b64: str) -> bytes:
    """AES-256-GCM decrypt."""
    iv = base64.b64decode(iv_b64)
    ct = base64.b64decode(payload_b64)
    tag = base64.b64decode(tag_b64)
    aesgcm = AESGCM(key)
    return aesgcm.decrypt(iv, ct + tag, None)


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

def _seed_admin():
    """Insert the default admin user into a clean database."""
    db = SessionLocal()
    try:
        admin_user = User(
            username="admin",
            password_hash=hash_password("admin123"),
            role=UserRole.admin,
            status=UserStatus.active,
        )
        db.add(admin_user)
        db.commit()
    finally:
        db.close()


@pytest.fixture(scope="session")
def client():
    """Session-scoped TestClient for the FastAPI app."""
    with TestClient(app) as c:
        yield c


@pytest.fixture(autouse=True)
def reset_db(client):
    """Drop + recreate all tables before every test; also clear crypto sessions
    and recreate the default admin user.

    Depends on ``client`` so the session-scoped TestClient is created first
    (its startup event expects the users table to exist)."""
    Base.metadata.drop_all(bind=engine)
    Base.metadata.create_all(bind=engine)

    # Recreate default admin
    _seed_admin()

    # Clear in-memory crypto sessions
    crypto.sessions.clear()

    yield


# ---------------------------------------------------------------------------
# High-level helpers
# ---------------------------------------------------------------------------

def admin_login(client: TestClient) -> str:
    """Log in as the default admin and return the JWT token string."""
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"admin123")
    resp = client.post("/api/auth/login", json={
        "username": "admin",
        "password_encrypted": enc_pw,
    })
    assert resp.status_code == 200, resp.text
    return resp.json()["access_token"]


def do_verify(client: TestClient, session_id: str, session_key: bytes,
              account: str, password: str, machine_code: str) -> dict:
    """Encrypt a verify request, POST it, decrypt the response, return the
    plain-text dict."""
    plain = json.dumps({
        "account": account,
        "password": password,
        "machine_code": machine_code,
    }).encode("utf-8")
    iv_b64, payload_b64, tag_b64 = aes_encrypt(session_key, plain)
    resp = client.post("/api/verify/", json={
        "session_id": session_id,
        "iv": iv_b64,
        "payload": payload_b64,
        "tag": tag_b64,
    })
    assert resp.status_code == 200, resp.text
    data = resp.json()
    decrypted = aes_decrypt(session_key, data["iv"], data["payload"], data["tag"])
    return json.loads(decrypted)


def create_approved_user(client: TestClient, username: str, password: str) -> int:
    """Register a user, approve via admin, set monthly license (30 days).
    Returns the user id."""
    pem = client.get("/api/auth/public-key").json()["public_key"]

    # 1) register
    enc_pw = rsa_encrypt_b64(pem, password.encode("utf-8"))
    reg_resp = client.post("/api/auth/register", json={
        "username": username,
        "password_encrypted": enc_pw,
    })
    assert reg_resp.status_code == 200, reg_resp.text
    user_id = reg_resp.json()["id"]

    # 2) admin approves + sets license
    token = admin_login(client)
    expire = (datetime.now(timezone.utc) + timedelta(days=30)).isoformat()
    patch_resp = client.patch(
        f"/api/admin/users/{user_id}",
        json={
            "status": "active",
            "license_type": "monthly",
            "license_expire_at": expire,
        },
        headers={"Authorization": f"Bearer {token}"},
    )
    assert patch_resp.status_code == 200, patch_resp.text

    return user_id
