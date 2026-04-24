"""Tests for app.log_decrypt (decrypt helper unit tests)."""
import os
import struct

from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

from tests.conftest import crypto  # CryptoManager singleton bound to test keys
from app.log_decrypt import MAGIC, VERSION, decrypt_log_bytes


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
    data = _build_log_bytes(["hello world", "second line 中文"])
    lines = list(decrypt_log_bytes(data, crypto.private_key))
    assert lines == ["hello world", "second line 中文"]
