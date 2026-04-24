from pydantic import BaseModel, Field
from typing import Optional, List, Any, Generic, TypeVar
from datetime import datetime
from enum import Enum

# ---------- 枚举 (用于 schema 层) ----------

class UserRoleEnum(str, Enum):
    admin = "admin"
    user = "user"


class UserStatusEnum(str, Enum):
    pending = "pending"
    active = "active"
    disabled = "disabled"


class LicenseTypeEnum(str, Enum):
    monthly = "monthly"
    yearly = "yearly"
    permanent = "permanent"


class BindingStatusEnum(str, Enum):
    active = "active"
    unbound = "unbound"


# ---------- Auth ----------

class RegisterRequest(BaseModel):
    username: str = Field(..., min_length=2, max_length=64)
    password_encrypted: str = Field(..., description="RSA 加密后的 base64 密码")
    email: Optional[str] = Field(None, max_length=128)


class LoginRequest(BaseModel):
    username: str = Field(..., min_length=2, max_length=64)
    password_encrypted: str = Field(..., description="RSA 加密后的 base64 密码")


class UserOut(BaseModel):
    id: int
    username: str
    email: Optional[str] = None
    role: str
    status: str
    license_type: Optional[str] = None
    license_expire_at: Optional[datetime] = None
    created_at: Optional[datetime] = None
    updated_at: Optional[datetime] = None

    model_config = {"from_attributes": True}


class LoginResponse(BaseModel):
    access_token: str
    token_type: str = "bearer"
    user: UserOut


class RegisterResponse(BaseModel):
    id: int
    username: str
    status: str
    message: str


# ---------- Verify ----------

class HandshakeRequest(BaseModel):
    encrypted_session_key: str = Field(..., description="RSA 加密的 32 字节 AES 密钥 (base64)")


class HandshakeResponse(BaseModel):
    session_id: str


class VerifyRequest(BaseModel):
    """客户端发来的 AES 加密验证请求."""
    session_id: str
    iv: str
    payload: str
    tag: str


class VerifyPlainRequest(BaseModel):
    """AES 解密后的明文验证请求."""
    account: str
    password: str
    machine_code: str


class VerifyResponse(BaseModel):
    """AES 加密后返回的验证响应外层."""
    iv: str
    payload: str
    tag: str


class VerifyPlainResponse(BaseModel):
    """验证结果明文 (加密前)."""
    authorized: bool
    license_type: Optional[str] = None
    expire_at: Optional[str] = None
    message: str


# ---------- Admin ----------

class UserUpdate(BaseModel):
    status: Optional[UserStatusEnum] = None
    role: Optional[UserRoleEnum] = None
    license_type: Optional[LicenseTypeEnum] = None
    license_expire_at: Optional[datetime] = None
    email: Optional[str] = None


class BindingOut(BaseModel):
    id: int
    user_id: int
    username: Optional[str] = None
    machine_code: str
    bound_at: Optional[datetime] = None
    last_verified_at: Optional[datetime] = None
    status: str

    model_config = {"from_attributes": True}


class LogOut(BaseModel):
    id: int
    user_id: Optional[int] = None
    username: Optional[str] = None
    machine_code: Optional[str] = None
    action: str
    ip_address: Optional[str] = None
    result: str
    detail: Optional[str] = None
    created_at: Optional[datetime] = None

    model_config = {"from_attributes": True}


T = TypeVar("T")


class PaginatedResponse(BaseModel, Generic[T]):
    total: int
    items: List[T]


class LogDecryptResponse(BaseModel):
    filename: str
    total_lines: int
    truncated: bool
    lines: List[str]
