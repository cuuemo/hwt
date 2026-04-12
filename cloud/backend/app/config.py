import os

DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./at.db")
# PostgreSQL: "postgresql://user:pass@host/dbname"

RSA_KEY_DIR = os.getenv("RSA_KEY_DIR", "./keys")
# 启动时如果不存在, 自动生成 rsa_private.pem + rsa_public.pem

JWT_SECRET = os.getenv("JWT_SECRET", "change-me-in-production")
JWT_ALGORITHM = "HS256"
JWT_EXPIRE_MINUTES = 60 * 24  # 24 小时

# AES 会话缓存 TTL
SESSION_TTL_HOURS = 24

# 默认管理员账号
DEFAULT_ADMIN_USERNAME = os.getenv("DEFAULT_ADMIN_USERNAME", "admin")
DEFAULT_ADMIN_PASSWORD = os.getenv("DEFAULT_ADMIN_PASSWORD", "admin123")
