# AT 网维系统

网吧/机房显示器注册表清理工具，支持云端授权验证、机器绑定、自动开机清理。

## 架构

```
互联网
└── 云服务器 (Python FastAPI，端口 10000)
        ↕ HTTP
内网
└── 网维服务器 (at-server.exe，静默运行)
    │   Web UI: http://服务器IP:19880 (登录后管理)
    │   TCP :19800 RSA+AES 加密通信
    ├── 工作站1 (at-client.exe，Windows Service)
    │       Web UI: http://工作站IP:19881 (状态查看)
    ├── 工作站2 (at-client.exe，Windows Service)
    └── 工作站N ...
```

**验证链路**：客户端 → 网维服务器 → 云端

客户端无需账号，连上已授权的网维服务器即可工作。

**Web UI**：服务器和客户端均内嵌 Web 页面（axum + WebSocket），程序静默运行，通过浏览器访问控制。支持中英文切换。

---

## 下载 EXE

前往 [Releases](https://github.com/cuuemo/at/releases) 下载：

| 文件 | 用途 |
|------|------|
| `at-server-ip.exe` | 网维服务器（连接测试云端 159.195.77.25:10000）|
| `at-server-domain.exe` | 网维服务器（连接正式云端 cuuemo.cn:10000）|
| `at-client.exe` | 工作站客户端（两个版本通用）|

---

## 完整使用流程

### 1. 用户注册

浏览器打开 `http://服务器IP:19880`，点击「Register」注册账号。

- 注册即激活，无需管理员审核
- 注册后**无法登录**，需等管理员设置授权时间

### 2. 管理员授权

登录云端管理后台（`http://cuuemo.cn:10000/admin` 或 `http://159.195.77.25:10000/admin`）：

1. 进入「用户管理」找到新注册用户
2. 设置「授权类型」（月付 / 年付 / 永久）
3. 设置「到期时间」（永久授权无需填写）
4. 保存

### 3. 网维服务器登录

浏览器打开 `http://服务器IP:19880`，输入账号密码登录：

- 云端验证账号密码 ✓
- 检查授权类型和到期时间 ✓
- 首次登录自动绑定本机机器码
- 登录成功后自动跳转 Dashboard，开始监听工作站连接（TCP 19800）
- Dashboard 实时显示在线客户端列表和日志

### 4. 工作站客户端运行

工作站开机后自动：

1. 扫描内网找到网维服务器（TCP 19800）
2. RSA + AES 加密握手
3. 请求授权（服务器已登录则通过）
4. 清理显示器注册表 + 随机化机器标识
5. 每 5 分钟心跳保活

可通过浏览器打开 `http://工作站IP:19881` 查看客户端实时状态和日志。

### 5. 换机器（解绑）

如需更换网维服务器机器，管理员在后台「机器绑定」中解绑旧机器码，用户重新登录即可绑定新机器。

---



### 方式A：Docker 原生部署（推荐）

**环境要求**：Linux 服务器，已安装 Docker 和 Docker Compose

```bash
# 1. 克隆项目或上传 cloud/backend/ 目录到服务器
git clone https://github.com/cuuemo/at.git
cd at/cloud/backend

# 2. 修改密码（必须改，否则有安全风险）
nano docker-compose.yml
# 修改以下两行：
#   JWT_SECRET=你的随机密钥（随便打一串字符）
#   DEFAULT_ADMIN_PASSWORD=你的管理员密码

# 3. 启动
docker compose up -d

# 4. 查看日志
docker logs -f at-cloud

# 常用命令
docker compose stop        # 停止
docker compose restart     # 重启
docker compose down        # 停止并删除容器（数据保留在 ./data/）
```

服务启动后：
- API 接口：`http://159.195.77.25:10000`
- Swagger 文档：`http://159.195.77.25:10000/docs`
- 数据持久化在 `cloud/backend/data/` 目录

---

### 方式B：宝塔面板 Docker 部署

**前提**：宝塔面板已安装 Docker 管理器插件

#### 步骤 1 — 上传文件

在宝塔文件管理器中，上传 `cloud/backend/` 整个目录到服务器，例如 `/www/at/`

#### 步骤 2 — 修改配置

打开 `/www/at/docker-compose.yml`，修改环境变量：

```yaml
environment:
  - JWT_SECRET=你的随机密钥
  - DEFAULT_ADMIN_USERNAME=admin
  - DEFAULT_ADMIN_PASSWORD=你的管理员密码
```

#### 步骤 3 — 构建并启动

在宝塔面板 **终端** 中执行：

```bash
cd /www/at
docker compose up -d --build
```

#### 步骤 4 — 宝塔防火墙放行端口

**安全** → **系统防火墙** → 添加端口规则：

| 端口 | 协议 | 说明 |
|------|------|------|
| 10000 | TCP | AT 云端 API |

#### 步骤 5 — （可选）配置反向代理

在宝塔 **网站** 中添加站点 `cuuemo.cn`，配置反向代理：

- 目标 URL：`http://127.0.0.1:10000`
- 开启 SSL（Let's Encrypt 免费证书）

配置好后客户端可用 `https://cuuemo.cn` 访问（需重新编译 server 改为 https）。

---

### 方式C：原生 Python 部署（不用 Docker）

```bash
cd cloud/backend
pip3 install -r requirements.txt
bash start.sh

# 或 systemd 守护进程（开机自启）
cp at-cloud.service /etc/systemd/system/
nano /etc/systemd/system/at-cloud.service   # 修改密码
systemctl daemon-reload
systemctl enable --now at-cloud
```

---

## 二、网维服务器部署（网吧主控机，Windows）

1. 从 Releases 下载 `at-server-ip.exe`（测试）或 `at-server-domain.exe`（正式）
2. **右键 → 以管理员身份运行**（程序静默运行，无窗口）
3. 浏览器打开 `http://127.0.0.1:19880`，登录并管理
4. 授权成功后自动监听局域网 TCP 19800 端口

---

## 三、工作站客户端部署（Windows，以管理员身份运行）

```bat
# 安装为系统服务（开机自启，SYSTEM 权限自动清理注册表）
at-client.exe install

# 查看服务状态
at-client.exe status

# 前台运行（调试用）
at-client.exe run

# 卸载服务
at-client.exe uninstall
```

安装后服务名：`AtCleanupService`

工作流程：开机 → 扫描内网找到网维服务器 → 加密握手授权 → 清理 `HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY` 注册表 → 每 5 分钟心跳保活

客户端 Web UI：`http://工作站IP:19881`（查看连接状态、清理结果、实时日志）

---

## 四、迁移（IP → 域名）

测试通过后：

1. 将 `dist/domain/at-server.exe` 替换到网维服务器
2. 云端服务无需改动，在新服务器按上面步骤重新部署即可

---

## 五、重新编译

```bash
# 一次编译 IP 版和域名版两份
bash build.sh

# 产物在：
# dist/ip/at-server-ip.exe   + at-client.exe
# dist/domain/at-server-domain.exe + at-client.exe
```

**编译环境要求**：
- Rust 1.70+
- `rustup target add x86_64-pc-windows-gnu`
- `apt install gcc-mingw-w64-x86-64`

---

## 六、注意事项

- 客户端需以 **SYSTEM 权限**运行才能修改注册表（安装为服务后自动满足）
- 网维服务器需以**管理员**身份运行
- 云端 `JWT_SECRET` 和 `DEFAULT_ADMIN_PASSWORD` 务必修改，勿使用默认值
- 默认管理员账号：`admin` / `admin123`
