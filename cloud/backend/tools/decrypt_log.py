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
