"""Tests for /api/auth endpoints (public-key, register, login)."""
from tests.conftest import rsa_encrypt_b64, create_approved_user


def test_public_key(client):
    """GET /api/auth/public-key returns a valid PEM public key."""
    resp = client.get("/api/auth/public-key")
    assert resp.status_code == 200
    pem = resp.json()["public_key"]
    assert pem.startswith("-----BEGIN PUBLIC KEY-----")
    assert pem.strip().endswith("-----END PUBLIC KEY-----")


def test_register(client):
    """Register a new user; status should be 'pending'."""
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"mypassword")
    resp = client.post("/api/auth/register", json={
        "username": "testuser",
        "password_encrypted": enc_pw,
    })
    assert resp.status_code == 200
    data = resp.json()
    assert data["username"] == "testuser"
    assert data["status"] == "pending"


def test_register_duplicate(client):
    """Registering the same username twice returns 400."""
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"pw1")
    resp1 = client.post("/api/auth/register", json={
        "username": "dupuser",
        "password_encrypted": enc_pw,
    })
    assert resp1.status_code == 200

    enc_pw2 = rsa_encrypt_b64(pem, b"pw2")
    resp2 = client.post("/api/auth/register", json={
        "username": "dupuser",
        "password_encrypted": enc_pw2,
    })
    assert resp2.status_code == 400


def test_login_pending(client):
    """A pending (not yet approved) user cannot log in."""
    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"secret")
    client.post("/api/auth/register", json={
        "username": "pendinguser",
        "password_encrypted": enc_pw,
    })

    # Try to log in
    enc_pw_login = rsa_encrypt_b64(pem, b"secret")
    resp = client.post("/api/auth/login", json={
        "username": "pendinguser",
        "password_encrypted": enc_pw_login,
    })
    assert resp.status_code == 401


def test_login_success(client):
    """An approved user can log in and receive a JWT token."""
    create_approved_user(client, "loginuser", "loginpass")

    pem = client.get("/api/auth/public-key").json()["public_key"]
    enc_pw = rsa_encrypt_b64(pem, b"loginpass")
    resp = client.post("/api/auth/login", json={
        "username": "loginuser",
        "password_encrypted": enc_pw,
    })
    assert resp.status_code == 200
    data = resp.json()
    assert "access_token" in data
    assert data["token_type"] == "bearer"
    assert data["user"]["username"] == "loginuser"
    assert data["user"]["status"] == "active"
