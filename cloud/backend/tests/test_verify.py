"""Tests for /api/verify endpoints (handshake + AES-encrypted verify)."""
import os
import base64
from datetime import datetime, timezone, timedelta

from tests.conftest import (
    rsa_encrypt_b64,
    aes_decrypt,
    create_approved_user,
    admin_login,
    do_verify,
)
from app.database import SessionLocal
from app.models import User


MACHINE1 = "a" * 64
MACHINE2 = "b" * 64


def _handshake(client):
    """Perform RSA handshake; returns (session_id, session_key)."""
    pem = client.get("/api/auth/public-key").json()["public_key"]
    session_key = os.urandom(32)
    enc_key = rsa_encrypt_b64(pem, session_key)
    resp = client.post("/api/verify/handshake", json={
        "encrypted_session_key": enc_key,
    })
    assert resp.status_code == 200, resp.text
    return resp.json()["session_id"], session_key


def test_handshake(client):
    """POST /api/verify/handshake with RSA-encrypted 32-byte key returns session_id."""
    session_id, _ = _handshake(client)
    assert isinstance(session_id, str)
    assert len(session_id) > 0


def test_verify_success(client):
    """Full handshake + verify flow yields authorized=true."""
    create_approved_user(client, "vuser", "vpass")
    session_id, session_key = _handshake(client)

    result = do_verify(client, session_id, session_key, "vuser", "vpass", MACHINE1)
    assert result["authorized"] is True
    assert result["license_type"] == "monthly"


def test_verify_machine_bind(client):
    """After a successful verify the machine binding should exist in admin API."""
    user_id = create_approved_user(client, "binduser", "bindpass")
    session_id, session_key = _handshake(client)
    result = do_verify(client, session_id, session_key, "binduser", "bindpass", MACHINE1)
    assert result["authorized"] is True

    # Check binding via admin API
    token = admin_login(client)
    resp = client.get(
        "/api/admin/bindings",
        params={"user_id": user_id},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200
    bindings = resp.json()
    assert bindings["total"] >= 1
    codes = [b["machine_code"] for b in bindings["items"]]
    assert MACHINE1 in codes


def test_verify_machine_mismatch(client):
    """First machine binds; a different machine code is rejected."""
    create_approved_user(client, "mmuser", "mmpass")
    session_id, session_key = _handshake(client)

    # Bind with MACHINE1
    r1 = do_verify(client, session_id, session_key, "mmuser", "mmpass", MACHINE1)
    assert r1["authorized"] is True

    # Attempt from MACHINE2 -> rejected
    r2 = do_verify(client, session_id, session_key, "mmuser", "mmpass", MACHINE2)
    assert r2["authorized"] is False
    assert "不匹配" in r2["message"]


def test_verify_expired_license(client):
    """If the license is already expired, authorized=false."""
    user_id = create_approved_user(client, "expuser", "exppass")

    # Set license_expire_at to the past via direct DB update
    db = SessionLocal()
    try:
        user = db.query(User).filter(User.id == user_id).first()
        user.license_expire_at = datetime.now(timezone.utc) - timedelta(days=1)
        db.commit()
    finally:
        db.close()

    session_id, session_key = _handshake(client)
    result = do_verify(client, session_id, session_key, "expuser", "exppass", MACHINE1)
    assert result["authorized"] is False
    assert "过期" in result["message"]
