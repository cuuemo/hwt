"""
Integration test simulating the Rust server's auth.rs flow:
  1. GET /api/auth/public-key -> PEM
  2. Generate 32-byte AES session key
  3. RSA-OAEP-SHA256 encrypt session key -> POST /api/verify/handshake -> session_id
  4. AES-GCM encrypt {account, password, machine_code} -> POST /api/verify/ -> encrypted response
  5. AES-GCM decrypt response -> check authorized
"""
import os
import json
import base64

from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

from tests.conftest import create_approved_user


MACHINE_CODE = "ab" * 32  # 64-char hex string like SHA256


def test_server_cloud_integration(client):
    """Replicate the exact flow that server/src/auth.rs performs."""

    # --- Step 0: Prepare a valid user ---
    create_approved_user(client, "rustuser", "rustpass")

    # --- Step 1: Fetch public key ---
    pk_resp = client.get("/api/auth/public-key")
    assert pk_resp.status_code == 200
    pem_str = pk_resp.json()["public_key"]
    assert "BEGIN PUBLIC KEY" in pem_str
    public_key = serialization.load_pem_public_key(pem_str.encode("utf-8"))

    # --- Step 2: Generate 32-byte AES session key ---
    session_key = os.urandom(32)

    # --- Step 3: RSA-OAEP-SHA256 encrypt session key and handshake ---
    encrypted_session_key = public_key.encrypt(
        session_key,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None,
        ),
    )
    enc_key_b64 = base64.b64encode(encrypted_session_key).decode("utf-8")

    hs_resp = client.post("/api/verify/handshake", json={
        "encrypted_session_key": enc_key_b64,
    })
    assert hs_resp.status_code == 200
    session_id = hs_resp.json()["session_id"]
    assert session_id

    # --- Step 4: AES-GCM encrypt verify payload ---
    verify_plain = json.dumps({
        "account": "rustuser",
        "password": "rustpass",
        "machine_code": MACHINE_CODE,
    }).encode("utf-8")

    iv = os.urandom(12)
    aesgcm = AESGCM(session_key)
    ct_with_tag = aesgcm.encrypt(iv, verify_plain, None)
    ct = ct_with_tag[:-16]
    tag = ct_with_tag[-16:]

    verify_resp = client.post("/api/verify/", json={
        "session_id": session_id,
        "iv": base64.b64encode(iv).decode(),
        "payload": base64.b64encode(ct).decode(),
        "tag": base64.b64encode(tag).decode(),
    })
    assert verify_resp.status_code == 200

    # --- Step 5: AES-GCM decrypt response ---
    resp_data = verify_resp.json()
    resp_iv = base64.b64decode(resp_data["iv"])
    resp_ct = base64.b64decode(resp_data["payload"])
    resp_tag = base64.b64decode(resp_data["tag"])

    resp_plain = aesgcm.decrypt(resp_iv, resp_ct + resp_tag, None)
    result = json.loads(resp_plain.decode("utf-8"))

    assert result["authorized"] is True
    assert result["license_type"] == "monthly"
    assert result["message"] == "验证成功"
