import os
import base64
import uuid
from datetime import datetime, timedelta, timezone
from typing import Dict, Any, Tuple

from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

from app.config import SESSION_TTL_HOURS


class CryptoManager:
    """RSA 密钥管理 + AES-GCM 加解密 + 会话管理."""

    def __init__(self, key_dir: str):
        """加载或生成 RSA 2048 密钥对."""
        self.key_dir = key_dir
        os.makedirs(key_dir, exist_ok=True)

        private_key_path = os.path.join(key_dir, "rsa_private.pem")
        public_key_path = os.path.join(key_dir, "rsa_public.pem")

        if os.path.exists(private_key_path) and os.path.exists(public_key_path):
            # 加载已有密钥
            with open(private_key_path, "rb") as f:
                self.private_key = serialization.load_pem_private_key(f.read(), password=None)
            with open(public_key_path, "rb") as f:
                self.public_key = serialization.load_pem_public_key(f.read())
        else:
            # 生成新密钥对
            self.private_key = rsa.generate_private_key(
                public_exponent=65537,
                key_size=2048,
            )
            self.public_key = self.private_key.public_key()

            # 保存私钥
            with open(private_key_path, "wb") as f:
                f.write(
                    self.private_key.private_bytes(
                        encoding=serialization.Encoding.PEM,
                        format=serialization.PrivateFormat.PKCS8,
                        encryption_algorithm=serialization.NoEncryption(),
                    )
                )

            # 保存公钥
            with open(public_key_path, "wb") as f:
                f.write(
                    self.public_key.public_bytes(
                        encoding=serialization.Encoding.PEM,
                        format=serialization.PublicFormat.SubjectPublicKeyInfo,
                    )
                )

        # 缓存公钥 PEM 字符串
        self._public_key_pem = self.public_key.public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo,
        ).decode("utf-8")

        # 会话存储: session_id -> {"key": bytes, "created_at": datetime}
        self.sessions: Dict[str, Dict[str, Any]] = {}

    def get_public_key_pem(self) -> str:
        """返回 PEM 格式公钥字符串."""
        return self._public_key_pem

    def rsa_decrypt(self, ciphertext_b64: str) -> bytes:
        """RSA OAEP-SHA256 解密 (base64 输入)."""
        ciphertext = base64.b64decode(ciphertext_b64)
        return self.private_key.decrypt(
            ciphertext,
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA256()),
                algorithm=hashes.SHA256(),
                label=None,
            ),
        )

    @staticmethod
    def aes_encrypt(key: bytes, plaintext: bytes) -> Tuple[str, str, str]:
        """AES-256-GCM 加密, 返回 (iv_b64, ciphertext_b64, tag_b64)."""
        iv = os.urandom(12)
        aesgcm = AESGCM(key)
        ct_with_tag = aesgcm.encrypt(iv, plaintext, None)
        # AESGCM.encrypt 返回 ciphertext + tag (tag 是最后 16 bytes)
        ct = ct_with_tag[:-16]
        tag = ct_with_tag[-16:]
        return (
            base64.b64encode(iv).decode(),
            base64.b64encode(ct).decode(),
            base64.b64encode(tag).decode(),
        )

    @staticmethod
    def aes_decrypt(key: bytes, iv_b64: str, payload_b64: str, tag_b64: str) -> bytes:
        """AES-256-GCM 解密."""
        iv = base64.b64decode(iv_b64)
        ct = base64.b64decode(payload_b64)
        tag = base64.b64decode(tag_b64)
        aesgcm = AESGCM(key)
        return aesgcm.decrypt(iv, ct + tag, None)

    def create_session(self, session_key: bytes) -> str:
        """创建新会话, 返回 session_id."""
        session_id = str(uuid.uuid4())
        self.sessions[session_id] = {
            "key": session_key,
            "created_at": datetime.now(timezone.utc),
        }
        return session_id

    def get_session_key(self, session_id: str) -> bytes | None:
        """根据 session_id 获取会话密钥, 不存在或已过期返回 None."""
        session = self.sessions.get(session_id)
        if session is None:
            return None
        # 检查是否过期
        created_at = session["created_at"]
        if datetime.now(timezone.utc) - created_at > timedelta(hours=SESSION_TTL_HOURS):
            del self.sessions[session_id]
            return None
        return session["key"]

    def cleanup_expired_sessions(self) -> int:
        """清理所有过期会话, 返回清理数量."""
        now = datetime.now(timezone.utc)
        expired = [
            sid
            for sid, s in self.sessions.items()
            if now - s["created_at"] > timedelta(hours=SESSION_TTL_HOURS)
        ]
        for sid in expired:
            del self.sessions[sid]
        return len(expired)
