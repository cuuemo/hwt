from sqlalchemy import Column, Integer, String, DateTime, Enum, ForeignKey
from sqlalchemy.sql import func
from app.database import Base
import enum


class UserRole(str, enum.Enum):
    admin = "admin"
    user = "user"


class UserStatus(str, enum.Enum):
    pending = "pending"      # 注册待审核
    active = "active"        # 已激活
    disabled = "disabled"    # 已禁用


class LicenseType(str, enum.Enum):
    monthly = "monthly"
    yearly = "yearly"
    permanent = "permanent"


class BindingStatus(str, enum.Enum):
    active = "active"
    unbound = "unbound"


class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True, autoincrement=True)
    username = Column(String(64), unique=True, nullable=False, index=True)
    password_hash = Column(String(128), nullable=False)
    email = Column(String(128), nullable=True)
    role = Column(Enum(UserRole), default=UserRole.user, nullable=False)
    status = Column(Enum(UserStatus), default=UserStatus.pending, nullable=False)
    license_type = Column(Enum(LicenseType), nullable=True)   # 管理员设置
    license_expire_at = Column(DateTime, nullable=True)        # permanent 时为 NULL
    created_at = Column(DateTime, server_default=func.now())
    updated_at = Column(DateTime, server_default=func.now(), onupdate=func.now())


class MachineBinding(Base):
    __tablename__ = "machine_bindings"

    id = Column(Integer, primary_key=True, autoincrement=True)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=False, index=True)
    machine_code = Column(String(64), nullable=False, index=True)  # SHA256 hex
    bound_at = Column(DateTime, server_default=func.now())
    last_verified_at = Column(DateTime, server_default=func.now())
    status = Column(Enum(BindingStatus), default=BindingStatus.active)


class AuthLog(Base):
    __tablename__ = "auth_logs"

    id = Column(Integer, primary_key=True, autoincrement=True)
    user_id = Column(Integer, ForeignKey("users.id"), nullable=True)
    username = Column(String(64), nullable=True)       # 即使 user 不存在也记录
    machine_code = Column(String(64), nullable=True)
    action = Column(String(32), nullable=False)         # login / verify / unbind / register
    ip_address = Column(String(45), nullable=True)
    result = Column(String(16), nullable=False)         # success / failed
    detail = Column(String(256), nullable=True)         # 失败原因等
    created_at = Column(DateTime, server_default=func.now())
