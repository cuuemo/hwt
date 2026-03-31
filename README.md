# HWT 网维系统

网吧/机房显示器注册表清理工具，支持云端授权验证、机器绑定、自动开机清理。

## 架构

```
互联网
└── 云服务器 (Python FastAPI)
        ↕ HTTPS :10000
内网
└── 网维服务器 (hwt-server.exe, egui GUI)
        ↕ TCP :19800 加密通信
    ├── 工作站1 (hwt-client.exe, Windows Service)
    ├── 工作站2 (hwt-client.exe, Windows Service)
    └── 工作站N ...
```

**验证链路**：客户端 → 网维服务器 → 云端

客户端无需账号，连上已授权的网维服务器即可工作。

---

## 目录结构

```
hwt/
├── client/          # 客户端 EXE (Rust, Windows Service)
├── server/          # 网维服务器 (Rust, egui GUI)
├── protocol/        # 共享通信协议 crate
├── cloud/
│   └── backend/     # 云端服务 (Python FastAPI)
│       ├── app/
│       ├── start.sh
│       └── hwt-cloud.service
└── dist/            # 编译产物输出目录
    ├── ip/          # 测试版 (连接 43.165.169.50:10000)
    └── domain/      # 正式版 (连接 cuuemo.cn:10000)
```

---

## 编译

### 环境要求

- Rust 1.70+
- `x86_64-pc-windows-gnu` target：`rustup target add x86_64-pc-windows-gnu`
- MinGW-w64：`apt install gcc-mingw-w64-x86-64`

### 一键编译两份

```bash
bash build.sh
```

输出：
- `dist/ip/hwt-server.exe`     — 连接 43.165.169.50:10000（测试）
- `dist/ip/hwt-client.exe`     — 客户端（两份相同）
- `dist/domain/hwt-server.exe` — 连接 cuuemo.cn:10000（正式）
- `dist/domain/hwt-client.exe` — 客户端（两份相同）

---

## 部署

### 1. 云端服务器

```bash
# 上传 cloud/backend/ 到服务器，然后：
pip3 install -r requirements.txt

# 直接启动
bash start.sh

# 或 systemd 守护进程
cp hwt-cloud.service /etc/systemd/system/
# 编辑 JWT_SECRET 和 DEFAULT_ADMIN_PASSWORD
systemctl daemon-reload && systemctl enable --now hwt-cloud
```

API 文档：`http://43.165.169.50:10000/docs`

默认管理员账号：`admin` / `admin123`（**部署前务必修改**）

### 2. 网维服务器（网吧收银机/主控机，Windows）

```
hwt-server.exe
```

双击运行，使用管理员账号登录云端，激活后开始监听 TCP 19800。

### 3. 客户端工作站（Windows，以管理员身份运行）

```bat
# 安装为系统服务（开机自启）
hwt-client.exe install

# 查看状态
hwt-client.exe status

# 卸载服务
hwt-client.exe uninstall

# 前台调试运行
hwt-client.exe run
```

安装后服务名：`HwtCleanupService`，随系统启动，自动扫描内网找到网维服务器，验证通过后清理 `HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY` 注册表。

---

## 云端 API 概览

| 方法 | 路径 | 说明 |
|------|------|------|
| GET  | `/` | 健康检查 |
| POST | `/api/auth/login` | 管理员登录 |
| GET  | `/api/auth/public-key` | 获取 RSA 公钥 |
| POST | `/api/verify/handshake` | 建立 AES 会话 |
| POST | `/api/verify` | 加密验证账号+机器码 |
| GET  | `/api/admin/users` | 用户列表 |
| POST | `/api/admin/users` | 创建用户 |

完整文档见 `/docs`（Swagger UI）。

---

## 迁移（IP → 域名）

测试通过后切换到正式版：

1. 将 `dist/domain/hwt-server.exe` 替换到网维服务器
2. 云端代码无需改动，直接在新服务器部署即可

---

## 注意事项

- 客户端需以 **SYSTEM 权限**运行才能修改注册表（安装为服务后自动满足）
- 网维服务器需以**管理员**身份运行
- 云端 `JWT_SECRET` 请设置为随机强密码，勿使用默认值
