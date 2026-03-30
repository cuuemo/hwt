"""Tests for /api/admin endpoints (users, bindings, logs)."""
import os
from datetime import datetime, timezone, timedelta

from tests.conftest import (
    rsa_encrypt_b64,
    admin_login,
    create_approved_user,
    do_verify,
)

MACHINE = "c" * 64


def test_admin_approve(client):
    """PATCH /api/admin/users/{id} to approve and set license."""
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"newpass")
    reg = client.post("/api/auth/register", json={
        "username": "toapprove",
        "password_encrypted": enc_pw,
    })
    assert reg.status_code == 200
    user_id = reg.json()["id"]

    token = admin_login(client)
    expire = (datetime.now(timezone.utc) + timedelta(days=365)).isoformat()
    resp = client.patch(
        f"/api/admin/users/{user_id}",
        json={
            "status": "active",
            "license_type": "yearly",
            "license_expire_at": expire,
        },
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200
    data = resp.json()
    assert data["status"] == "active"
    assert data["license_type"] == "yearly"


def test_admin_unbind(client):
    """DELETE /api/admin/bindings/{id} returns success message with unbind text."""
    user_id = create_approved_user(client, "unbinduser", "unbindpass")

    # Perform verify to create a binding
    pem = client.get("/api/auth/public-key").json()["public_key"]
    session_key = os.urandom(32)
    enc_key = rsa_encrypt_b64(pem, session_key)
    hs = client.post("/api/verify/handshake", json={"encrypted_session_key": enc_key})
    session_id = hs.json()["session_id"]
    do_verify(client, session_id, session_key, "unbinduser", "unbindpass", MACHINE)

    # Find the binding
    token = admin_login(client)
    bindings_resp = client.get(
        "/api/admin/bindings",
        params={"user_id": user_id},
        headers={"Authorization": f"Bearer {token}"},
    )
    binding_id = bindings_resp.json()["items"][0]["id"]

    # Unbind
    del_resp = client.delete(
        f"/api/admin/bindings/{binding_id}",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert del_resp.status_code == 200
    assert "解绑成功" in del_resp.json()["message"]


def test_admin_logs(client):
    """After register + login there should be auth logs visible to admin."""
    create_approved_user(client, "loguser", "logpass")

    # Login to generate a login log
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"logpass")
    client.post("/api/auth/login", json={
        "username": "loguser",
        "password_encrypted": enc_pw,
    })

    token = admin_login(client)
    resp = client.get(
        "/api/admin/logs",
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200
    data = resp.json()
    assert data["total"] > 0
    actions = {item["action"] for item in data["items"]}
    assert "register" in actions
    assert "login" in actions


def test_admin_list_users(client):
    """GET /api/admin/users with pagination returns users."""
    create_approved_user(client, "listuser1", "lp1")
    create_approved_user(client, "listuser2", "lp2")

    token = admin_login(client)
    resp = client.get(
        "/api/admin/users",
        params={"page": 1, "size": 10},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200
    data = resp.json()
    # admin + listuser1 + listuser2 = at least 3
    assert data["total"] >= 3
    usernames = [u["username"] for u in data["items"]]
    assert "listuser1" in usernames
    assert "listuser2" in usernames


def test_admin_list_bindings(client):
    """GET /api/admin/bindings filtered by user_id returns correct bindings."""
    user_id = create_approved_user(client, "blistuser", "blp")

    # Create binding through verify
    pem = client.get("/api/auth/public-key").json()["public_key"]
    session_key = os.urandom(32)
    enc_key = rsa_encrypt_b64(pem, session_key)
    hs = client.post("/api/verify/handshake", json={"encrypted_session_key": enc_key})
    session_id = hs.json()["session_id"]
    do_verify(client, session_id, session_key, "blistuser", "blp", MACHINE)

    token = admin_login(client)
    resp = client.get(
        "/api/admin/bindings",
        params={"user_id": user_id},
        headers={"Authorization": f"Bearer {token}"},
    )
    assert resp.status_code == 200
    data = resp.json()
    assert data["total"] >= 1
    assert all(b["user_id"] == user_id for b in data["items"])
