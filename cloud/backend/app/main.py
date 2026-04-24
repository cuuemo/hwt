from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from fastapi.responses import FileResponse
import os
from sqlalchemy.exc import IntegrityError, OperationalError

from app.api import auth, verify, admin
from app.database import engine, Base, SessionLocal
from app.crypto import CryptoManager
from app.config import RSA_KEY_DIR, DEFAULT_ADMIN_USERNAME, DEFAULT_ADMIN_PASSWORD
from app.models import User, UserRole, UserStatus
from app.auth import hash_password

def ensure_schema():
    """Create tables once, tolerating startup races between multiple workers."""
    try:
        Base.metadata.create_all(bind=engine)
    except OperationalError as exc:
        if "already exists" in str(exc).lower():
            print("[INIT] 数据表已由其他 worker 创建")
        else:
            raise


# 创建所有表
ensure_schema()

app = FastAPI(title="AT 网维系统云端", version="1.0.0")

# CORS (允许前端访问)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # 生产环境改为具体域名
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# 初始化加密管理器 (全局单例)
crypto = CryptoManager(RSA_KEY_DIR)

# 挂载路由
app.include_router(auth.router, prefix="/api/auth", tags=["认证"])
app.include_router(verify.router, prefix="/api/verify", tags=["验证"])
app.include_router(admin.router, prefix="/api/admin", tags=["管理"])


@app.on_event("startup")
async def init_admin():
    """启动事件: 确保至少有一个 admin 用户, 没有则创建默认管理员."""
    db = SessionLocal()
    try:
        admin_user = db.query(User).filter(User.role == UserRole.admin).first()
        if admin_user is None:
            admin_user = User(
                username=DEFAULT_ADMIN_USERNAME,
                password_hash=hash_password(DEFAULT_ADMIN_PASSWORD),
                role=UserRole.admin,
                status=UserStatus.active,
            )
            db.add(admin_user)
            try:
                db.commit()
                print(f"[INIT] 已创建默认管理员用户: {DEFAULT_ADMIN_USERNAME}")
            except IntegrityError:
                # Multiple workers can race during startup. If another worker
                # created the admin first, treat it as success.
                db.rollback()
                admin_user = db.query(User).filter(User.role == UserRole.admin).first()
                if admin_user is not None:
                    print(f"[INIT] 管理员已由其他 worker 创建: {admin_user.username}")
                else:
                    raise
        else:
            print(f"[INIT] 已存在管理员用户: {admin_user.username}")
    finally:
        db.close()


@app.get("/healthz")
def healthz():
    """健康检查."""
    return {"status": "ok", "service": "AT 网维系统云端"}


# Vue SPA (cloud/frontend) dist, copied into the image by the Dockerfile.
_FRONTEND_DIST = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "frontend_dist")
)
_INDEX_HTML = os.path.join(_FRONTEND_DIST, "index.html")
_ASSETS_DIR = os.path.join(_FRONTEND_DIST, "assets")

if os.path.isdir(_ASSETS_DIR):
    app.mount("/assets", StaticFiles(directory=_ASSETS_DIR), name="assets")


@app.get("/{full_path:path}", include_in_schema=False)
def spa_fallback(full_path: str):
    """Serve Vue SPA for any non-API path so client-side routing works."""
    if full_path.startswith("api/"):
        raise HTTPException(status_code=404, detail="Not Found")
    if not os.path.isfile(_INDEX_HTML):
        raise HTTPException(status_code=503, detail="frontend not built")
    return FileResponse(_INDEX_HTML)
