from fastapi import APIRouter, Depends, HTTPException, Request, status
from sqlalchemy.orm import Session

from app.database import get_db
from app.models import User, UserStatus, UserRole, AuthLog
from app.schemas import (
    RegisterRequest,
    RegisterResponse,
    LoginRequest,
    LoginResponse,
    UserOut,
)
from app.auth import hash_password, verify_password, create_access_token

router = APIRouter()


def _get_crypto():
    """获取全局 CryptoManager 实例 (延迟导入避免循环依赖)."""
    from app.main import crypto
    return crypto


@router.get("/public-key")
def get_public_key():
    """返回 RSA 公钥 PEM."""
    cm = _get_crypto()
    return {"public_key": cm.get_public_key_pem()}


@router.post("/register", response_model=RegisterResponse)
def register(req: RegisterRequest, request: Request, db: Session = Depends(get_db)):
    """
    用户注册.
    1. RSA 解密 password_encrypted -> 明文密码
    2. bcrypt 哈希密码
    3. 创建 User (status=pending)
    4. 记录 AuthLog
    """
    # 检查用户名是否已存在
    existing = db.query(User).filter(User.username == req.username).first()
    if existing:
        # 记录失败日志
        log = AuthLog(
            username=req.username,
            action="register",
            ip_address=request.client.host if request.client else None,
            result="failed",
            detail="用户名已存在",
        )
        db.add(log)
        db.commit()
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="用户名已存在",
        )

    # RSA 解密密码
    cm = _get_crypto()
    try:
        plain_password = cm.rsa_decrypt(req.password_encrypted).decode("utf-8")
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="密码解密失败",
        )

    # 创建用户
    user = User(
        username=req.username,
        password_hash=hash_password(plain_password),
        email=req.email,
        role=UserRole.user,
        status=UserStatus.active,
    )
    db.add(user)
    db.commit()
    db.refresh(user)

    # 记录成功日志
    log = AuthLog(
        user_id=user.id,
        username=user.username,
        action="register",
        ip_address=request.client.host if request.client else None,
        result="success",
    )
    db.add(log)
    db.commit()

    return RegisterResponse(
        id=user.id,
        username=user.username,
        status=user.status.value if isinstance(user.status, UserStatus) else user.status,
        message="注册成功，请联系管理员获取授权",
    )


@router.post("/login", response_model=LoginResponse)
def login(req: LoginRequest, request: Request, db: Session = Depends(get_db)):
    """
    用户登录.
    1. RSA 解密密码
    2. 验证账号密码
    3. 检查账号状态
    4. 返回 JWT token
    """
    ip_address = request.client.host if request.client else None

    # RSA 解密密码
    cm = _get_crypto()
    try:
        plain_password = cm.rsa_decrypt(req.password_encrypted).decode("utf-8")
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="密码解密失败",
        )

    # 查找用户
    user = db.query(User).filter(User.username == req.username).first()
    if user is None or not verify_password(plain_password, user.password_hash):
        # 记录失败日志
        log = AuthLog(
            user_id=user.id if user else None,
            username=req.username,
            action="login",
            ip_address=ip_address,
            result="failed",
            detail="用户名或密码错误",
        )
        db.add(log)
        db.commit()
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="用户名或密码错误",
        )

    # 检查账号状态
    user_status = user.status.value if isinstance(user.status, UserStatus) else user.status
    if user_status != UserStatus.active.value:
        log = AuthLog(
            user_id=user.id,
            username=user.username,
            action="login",
            ip_address=ip_address,
            result="failed",
            detail="账号未激活",
        )
        db.add(log)
        db.commit()
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="账号未激活",
        )

    # 生成 JWT
    token = create_access_token(
        data={"user_id": user.id, "username": user.username, "role": user.role.value if isinstance(user.role, UserRole) else user.role}
    )

    # 记录成功日志
    log = AuthLog(
        user_id=user.id,
        username=user.username,
        action="login",
        ip_address=ip_address,
        result="success",
    )
    db.add(log)
    db.commit()

    user_role = user.role.value if isinstance(user.role, UserRole) else user.role
    user_status_val = user.status.value if isinstance(user.status, UserStatus) else user.status

    return LoginResponse(
        access_token=token,
        token_type="bearer",
        user=UserOut(
            id=user.id,
            username=user.username,
            email=user.email,
            role=user_role,
            status=user_status_val,
            license_type=user.license_type.value if user.license_type else None,
            license_expire_at=user.license_expire_at,
            created_at=user.created_at,
            updated_at=user.updated_at,
        ),
    )
