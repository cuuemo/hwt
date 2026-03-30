"""End-to-end lifecycle test covering the full user journey."""
import os

from tests.conftest import (
    rsa_encrypt_b64,
    admin_login,
    do_verify,
)
from datetime import datetime, timezone, timedelta


MACHINE_A = "1a" * 32  # 64-char hex
MACHINE_B = "2b" * 32  # 64-char hex


def test_full_lifecycle(client):
    """
    register -> approve -> login -> handshake -> verify(bind)
    -> verify(same=ok) -> verify(diff=fail) -> unbind -> verify(new=ok)
    -> check logs
    """
    pem = client.get("/api/auth/public-key").json()["public_key"]

    # 1) Register
    enc_pw = rsa_encrypt_b64(pem, b"lifecycle_pw")
    reg = client.post("/api/auth/register", json={
        "username": "lcuser",
        "password_encrypted": enc_pw,
    })
    assert reg.status_code == 200
    user_id = reg.json()["id"]
    assert reg.json()["status"] == "pending"

    # 2) Admin approves + sets monthly license
    token = admin_login(client)
    expire = (datetime.now(timezone.utc) + timedelta(days=30)).isoformat()
    patch = client.patch(
        f"/api/admin/users/{user_id}",
        json={
            "status": "active",
            "license_type": "monthly",
            "license_expire_at": expire,
        },
        headers={"Authorization": f"Bearer {token}"},
    )
    assert patch.status_code == 200

    # 3) Login
    enc_pw_login = rsa_encrypt_b64(pem, b"lifecycle_pw")
    login_resp = client.post("/api/auth/login", json={
        "username": "lcuser",
        "password_encrypted": enc_pw_login,
    })
    assert login_resp.status_code == 200
    assert "access_token" in login_resp.json()

    # 4) Handshake
    session_key = os.urandom(32)
    enc_key = rsa_encrypt_b64(pem, session_key)
    hs = client.post("/api/verify/handshake", json={"encrypted_session_key": enc_key})
    assert hs.status_code == 200
    session_id = hs.json()["session_id"]

    # 5) Verify (first time -> bind MACHINE_A)
    r1 = do_verify(client, session_id, session_key, "lcuser", "lifecycle_pw", MACHINE_A)
    assert r1["authorized"] is True

    # 6) Verify again from same machine -> ok
    r2 = do_verify(client, session_id, session_key, "lcuser", "lifecycle_pw", MACHINE_A)
    assert r2["authorized"] is True

    # 7) Verify from different machine -> fail
    r3 = do_verify(client, session_id, session_key, "lcuser", "lifecycle_pw", MACHINE_B)
    assert r3["authorized"] is False
    assert "不匹配" in r3["message"]

    # 8) Admin unbinds
    token = admin_login(client)
    bindings = client.get(
        "/api/admin/bindings",
        params={"user_id": user_id},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert bindings.status_code == 200
    active_bindings = [
        b for b in bindings.json()["items"] if b["status"] == "active"
    ]
    assert len(active_bindings) >= 1
    binding_id = active_bindings[0]["id"]

    del_resp = client.delete(
        f"/api/admin/bindings/{binding_id}",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert del_resp.status_code == 200

    # 9) Verify from new machine (MACHINE_B) -> ok (re-binds)
    r4 = do_verify(client, session_id, session_key, "lcuser", "lifecycle_pw", MACHINE_B)
    assert r4["authorized"] is True

    # 10) Check logs
    logs = client.get(
        "/api/admin/logs",
        params={"username": "lcuser"},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert logs.status_code == 200
    log_data = logs.json()
    assert log_data["total"] > 0
    actions = {item["action"] for item in log_data["items"]}
    assert "register" in actions
    assert "verify" in actions
