#!/usr/bin/env python3
"""Decrypt an AT encrypted log file (.log.enc).

File format:
    [4B magic "ATLG"][2B version][2B RSA-key-len][RSA(AES-256 key)]
    repeated: [4B frame-len][12B nonce][ciphertext+16B tag]

Usage:
    python decrypt_log.py <log.enc> [--key rsa_private.pem] [--out out.txt]
"""
import argparse
import struct
import sys
from pathlib import Path

from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

MAGIC = b"ATLG"


def decrypt(enc_path: Path, key_path: Path):
    data = enc_path.read_bytes()
    if data[:4] != MAGIC:
        raise SystemExit(f"not an AT log file: bad magic {data[:4]!r}")
    version = struct.unpack(">H", data[4:6])[0]
    if version != 1:
        raise SystemExit(f"unsupported version: {version}")
    key_len = struct.unpack(">H", data[6:8])[0]
    wrapped = data[8 : 8 + key_len]

    priv = serialization.load_pem_private_key(key_path.read_bytes(), password=None)
    aes_key = priv.decrypt(
        wrapped,
        padding.OAEP(
            mgf=padding.MGF1(algorithm=hashes.SHA256()),
            algorithm=hashes.SHA256(),
            label=None,
        ),
    )
    aes = AESGCM(aes_key)

    pos = 8 + key_len
    while pos < len(data):
        flen = struct.unpack(">I", data[pos : pos + 4])[0]
        pos += 4
        frame = data[pos : pos + flen]
        pos += flen
        nonce = frame[:12]
        ct = frame[12:]
        yield aes.decrypt(nonce, ct, None).decode("utf-8", errors="replace")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("log", type=Path, help="encrypted log file (.log.enc)")
    ap.add_argument(
        "--key",
        type=Path,
        default=Path(__file__).resolve().parents[1] / "keys" / "rsa_private.pem",
        help="RSA private key PEM (default: ../keys/rsa_private.pem)",
    )
    ap.add_argument("--out", type=Path, help="output file (default: stdout)")
    args = ap.parse_args()

    out = open(args.out, "w", encoding="utf-8") if args.out else sys.stdout
    try:
        for line in decrypt(args.log, args.key):
            out.write(line + "\n")
    finally:
        if args.out:
            out.close()


if __name__ == "__main__":
    main()
