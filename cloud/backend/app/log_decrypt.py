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
