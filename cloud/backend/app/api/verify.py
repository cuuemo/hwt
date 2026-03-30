import json
from datetime import datetime, timezone

from fastapi import APIRouter, Depends, HTTPException, Request, status
from sqlalchemy.orm import Session

from app.database import get_db
from app.models import (
    User,
    UserStatus,
    LicenseType,
    MachineBinding,
    BindingStatus,
    AuthLog,
)
from app.schemas import (
    HandshakeRequest,
    HandshakeResponse,
    VerifyRequest,
    VerifyResponse,
)
from app.auth import verify_password

router = APIRouter()


def _get_crypto():
    """获取全局 CryptoManager 实例."""
    from app.main import crypto
    return crypto


@router.post("/handshake", response_model=HandshakeResponse)
def handshake(req: HandshakeRequest):
    """
    RSA 握手: 客户端用 RSA 公钥加密 32 字节 AES 会话密钥, 服务端解密并缓存.
    """
    cm = _get_crypto()
    try:
        session_key = cm.rsa_decrypt(req.encrypted_session_key)
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="会话密钥解密失败",
        )

    if len(session_key) != 32:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="会话密钥长度无效, 需要 32 字节",
        )

    session_id = cm.create_session(session_key)
    return HandshakeResponse(session_id=session_id)


@router.post("/", response_model=VerifyResponse)
def verify(req: VerifyRequest, request: Request, db: Session = Depends(get_db)):
    """
    AES 加密验证接口.
    1. 根据 session_id 查找 session_key
    2. AES-GCM 解密 -> 明文 JSON (account, password, machine_code)
    3. 验证账号密码
    4. 检查用户状态 == active
    5. 检查授权 (license_type, license_expire_at)
    6. 检查机器码绑定
    7. 记录 AuthLog
    8. AES 加密响应
    """
    cm = _get_crypto()
    ip_address = request.client.host if request.client else None

    # 1. 查找会话密钥
    session_key = cm.get_session_key(req.session_id)
    if session_key is None:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="会话不存在或已过期, 请重新握手",
        )

    # 2. AES 解密
    try:
        plaintext = cm.aes_decrypt(session_key, req.iv, req.payload, req.tag)
        plain_data = json.loads(plaintext.decode("utf-8"))
        account = plain_data["account"]
        password = plain_data["password"]
        machine_code = plain_data["machine_code"]
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="验证请求解密失败",
        )

    # 辅助函数: 构造加密响应
    def make_response(authorized: bool, message: str,
                      license_type: str = None, expire_at: str = None) -> VerifyResponse:
        resp_data = {
            "authorized": authorized,
            "license_type": license_type,
            "expire_at": expire_at,
            "message": message,
        }
        resp_bytes = json.dumps(resp_data, ensure_ascii=False).encode("utf-8")
        iv_b64, payload_b64, tag_b64 = cm.aes_encrypt(session_key, resp_bytes)
        return VerifyResponse(iv=iv_b64, payload=payload_b64, tag=tag_b64)

    # 3. 验证账号密码
    user = db.query(User).filter(User.username == account).first()
    if user is None or not verify_password(password, user.password_hash):
        # 记录失败日志
        log = AuthLog(
            user_id=user.id if user else None,
            username=account,
            machine_code=machine_code,
            action="verify",
            ip_address=ip_address,
            result="failed",
            detail="账号或密码错误",
        )
        db.add(log)
        db.commit()
        return make_response(False, "账号或密码错误")

    # 4. 检查用户状态
    user_status = user.status.value if isinstance(user.status, UserStatus) else user.status
    if user_status != UserStatus.active.value:
        log = AuthLog(
            user_id=user.id,
            username=account,
            machine_code=machine_code,
            action="verify",
            ip_address=ip_address,
            result="failed",
            detail="账号未激活, 请联系管理员",
        )
        db.add(log)
        db.commit()
        return make_response(False, "账号未激活, 请联系管理员")

    # 5. 检查授权
    if user.license_type is None:
        log = AuthLog(
            user_id=user.id,
            username=account,
            machine_code=machine_code,
            action="verify",
            ip_address=ip_address,
            result="failed",
            detail="未设置授权类型",
        )
        db.add(log)
        db.commit()
        return make_response(False, "未设置授权类型, 请联系管理员")

    lt = user.license_type.value if isinstance(user.license_type, LicenseType) else user.license_type
    if lt != LicenseType.permanent.value:
        if user.license_expire_at is None:
            log = AuthLog(
                user_id=user.id,
                username=account,
                machine_code=machine_code,
                action="verify",
                ip_address=ip_address,
                result="failed",
                detail="授权到期时间未设置",
            )
            db.add(log)
            db.commit()
            return make_response(False, "授权到期时间未设置, 请联系管理员")

        expire_at_aware = user.license_expire_at
        if expire_at_aware.tzinfo is None:
            expire_at_aware = expire_at_aware.replace(tzinfo=timezone.utc)
        if expire_at_aware < datetime.now(timezone.utc):
            log = AuthLog(
                user_id=user.id,
                username=account,
                machine_code=machine_code,
                action="verify",
                ip_address=ip_address,
                result="failed",
                detail="授权已过期, 请续费",
            )
            db.add(log)
            db.commit()
            return make_response(False, "授权已过期, 请续费")

    # 6. 检查机器码绑定
    active_binding = (
        db.query(MachineBinding)
        .filter(
            MachineBinding.user_id == user.id,
            MachineBinding.status == BindingStatus.active,
        )
        .first()
    )

    if active_binding is None:
        # 首次绑定
        new_binding = MachineBinding(
            user_id=user.id,
            machine_code=machine_code,
            status=BindingStatus.active,
        )
        db.add(new_binding)
        db.commit()
    elif active_binding.machine_code == machine_code:
        # 匹配 -> 更新 last_verified_at
        active_binding.last_verified_at = datetime.now(timezone.utc)
        db.commit()
    else:
        # 不匹配 -> 拒绝
        log = AuthLog(
            user_id=user.id,
            username=account,
            machine_code=machine_code,
            action="verify",
            ip_address=ip_address,
            result="failed",
            detail="机器码不匹配, 请联系管理员解绑",
        )
        db.add(log)
        db.commit()
        return make_response(False, "机器码不匹配, 请联系管理员解绑")

    # 7. 记录成功日志
    log = AuthLog(
        user_id=user.id,
        username=account,
        machine_code=machine_code,
        action="verify",
        ip_address=ip_address,
        result="success",
    )
    db.add(log)
    db.commit()

    # 8. 构造成功响应
    expire_at_str = None
    if lt != LicenseType.permanent.value and user.license_expire_at:
        expire_at_str = user.license_expire_at.isoformat() + "Z" if user.license_expire_at.tzinfo is None else user.license_expire_at.isoformat()

    return make_response(
        authorized=True,
        message="验证成功",
        license_type=lt,
        expire_at=expire_at_str,
    )
