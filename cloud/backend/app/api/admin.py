from datetime import datetime
from typing import Optional

from fastapi import APIRouter, Depends, File, HTTPException, Query, UploadFile, status
from sqlalchemy.orm import Session
from sqlalchemy import func as sa_func

from app.database import get_db
from app.models import (
    User,
    UserRole,
    UserStatus,
    LicenseType,
    MachineBinding,
    BindingStatus,
    AuthLog,
)
from app.schemas import (
    UserOut,
    UserUpdate,
    BindingOut,
    LogOut,
    PaginatedResponse,
    LogDecryptResponse,
)
from app.auth import require_admin
from app.log_decrypt import decrypt_log_bytes, LogDecryptError

MAX_LOG_BYTES = 20 * 1024 * 1024  # 20 MB
MAX_LOG_LINES = 50_000

router = APIRouter()


# ---------- 用户管理 ----------

@router.get("/users", response_model=PaginatedResponse[UserOut])
def list_users(
    page: int = Query(1, ge=1),
    size: int = Query(20, ge=1, le=100),
    status: Optional[str] = Query(None, description="按状态筛选: pending/active/disabled"),
    username: Optional[str] = Query(None, description="按用户名模糊搜索"),
    db: Session = Depends(get_db),
    _admin: User = Depends(require_admin),
):
    """分页查询用户列表."""
    query = db.query(User)

    if status:
        query = query.filter(User.status == status)
    if username:
        query = query.filter(User.username.contains(username))

    total = query.count()
    users = query.order_by(User.id.desc()).offset((page - 1) * size).limit(size).all()

    items = []
    for u in users:
        items.append(
            UserOut(
                id=u.id,
                username=u.username,
                email=u.email,
                role=u.role.value if isinstance(u.role, UserRole) else u.role,
                status=u.status.value if isinstance(u.status, UserStatus) else u.status,
                license_type=u.license_type.value if u.license_type and isinstance(u.license_type, LicenseType) else u.license_type,
                license_expire_at=u.license_expire_at,
                created_at=u.created_at,
                updated_at=u.updated_at,
            )
        )

    return PaginatedResponse(total=total, items=items)


@router.patch("/users/{user_id}", response_model=UserOut)
def update_user(
    user_id: int,
    update: UserUpdate,
    db: Session = Depends(get_db),
    _admin: User = Depends(require_admin),
):
    """部分更新用户 (审核/禁用/设置授权)."""
    user = db.query(User).filter(User.id == user_id).first()
    if user is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="用户不存在",
        )

    update_data = update.model_dump(exclude_unset=True)

    if "status" in update_data and update_data["status"] is not None:
        user.status = update_data["status"]
    if "role" in update_data and update_data["role"] is not None:
        user.role = update_data["role"]
    if "license_type" in update_data and update_data["license_type"] is not None:
        user.license_type = update_data["license_type"]
    if "license_expire_at" in update_data:
        user.license_expire_at = update_data["license_expire_at"]
    if "email" in update_data:
        user.email = update_data["email"]

    db.commit()
    db.refresh(user)

    return UserOut(
        id=user.id,
        username=user.username,
        email=user.email,
        role=user.role.value if isinstance(user.role, UserRole) else user.role,
        status=user.status.value if isinstance(user.status, UserStatus) else user.status,
        license_type=user.license_type.value if user.license_type and isinstance(user.license_type, LicenseType) else user.license_type,
        license_expire_at=user.license_expire_at,
        created_at=user.created_at,
        updated_at=user.updated_at,
    )


# ---------- 绑定管理 ----------

@router.get("/bindings", response_model=PaginatedResponse[BindingOut])
def list_bindings(
    page: int = Query(1, ge=1),
    size: int = Query(20, ge=1, le=100),
    user_id: Optional[int] = Query(None, description="按用户 ID 筛选"),
    db: Session = Depends(get_db),
    _admin: User = Depends(require_admin),
):
    """分页查询机器码绑定列表."""
    query = db.query(MachineBinding)

    if user_id is not None:
        query = query.filter(MachineBinding.user_id == user_id)

    total = query.count()
    bindings = query.order_by(MachineBinding.id.desc()).offset((page - 1) * size).limit(size).all()

    items = []
    for b in bindings:
        # 查询关联用户名
        user = db.query(User).filter(User.id == b.user_id).first()
        username = user.username if user else None
        items.append(
            BindingOut(
                id=b.id,
                user_id=b.user_id,
                username=username,
                machine_code=b.machine_code,
                bound_at=b.bound_at,
                last_verified_at=b.last_verified_at,
                status=b.status.value if isinstance(b.status, BindingStatus) else b.status,
            )
        )

    return PaginatedResponse(total=total, items=items)


@router.delete("/bindings/{binding_id}")
def unbind(
    binding_id: int,
    db: Session = Depends(get_db),
    _admin: User = Depends(require_admin),
):
    """软删除绑定 (status -> unbound)."""
    binding = db.query(MachineBinding).filter(MachineBinding.id == binding_id).first()
    if binding is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="绑定记录不存在",
        )

    binding.status = BindingStatus.unbound
    db.commit()

    # 记录解绑日志
    user = db.query(User).filter(User.id == binding.user_id).first()
    log = AuthLog(
        user_id=binding.user_id,
        username=user.username if user else None,
        machine_code=binding.machine_code,
        action="unbind",
        result="success",
        detail=f"管理员解绑, binding_id={binding_id}",
    )
    db.add(log)
    db.commit()

    return {"message": "解绑成功"}


# ---------- 日志查询 ----------

@router.get("/logs", response_model=PaginatedResponse[LogOut])
def list_logs(
    page: int = Query(1, ge=1),
    size: int = Query(50, ge=1, le=200),
    username: Optional[str] = Query(None, description="按用户名筛选"),
    action: Optional[str] = Query(None, description="按操作类型筛选: login/verify/register/unbind"),
    date_from: Optional[str] = Query(None, description="开始日期 (YYYY-MM-DD)"),
    date_to: Optional[str] = Query(None, description="结束日期 (YYYY-MM-DD)"),
    db: Session = Depends(get_db),
    _admin: User = Depends(require_admin),
):
    """分页查询操作日志."""
    query = db.query(AuthLog)

    if username:
        query = query.filter(AuthLog.username.contains(username))
    if action:
        query = query.filter(AuthLog.action == action)
    if date_from:
        try:
            dt_from = datetime.strptime(date_from, "%Y-%m-%d")
            query = query.filter(AuthLog.created_at >= dt_from)
        except ValueError:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="date_from 格式无效, 应为 YYYY-MM-DD",
            )
    if date_to:
        try:
            dt_to = datetime.strptime(date_to, "%Y-%m-%d")
            # 包含当天: 加到次日 00:00
            dt_to = dt_to.replace(hour=23, minute=59, second=59)
            query = query.filter(AuthLog.created_at <= dt_to)
        except ValueError:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="date_to 格式无效, 应为 YYYY-MM-DD",
            )

    total = query.count()
    logs = query.order_by(AuthLog.id.desc()).offset((page - 1) * size).limit(size).all()

    items = []
    for log in logs:
        items.append(
            LogOut(
                id=log.id,
                user_id=log.user_id,
                username=log.username,
                machine_code=log.machine_code,
                action=log.action,
                ip_address=log.ip_address,
                result=log.result,
                detail=log.detail,
                created_at=log.created_at,
            )
        )

    return PaginatedResponse(total=total, items=items)


# ---------- 客户端日志解密 ----------

@router.post("/logs/decrypt", response_model=LogDecryptResponse)
async def decrypt_client_log(
    file: UploadFile = File(...),
    _admin: User = Depends(require_admin),
):
    """Decrypt an uploaded .log.enc file using the cloud RSA private key."""
    from app.main import crypto

    if file.size is not None and file.size > MAX_LOG_BYTES:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"文件过大 (>{MAX_LOG_BYTES // (1024 * 1024)} MB)",
        )
    data = await file.read(MAX_LOG_BYTES + 1)
    if len(data) > MAX_LOG_BYTES:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"文件过大 (>{MAX_LOG_BYTES // (1024 * 1024)} MB)",
        )

    lines: list[str] = []
    truncated = False
    try:
        for line in decrypt_log_bytes(data, crypto.private_key):
            if len(lines) >= MAX_LOG_LINES:
                truncated = True
                break
            lines.append(line)
    except LogDecryptError as exc:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=str(exc),
        )

    return LogDecryptResponse(
        filename=file.filename or "uploaded.log.enc",
        total_lines=len(lines),
        truncated=truncated,
        lines=lines,
    )
