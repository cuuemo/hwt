# 网维系统 - 完整实施文档

> 本文档详细到可以直接照着写代码，每个模块的输入输出、API 格式、字节级协议、Win32 调用参数均有说明。

---

## 一、架构总览

```
┌─────────────────────────────────────────────────────────┐
│                    互联网 (公网)                          │
│  ┌─────────────────────────────────────────────────┐    │
│  │          云服务器 (Python FastAPI + Vue 3)        │    │
│  │  - 账号注册/管理 (自助注册, 管理员审核)             │    │
│  │  - 授权管理 (月付/年付/永久)                       │    │
│  │  - 机器码绑定/解绑                                 │    │
│  │  - Web 管理后台 (Element Plus)                    │    │
│  └──────────────────────┬──────────────────────────┘    │
│                         │ HTTPS API                      │
└─────────────────────────┼───────────────────────────────┘
                          │
┌─────────────────────────┼───────────────────────────────┐
│                    内网 (网吧 LAN)                        │
│                         │                                │
│  ┌──────────────────────┴──────────────────────────┐    │
│  │       网维服务器 (Rust, Windows, egui GUI)         │    │
│  │  - 监听 TCP 19800                                 │    │
│  │  - 店老板登录账号密码                               │    │
│  │  - 采集机器码, 绑定到账号                           │    │
│  │  - RSA+AES 加密与云端/客户端通信                    │    │
│  │  - GUI: 登录、状态、在线列表、日志                   │    │
│  └──┬────────────┬────────────┬─────────────────────┘    │
│     │ TCP:19800  │            │                          │
│  ┌──┴──┐     ┌──┴──┐     ┌──┴──┐                       │
│  │客户端│     │客户端│     │客户端│  (工作站 x N)         │
│  │ EXE │     │ EXE │     │ EXE │                        │
│  └─────┘     └─────┘     └─────┘                        │
│  Rust, Windows Service (SYSTEM), 无 GUI                  │
│  开机 → 扫描 19800 → 加密握手 → 授权 → 清理设备          │
└─────────────────────────────────────────────────────────┘
```

**验证链路**: 客户端 →(TCP 19800 AES)→ 网维服务器 →(HTTPS RSA+AES)→ 云端
**客户端无需账号**, 连上已授权的网维服务器即可工作。

---

## 二、项目目录结构

```
hwt/
├── client/                     # 客户端 EXE (Rust)
│   ├── Cargo.toml
│   ├── build.rs                # 嵌入 Windows manifest (requireAdministrator)
│   └── src/
│       ├── main.rs             # 入口: install / uninstall / status / run
│       ├── service.rs          # Windows Service 生命周期
│       ├── scanner.rs          # 内网端口扫描
│       ├── registry.rs         # 显示器注册表清理 (删 DISPLAY)
│       ├── cleanup.rs          # 幽灵设备清理 (Setup API)
│       └── protocol.rs         # 与网维服务器通信 (复用 protocol crate)
│
├── server/                     # 网维服务器 (Rust + egui GUI)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # 入口
│       ├── gui.rs              # egui 界面 (登录页 + 主页)
│       ├── auth.rs             # 云端验证 (RSA握手 + AES通信)
│       ├── listener.rs         # TCP 监听, 接受客户端连接
│       ├── machine.rs          # 机器码采集 (WMI)
│       └── protocol.rs         # 通信协议 (复用 protocol crate)
│
├── protocol/                   # 共享 Rust lib crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # 消息类型定义 (serde)
│       ├── crypto.rs           # RSA + AES-256-GCM 封装
│       └── frame.rs            # TCP 帧编解码器
│
├── cloud/                      # 云服务器
│   ├── backend/                # Python FastAPI
│   │   ├── requirements.txt
│   │   ├── alembic.ini
│   │   ├── alembic/            # 数据库迁移
│   │   └── app/
│   │       ├── __init__.py
│   │       ├── main.py         # FastAPI app, CORS, 路由挂载
│   │       ├── config.py       # 配置 (数据库URL, RSA密钥路径, JWT密钥)
│   │       ├── database.py     # SQLAlchemy engine + session
│   │       ├── models.py       # ORM 模型
│   │       ├── schemas.py      # Pydantic request/response schema
│   │       ├── crypto.py       # RSA 密钥管理 + AES-GCM 加解密
│   │       ├── auth.py         # JWT 生成/验证, 密码哈希
│   │       └── api/
│   │           ├── __init__.py
│   │           ├── auth.py     # POST /register, /login, GET /public-key
│   │           ├── verify.py   # POST /verify/handshake, /verify
│   │           └── admin.py    # 用户管理, 绑定管理, 日志查询
│   │
│   └── frontend/               # Vue 3 + Vite + Element Plus
│       ├── package.json
│       ├── vite.config.ts      # 含 obfuscator 插件配置
│       └── src/
│           ├── main.ts
│           ├── App.vue
│           ├── router/index.ts
│           ├── api/            # axios 封装 + 各模块 API
│           │   ├── request.ts  # axios 实例, 拦截器, JWT header
│           │   ├── auth.ts     # 登录注册 API (含 RSA 加密)
│           │   ├── admin.ts    # 管理后台 API
│           │   └── crypto.ts   # jsencrypt RSA 封装
│           ├── views/
│           │   ├── Login.vue
│           │   ├── Dashboard.vue
│           │   ├── Users.vue
│           │   ├── Bindings.vue
│           │   └── Logs.vue
│           └── layouts/
│               └── AdminLayout.vue
│
├── Makefile                    # 构建命令
└── DESIGN.md                   # 本文档
```

---

## 三、共享协议库 (protocol/)

### 3.1 Cargo.toml

```toml
[package]
name = "hwt-protocol"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rsa = { version = "0.9", features = ["sha2"] }
aes-gcm = "0.10"
rand = "0.8"
base64 = "0.22"
```

### 3.2 消息类型 (lib.rs)

客户端 ↔ 网维服务器之间的 TCP 消息，全部为 JSON 序列化后加密传输。

```rust
// === 握手阶段 (明文 JSON, 仅握手阶段使用) ===

// 客户端 → 服务器: 请求握手
{ "type": "handshake" }

// 服务器 → 客户端: 返回 RSA 公钥 (PEM 格式)
{
  "type": "handshake_response",
  "public_key": "-----BEGIN PUBLIC KEY-----\nMIIBI..."
}

// 客户端 → 服务器: 用服务器 RSA 公钥加密的 AES 会话密钥
{
  "type": "key_exchange",
  "encrypted_key": "<base64 编码的 RSA 加密数据>"
}

// 服务器 → 客户端: 确认 (此消息开始用 AES 加密)
{ "type": "key_exchange_ok" }

// === 业务阶段 (AES-256-GCM 加密后传输) ===

// 客户端 → 服务器: 请求授权
{
  "type": "auth_request",
  "client_id": "PC-HOSTNAME-01",     // 工作站主机名
  "client_mac": "AA:BB:CC:DD:EE:FF"  // 可选, 用于日志
}

// 服务器 → 客户端: 授权结果
{
  "type": "auth_response",
  "authorized": true,                 // true=已授权, false=拒绝
  "message": "授权成功",               // 人类可读消息
  "server_time": 1711612800           // 服务器 Unix 时间戳
}

// 客户端 → 服务器: 心跳 (每 5 分钟)
{ "type": "heartbeat" }

// 服务器 → 客户端: 心跳回复
{ "type": "heartbeat_ack" }
```

Rust 类型定义:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    // 握手阶段
    #[serde(rename = "handshake")]
    Handshake,

    #[serde(rename = "handshake_response")]
    HandshakeResponse { public_key: String },

    #[serde(rename = "key_exchange")]
    KeyExchange { encrypted_key: String },

    #[serde(rename = "key_exchange_ok")]
    KeyExchangeOk,

    // 业务阶段 (加密传输)
    #[serde(rename = "auth_request")]
    AuthRequest {
        client_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_mac: Option<String>,
    },

    #[serde(rename = "auth_response")]
    AuthResponse {
        authorized: bool,
        message: String,
        server_time: u64,
    },

    #[serde(rename = "heartbeat")]
    Heartbeat,

    #[serde(rename = "heartbeat_ack")]
    HeartbeatAck,
}
```

### 3.3 TCP 帧格式 (frame.rs)

所有 TCP 通信使用长度前缀帧。握手阶段为明文帧，握手完成后为加密帧。

**明文帧** (握手阶段):
```
[4 bytes: payload_len (big-endian u32)] [payload_len bytes: JSON UTF-8]
```

**加密帧** (业务阶段):
```
[4 bytes: total_len (big-endian u32)] [12 bytes: IV/Nonce] [N bytes: 密文] [16 bytes: GCM TAG]
total_len = 12 + N + 16
密文内容 = AES-256-GCM-Encrypt(session_key, iv, JSON UTF-8 明文)
```

frame.rs 需实现:
```rust
// 从 TcpStream 读取一帧
pub async fn read_frame(stream: &mut TcpStream) -> Result<Vec<u8>>;

// 写入一帧 (自动加长度前缀)
pub async fn write_frame(stream: &mut TcpStream, data: &[u8]) -> Result<()>;

// 加密并写入
pub async fn write_encrypted(stream: &mut TcpStream, key: &[u8; 32], msg: &Message) -> Result<()>;

// 读取并解密
pub async fn read_encrypted(stream: &mut TcpStream, key: &[u8; 32]) -> Result<Message>;
```

### 3.4 加密封装 (crypto.rs)

```rust
/// 生成 RSA 2048-bit 密钥对
pub fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey);

/// RSA 公钥 → PEM 字符串
pub fn public_key_to_pem(key: &RsaPublicKey) -> String;

/// PEM 字符串 → RSA 公钥
pub fn public_key_from_pem(pem: &str) -> Result<RsaPublicKey>;

/// RSA 加密 (OAEP-SHA256), 输入 ≤ 190 bytes (2048-bit key)
pub fn rsa_encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>>;

/// RSA 解密
pub fn rsa_decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>>;

/// 生成随机 AES-256 密钥 (32 bytes)
pub fn generate_aes_key() -> [u8; 32];

/// AES-256-GCM 加密, 返回 (iv, ciphertext, tag) 拼接的 bytes
/// 格式: [12 bytes IV][N bytes ciphertext][16 bytes tag]
pub fn aes_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>>;

/// AES-256-GCM 解密, 输入为 aes_encrypt 的输出
pub fn aes_decrypt(key: &[u8; 32], encrypted: &[u8]) -> Result<Vec<u8>>;
```

---

## 四、客户端 EXE (client/)

### 4.1 Cargo.toml

```toml
[package]
name = "hwt-client"
version = "0.1.0"
edition = "2021"

[dependencies]
hwt-protocol = { path = "../protocol" }
windows-service = "0.7"
winreg = "0.52"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rsa = { version = "0.9", features = ["sha2"] }
aes-gcm = "0.10"
rand = "0.8"
base64 = "0.22"
obfstr = "0.4"
log = "0.4"
env_logger = "0.11"
gethostname = "0.4"

[dependencies.windows]
version = "0.58"
features = [
    "Win32_Devices_DeviceAndDriverInstallation",
    "Win32_Foundation",
    "Win32_NetworkManagement_IpHelper",
    "Win32_Networking_WinSock",
]

[profile.release]
lto = true
strip = true
opt-level = "z"
panic = "abort"
```

### 4.2 main.rs — 入口

```
用法:
  client.exe install     注册为 Windows 服务 "HwtCleanupService" 并启动
  client.exe uninstall   停止并删除服务
  client.exe status      查询服务状态
  client.exe run         前台运行 (调试用, 不注册服务)

无参数时: 由 Windows SCM 调用, 执行服务入口
```

核心逻辑:
```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("install")   => service::install(),
        Some("uninstall") => service::uninstall(),
        Some("status")    => service::status(),
        Some("run")       => run_foreground(),   // 调试模式
        _                 => service::dispatch(), // SCM 调用
    }
}
```

### 4.3 service.rs — Windows Service

服务名: `HwtCleanupService`
显示名: `HWT Device Cleanup Service`

**注册服务** (`install`):
```rust
// 使用 windows-service crate
// 1. OpenSCManager(SC_MANAGER_CREATE_SERVICE)
// 2. CreateServiceW(
//      service_name: "HwtCleanupService",
//      display_name: "HWT Device Cleanup Service",
//      service_type: SERVICE_WIN32_OWN_PROCESS,
//      start_type: SERVICE_AUTO_START,      // 开机自启
//      binary_path: 当前 exe 的绝对路径,
//      account: LocalSystem,                // SYSTEM 权限
//    )
// 3. StartServiceW() 立即启动
```

**服务主循环** (`dispatch` → `service_main`):
```rust
fn service_main(_args: Vec<OsString>) {
    // 1. 注册服务控制处理器 (处理 Stop, Shutdown)
    // 2. 设置状态为 Running
    // 3. 进入 tokio runtime
    // 4. 主循环:
    loop {
        match run_cleanup_cycle().await {
            Ok(_) => log::info!("清理完成"),
            Err(e) => log::error!("清理失败: {}", e),
        }
        // 等待 60 秒后重试 (如果首次连接失败) 或等待下次重启
        // 收到 Stop 信号时退出循环
        tokio::select! {
            _ = stop_signal.recv() => break,
            _ = tokio::time::sleep(Duration::from_secs(60)) => continue,
        }
    }
    // 5. 设置状态为 Stopped
}
```

### 4.4 scanner.rs — 内网端口扫描

```rust
const SCAN_PORT: u16 = 19800;
const SCAN_TIMEOUT: Duration = Duration::from_millis(200);  // 每个 IP 超时
const SCAN_CONCURRENCY: usize = 64;                         // 并发数

/// 扫描本机所在网段, 返回第一个响应的网维服务器 IP
pub async fn find_server() -> Result<IpAddr> {
    // 1. 获取本机所有网卡 IP 和子网掩码
    //    Windows API: GetAdaptersAddresses() 或 GetIpAddrTable()
    //    过滤: 只保留 IPv4, 排除 127.x.x.x 和 169.254.x.x
    //
    // 2. 对每个网段, 计算 IP 范围
    //    例: 本机 192.168.1.100/24 → 扫描 192.168.1.1 ~ 192.168.1.254
    //    计算方法: network = ip & mask, broadcast = network | !mask
    //    遍历 network+1 到 broadcast-1
    //
    // 3. 并发 TCP connect (tokio::spawn, 信号量控制并发)
    //    对每个 IP: TcpStream::connect_timeout((ip, SCAN_PORT), SCAN_TIMEOUT)
    //    连接成功后发送 handshake 消息验证是否为网维服务器
    //    验证方法: 发送 {"type":"handshake"}, 收到 {"type":"handshake_response",...}
    //
    // 4. 找到第一个有效服务器后取消其余扫描任务, 返回 IP
    //
    // 5. 扫描完所有网段仍未找到 → 返回错误, 外层等待后重试
}
```

### 4.5 registry.rs — 显示器注册表清理

```rust
use winreg::enums::*;
use winreg::RegKey;

const DISPLAY_KEY_PATH: &str = r"SYSTEM\CurrentControlSet\Enum\DISPLAY";

/// 删除 HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY 下的所有子键
/// 需要 SYSTEM 权限 (Windows Service 天然满足)
pub fn clean_display_registry() -> Result<u32> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // 1. 打开 DISPLAY 键
    //    hklm.open_subkey_with_flags(DISPLAY_KEY_PATH, KEY_READ | KEY_WRITE)
    //    如果不存在 → 返回 Ok(0), 无需清理

    // 2. 枚举所有子键名
    //    display_key.enum_keys().collect::<Vec<_>>()
    //    子键结构示例:
    //      DISPLAY\
    //        Default_Monitor\           ← 一级子键 (显示器型号)
    //          4&12345678&0&UID0\       ← 二级子键 (实例)
    //            Device Parameters\     ← 三级子键
    //            Properties\            ← 三级子键

    // 3. 递归删除每个一级子键
    //    对每个子键调用 hklm.delete_subkey_all(format!("{DISPLAY_KEY_PATH}\\{name}"))
    //    delete_subkey_all 会递归删除所有子键和值

    // 4. 返回删除的子键数量
    //    注意: 只删子键, 不删 DISPLAY 键本身 (Windows 需要这个空键存在)
}
```

**注意事项**:
- `HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY` 的子键有安全描述符保护
- SYSTEM 账户有完全控制权限, 普通 Administrator 可能需要先 take ownership
- 用 `RegKey::open_subkey_with_flags()` 时使用 `KEY_ALL_ACCESS`
- 如果某个子键删除失败 (可能被占用), 记录日志并继续删除其他子键

### 4.6 cleanup.rs — 幽灵设备清理 (Setup API)

基于 DeviceCleanup V1.3.1 的分析实现, 使用 Windows Setup API 正规移除幽灵设备。

```rust
use windows::Win32::Devices::DeviceAndDriverInstallation::*;

/// 需要清理的设备类 GUID
const DISPLAY_CLASS_GUID: &str = "{4d36e968-e325-11ce-bfc1-08002be10318}";
// 可扩展:
// const PCI_CLASS_GUID: &str = "{4d36e97d-e325-11ce-bfc1-08002be10318}";
// const MONITOR_CLASS_GUID: &str = "{4d36e96e-e325-11ce-bfc1-08002be10318}";

/// 清理幽灵设备, 返回 (扫描数, 清理数)
pub fn cleanup_phantom_devices() -> Result<(u32, u32)> {
    let mut scanned = 0u32;
    let mut removed = 0u32;

    // === 第 1 步: 枚举所有设备 ===
    // hDevInfo = SetupDiGetClassDevsW(
    //     ptr::null(),            // ClassGuid = NULL (所有类)
    //     ptr::null(),            // Enumerator = NULL
    //     HWND::default(),        // hwndParent = NULL
    //     DIGCF_ALLCLASSES        // Flags = 0x04 (所有设备类)
    // )
    // 如果返回 INVALID_HANDLE_VALUE → 失败

    // === 第 2 步: 遍历每个设备 ===
    // let mut dev_info_data = SP_DEVINFO_DATA {
    //     cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
    //     ..Default::default()
    // };
    // let mut index = 0;
    // while SetupDiEnumDeviceInfo(hDevInfo, index, &mut dev_info_data) == TRUE {
    //     index += 1;
    //     scanned += 1;
    //
    //     // === 第 3 步: 检查是否为幽灵设备 ===
    //     let mut status: u32 = 0;
    //     let mut problem: u32 = 0;
    //     let ret = CM_Get_DevNode_Status(
    //         &mut status,
    //         &mut problem,
    //         dev_info_data.DevInst,
    //         0
    //     );
    //
    //     // 判断幽灵设备的条件:
    //     // - CM_Get_DevNode_Status 返回 CR_NO_SUCH_DEVINST (13)
    //     //   → 设备不存在, 强制 problem = 45
    //     // - problem == 45 (CM_PROB_PHANTOM)
    //     //   → "设备当前仅存在于注册表中"
    //     //
    //     // 非幽灵设备 → continue
    //     let is_phantom = (ret == CR_NO_SUCH_DEVINST) || (problem == 45);
    //     if !is_phantom { continue; }
    //
    //     // === 第 4 步: 过滤目标设备类 ===
    //     // 获取设备类 GUID
    //     // CM_Get_DevNode_Registry_PropertyW(
    //     //     dev_info_data.DevInst,
    //     //     CM_DRP_CLASSGUID,   // 属性 9: 设备类 GUID
    //     //     ..., &buf, &buf_size, 0
    //     // )
    //     // 比较 GUID 是否在目标清理列表中
    //     // 如果不是目标类 → continue
    //
    //     // === 第 5 步: 获取设备 ID (用于日志) ===
    //     // let mut device_id = [0u16; 260];
    //     // CM_Get_Device_IDW(dev_info_data.DevInst, &mut device_id, 260, 0);
    //     // 日志记录: "正在移除: PCI\VEN_XXXX&DEV_YYYY\..."
    //
    //     // === 第 6 步: 执行移除 ===
    //     // 关键: 必须用 SP_REMOVEDEVICE_PARAMS (16 bytes), 不能只传 SP_CLASSINSTALL_HEADER (8 bytes)!
    //     // 只传 8 bytes 会导致 ERROR_INVALID_PARAMETER (87)
    //     // struct SP_REMOVEDEVICE_PARAMS {
    //     //     ClassInstallHeader: SP_CLASSINSTALL_HEADER,  // 8 bytes
    //     //     Scope: u32,      // DI_REMOVEDEVICE_GLOBAL = 1
    //     //     HwProfile: u32,  // 0
    //     // }
    //     //
    //     // let params = SP_REMOVEDEVICE_PARAMS {
    //     //     class_install_header: SP_CLASSINSTALL_HEADER {
    //     //         cbSize: 8,  // sizeof(SP_CLASSINSTALL_HEADER)
    //     //         InstallFunction: DIF_REMOVE,  // 0x05
    //     //     },
    //     //     scope: 1,       // DI_REMOVEDEVICE_GLOBAL
    //     //     hw_profile: 0,
    //     // };
    //     //
    //     // SetupDiSetClassInstallParamsW(
    //     //     hDevInfo,
    //     //     &dev_info_data,
    //     //     &params.class_install_header,
    //     //     16  // sizeof(SP_REMOVEDEVICE_PARAMS), NOT 8!
    //     // );
    //     //
    //     // SetupDiCallClassInstaller(DIF_REMOVE, hDevInfo, &dev_info_data);
    //
    //     // === 第 7 步: 验证移除结果 ===
    //     // if success == TRUE {
    //     //     // 二次验证: 尝试再次定位设备
    //     //     let mut verify_inst: u32 = 0;
    //     //     if CM_Locate_DevNodeW(&mut verify_inst, device_id.as_ptr(), 0) != CR_SUCCESS {
    //     //         // 设备已成功移除
    //     //         removed += 1;
    //     //         log::info!("已移除: {}", device_id_string);
    //     //     } else {
    //     //         log::warn!("移除后设备仍存在: {}", device_id_string);
    //     //     }
    //     // } else {
    //     //     let err = GetLastError();
    //     //     log::error!("移除失败: {}, 错误码: {}", device_id_string, err);
    //     // }
    // }

    // === 第 8 步: 清理 ===
    // SetupDiDestroyDeviceInfoList(hDevInfo);

    Ok((scanned, removed))
}
```

**Windows API 调用参数速查**:

| API | 参数 | 值 |
|-----|------|-----|
| `SetupDiGetClassDevsW` | ClassGuid | NULL (所有类) |
| | Enumerator | NULL |
| | Flags | `DIGCF_ALLCLASSES` (0x04) |
| `CM_Get_DevNode_Status` | 返回值 13 | `CR_NO_SUCH_DEVINST` |
| | problem 45 | `CM_PROB_PHANTOM` |
| `CM_Get_DevNode_Registry_PropertyW` | 属性 9 | `CM_DRP_CLASSGUID` |
| `SetupDiCallClassInstaller` | InstallFunction | `DIF_REMOVE` (0x05) |
| `SP_CLASSINSTALL_HEADER.cbSize` | | 8 bytes (u32 + u32) |
| `SP_REMOVEDEVICE_PARAMS` 总大小 | | **16 bytes** (header 8 + scope 4 + hwprofile 4) |
| `SetupDiSetClassInstallParamsW` size 参数 | | **必须传 16**, 传 8 会 ERROR_INVALID_PARAMETER(87) |
| `CM_Get_DevNode_Status` status 类型 | | `CM_DEVNODE_STATUS_FLAGS` (不是 u32) |
| `CM_Get_DevNode_Status` problem 类型 | | `CM_PROB` (不是 u32) |
| `CM_Locate_DevNodeW` flags 类型 | | `CM_LOCATE_DEVNODE_FLAGS(0)` |

### 4.7 主清理流程 (run_cleanup_cycle)

```rust
async fn run_cleanup_cycle() -> Result<()> {
    // 1. 扫描内网找网维服务器
    let server_ip = scanner::find_server().await?;
    log::info!("找到网维服务器: {}", server_ip);

    // 2. TCP 连接
    let mut stream = TcpStream::connect((server_ip, 19800)).await?;

    // 3. RSA 握手
    //    a. 发送 {"type":"handshake"} (明文帧)
    //    b. 收到 {"type":"handshake_response","public_key":"..."} (明文帧)
    //    c. 生成随机 AES-256 session_key (32 bytes)
    //    d. RSA 加密 session_key
    //    e. 发送 {"type":"key_exchange","encrypted_key":"<base64>"} (明文帧)
    //    f. 收到 {"type":"key_exchange_ok"} (加密帧, 用 session_key 解密验证)

    // 4. 请求授权 (AES 加密帧)
    //    发送: {"type":"auth_request","client_id":"<hostname>"}
    //    收到: {"type":"auth_response","authorized":true/false,...}

    // 5. 如果 authorized == false → 记录日志, 返回错误, 外层重试

    // 6. 授权通过, 执行清理
    //    a. registry::clean_display_registry()   → 删 DISPLAY 注册表
    //    b. cleanup::cleanup_phantom_devices()    → Setup API 清理幽灵设备
    log::info!("显示器注册表清理: 删除 {} 项", display_count);
    log::info!("幽灵设备清理: 扫描 {}, 移除 {}", scanned, removed);

    // 7. 保持连接, 定期心跳
    //    每 5 分钟发送 {"type":"heartbeat"}, 收 {"type":"heartbeat_ack"}
    //    连接断开 → 外层重新扫描连接
}
```

### 4.8 build.rs — Windows Manifest

```rust
// 嵌入 manifest 请求管理员权限 (仅 install/uninstall 命令行模式需要)
// Windows Service 以 SYSTEM 运行时不需要 manifest 提权

fn main() {
    // 仅 Windows 目标时编译
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest(r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
              <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                  <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator"/>
                  </requestedPrivileges>
                </security>
              </trustInfo>
            </assembly>
        "#);
        res.compile().unwrap();
    }
}
```

需要在 Cargo.toml 中添加:
```toml
[build-dependencies]
winres = "0.1"
```

---

## 五、网维服务器 (server/)

### 5.1 Cargo.toml

```toml
[package]
name = "hwt-server"
version = "0.1.0"
edition = "2021"

[dependencies]
hwt-protocol = { path = "../protocol" }
eframe = "0.28"
egui = "0.28"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rsa = { version = "0.9", features = ["sha2"] }
aes-gcm = "0.10"
rand = "0.8"
base64 = "0.22"
sha2 = "0.10"
obfstr = "0.4"
log = "0.4"
env_logger = "0.11"
chrono = "0.4"

[target.'cfg(windows)'.dependencies]
wmi = "0.13"

[profile.release]
lto = true
strip = true
opt-level = "z"
panic = "abort"
```

### 5.2 machine.rs — 机器码采集

```rust
use sha2::{Sha256, Digest};
use wmi::{COMLibrary, WMIConnection};

/// 采集本机硬件信息, 生成机器码 (SHA256 hex string, 64 chars)
pub fn get_machine_code() -> Result<String> {
    // 1. 初始化 COM
    let com = COMLibrary::new()?;
    let wmi = WMIConnection::new(com)?;

    // 2. 查询 CPU ID
    //    WMI 查询: "SELECT ProcessorId FROM Win32_Processor"
    //    取第一条结果的 ProcessorId 字段
    //    示例值: "BFEBFBFF000906EA"

    // 3. 查询主板序列号
    //    WMI 查询: "SELECT SerialNumber FROM Win32_BaseBoard"
    //    示例值: "/7PRGQ2/CNWS200800A6/"

    // 4. 查询硬盘序列号
    //    WMI 查询: "SELECT SerialNumber FROM Win32_DiskDrive WHERE Index=0"
    //    取第一块硬盘
    //    示例值: "S3YKNX0J123456"

    // 5. 拼接并哈希
    //    let raw = format!("{}-{}-{}", cpu_id, baseboard_sn, disk_sn);
    //    let hash = Sha256::digest(raw.as_bytes());
    //    let machine_code = hex::encode(hash);  // 64 字符 hex
    //    return Ok(machine_code);
}
```

WMI 查询的 Rust 结构体:
```rust
#[derive(Deserialize)]
struct Win32Processor {
    ProcessorId: String,
}
#[derive(Deserialize)]
struct Win32BaseBoard {
    SerialNumber: String,
}
#[derive(Deserialize)]
struct Win32DiskDrive {
    SerialNumber: String,
}
```

### 5.3 auth.rs — 云端验证

```rust
const CLOUD_BASE_URL: &str = "https://你的域名.com";  // 可配置

/// 与云端进行 RSA 握手, 获取 AES 会话密钥
pub async fn cloud_handshake(client: &reqwest::Client) -> Result<(String, [u8; 32])> {
    // 1. GET {CLOUD_BASE_URL}/api/auth/public-key
    //    响应: { "public_key": "-----BEGIN PUBLIC KEY-----\n..." }
    //    解析出 RSA 公钥

    // 2. 生成随机 AES-256 session_key (32 bytes)

    // 3. RSA 加密 session_key
    //    encrypted = rsa_encrypt(cloud_public_key, &session_key)
    //    encoded = base64_encode(encrypted)

    // 4. POST {CLOUD_BASE_URL}/api/verify/handshake
    //    请求体: { "encrypted_session_key": "<base64>" }
    //    响应:   { "session_id": "uuid-xxxx" }

    // 返回 (session_id, session_key)
}

/// 验证账号 + 机器码 (AES 加密通道)
pub async fn cloud_verify(
    client: &reqwest::Client,
    session_id: &str,
    session_key: &[u8; 32],
    account: &str,
    password: &str,
    machine_code: &str,
) -> Result<VerifyResponse> {
    // 1. 构造明文 JSON
    //    { "account": "xxx", "password": "yyy", "machine_code": "abc123..." }

    // 2. AES-256-GCM 加密
    //    encrypted = aes_encrypt(session_key, json_bytes)
    //    iv = encrypted[0..12], payload = encrypted[12..n-16], tag = encrypted[n-16..n]

    // 3. POST {CLOUD_BASE_URL}/api/verify
    //    请求体: {
    //      "session_id": "uuid-xxxx",
    //      "iv": "<base64>",
    //      "payload": "<base64>",
    //      "tag": "<base64>"
    //    }

    // 4. 响应也是 AES 加密的:
    //    { "iv": "...", "payload": "...", "tag": "..." }
    //    解密后得到:
    //    {
    //      "authorized": true,
    //      "license_type": "yearly",
    //      "expire_at": "2027-03-28T00:00:00Z",  // permanent 时为 null
    //      "message": "授权验证成功"
    //    }
}

#[derive(Deserialize)]
pub struct VerifyResponse {
    pub authorized: bool,
    pub license_type: String,   // "monthly" | "yearly" | "permanent"
    pub expire_at: Option<String>,
    pub message: String,
}
```

### 5.4 listener.rs — TCP 监听 (接受客户端)

```rust
const LISTEN_PORT: u16 = 19800;

/// 启动 TCP 监听, 处理客户端连接
pub async fn start_listener(
    authorized: Arc<AtomicBool>,       // 当前是否已授权
    clients: Arc<Mutex<Vec<ClientInfo>>>,  // 在线客户端列表 (给 GUI 用)
) -> Result<()> {
    // 1. 程序启动时生成 RSA 密钥对 (每次启动重新生成)
    let (rsa_private, rsa_public) = generate_rsa_keypair();
    let public_key_pem = public_key_to_pem(&rsa_public);

    // 2. 绑定 0.0.0.0:19800
    let listener = TcpListener::bind(("0.0.0.0", LISTEN_PORT)).await?;

    // 3. accept 循环
    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let auth = authorized.clone();
        let key = rsa_private.clone();
        let pem = public_key_pem.clone();
        let clients_list = clients.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, peer_addr, auth, key, pem, clients_list).await {
                log::error!("客户端 {} 处理错误: {}", peer_addr, e);
            }
        });
    }
}

async fn handle_client(
    mut stream: TcpStream,
    peer_addr: SocketAddr,
    authorized: Arc<AtomicBool>,
    rsa_private: RsaPrivateKey,
    public_key_pem: String,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
) -> Result<()> {
    // === RSA 握手 ===

    // 1. 读取客户端 handshake (明文帧)
    //    期望: {"type":"handshake"}

    // 2. 返回 RSA 公钥 (明文帧)
    //    发送: {"type":"handshake_response","public_key":"<PEM>"}

    // 3. 读取客户端 key_exchange (明文帧)
    //    期望: {"type":"key_exchange","encrypted_key":"<base64>"}
    //    RSA 解密得到 session_key (32 bytes)

    // 4. 发送 key_exchange_ok (加密帧, 用 session_key)
    //    发送: 加密({"type":"key_exchange_ok"})

    // === 业务循环 ===
    loop {
        let msg = read_encrypted(&mut stream, &session_key).await?;
        match msg {
            Message::AuthRequest { client_id, client_mac } => {
                // 检查当前授权状态
                let is_auth = authorized.load(Ordering::Relaxed);
                // 发送授权结果
                let resp = Message::AuthResponse {
                    authorized: is_auth,
                    message: if is_auth { "授权成功".into() } else { "服务器未授权".into() },
                    server_time: now_unix_timestamp(),
                };
                write_encrypted(&mut stream, &session_key, &resp).await?;

                // 添加到在线列表
                if is_auth {
                    clients.lock().await.push(ClientInfo {
                        ip: peer_addr.ip(),
                        client_id,
                        connected_at: chrono::Local::now(),
                    });
                }
            }
            Message::Heartbeat => {
                write_encrypted(&mut stream, &session_key, &Message::HeartbeatAck).await?;
            }
            _ => break, // 未知消息, 断开
        }
    }

    // 从在线列表移除
    clients.lock().await.retain(|c| c.ip != peer_addr.ip());
    Ok(())
}

pub struct ClientInfo {
    pub ip: IpAddr,
    pub client_id: String,
    pub connected_at: chrono::DateTime<chrono::Local>,
}
```

### 5.5 gui.rs — egui 界面

两个页面: 登录页 + 主页

```rust
struct App {
    // 状态
    page: Page,               // Login | Main
    username: String,
    password: String,
    login_error: Option<String>,

    // 授权信息
    authorized: Arc<AtomicBool>,
    license_type: String,
    expire_at: Option<String>,
    machine_code: String,

    // 在线客户端
    clients: Arc<Mutex<Vec<ClientInfo>>>,

    // 日志
    log_messages: Arc<Mutex<Vec<String>>>,

    // 后台任务通道
    auth_tx: mpsc::Sender<AuthCommand>,
}

enum Page { Login, Main }

// === 登录页 ===
// ┌────────────────────────────┐
// │       网维服务器 V1.0        │
// │                            │
// │  账号: [_______________]   │
// │  密码: [_______________]   │
// │                            │
// │  机器码: a1b2c3d4e5...     │
// │                            │
// │       [  登  录  ]          │
// │                            │
// │  ❌ 登录失败: 账号密码错误   │ (可选错误提示)
// └────────────────────────────┘

// === 主页 ===
// ┌────────────────────────────┐
// │  授权状态: ✅ 已授权         │
// │  授权类型: 年付              │
// │  到期时间: 2027-03-28       │
// │  机器码: a1b2c3d4e5f6...   │
// │  最后验证: 2026-03-28 10:30 │
// ├────────────────────────────┤
// │  在线客户端 (3台)            │
// │  ┌──────┬──────┬─────────┐ │
// │  │ IP   │ 主机名│ 连接时间 │ │
// │  ├──────┼──────┼─────────┤ │
// │  │.101  │PC-01 │10:30:15 │ │
// │  │.102  │PC-02 │10:31:22 │ │
// │  │.103  │PC-03 │10:32:08 │ │
// │  └──────┴──────┴─────────┘ │
// ├────────────────────────────┤
// │  日志:                      │
// │  [10:30] 云端验证成功        │
// │  [10:30] TCP 监听已启动      │
// │  [10:31] 客户端 PC-01 连接   │
// └────────────────────────────┘
```

**登录流程** (点击登录按钮后):
1. GUI 线程通过 channel 发送 `AuthCommand::Login { username, password }` 到后台
2. 后台 tokio 任务:
   a. 采集机器码 (`machine::get_machine_code()`)
   b. 与云端 RSA 握手 (`auth::cloud_handshake()`)
   c. 加密验证 (`auth::cloud_verify()`)
   d. 如果成功: 设置 `authorized = true`, 启动 TCP 监听
   e. 通过 channel 返回结果给 GUI
3. GUI 收到结果后切换到主页或显示错误

**定时重验** (后台任务):
- 每 60 分钟向云端重新验证一次
- 验证失败 → `authorized = false`, GUI 显示 "授权已过期"
- 所有在线客户端下次心跳时会收到 `authorized: false`

### 5.6 main.rs

```rust
fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 640.0])
            .with_title("网维服务器"),
        ..Default::default()
    };

    eframe::run_native(
        "hwt-server",
        options,
        Box::new(|_cc| Box::new(App::new())),
    )
}
```

---

## 六、云服务器 (cloud/)

### 6.1 后端 (cloud/backend/)

#### requirements.txt

```
fastapi==0.115.*
uvicorn[standard]==0.32.*
sqlalchemy==2.0.*
alembic==1.13.*
pydantic==2.9.*
python-jose[cryptography]==3.3.*
passlib[bcrypt]==1.7.*
cryptography==43.*
python-multipart==0.0.*
```

#### app/config.py

```python
import os

DATABASE_URL = os.getenv("DATABASE_URL", "sqlite:///./hwt.db")
# PostgreSQL: "postgresql://user:pass@host/dbname"

RSA_KEY_DIR = os.getenv("RSA_KEY_DIR", "./keys")
# 启动时如果不存在, 自动生成 rsa_private.pem + rsa_public.pem

JWT_SECRET = os.getenv("JWT_SECRET", "change-me-in-production")
JWT_ALGORITHM = "HS256"
JWT_EXPIRE_MINUTES = 60 * 24  # 24 小时

# AES 会话缓存 TTL
SESSION_TTL_HOURS = 24
```

#### app/models.py

```python
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
```

#### app/crypto.py

```python
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
import os, base64, json

class CryptoManager:
    def __init__(self, key_dir: str):
        """加载或生成 RSA 密钥对"""
        # 私钥路径: {key_dir}/rsa_private.pem
        # 公钥路径: {key_dir}/rsa_public.pem
        # 如果文件不存在 → 生成 2048-bit 密钥对并保存

    def get_public_key_pem(self) -> str:
        """返回 PEM 格式公钥字符串"""

    def rsa_decrypt(self, ciphertext_b64: str) -> bytes:
        """RSA OAEP-SHA256 解密 (base64 输入)"""
        ciphertext = base64.b64decode(ciphertext_b64)
        return self.private_key.decrypt(
            ciphertext,
            padding.OAEP(
                mgf=padding.MGF1(algorithm=hashes.SHA256()),
                algorithm=hashes.SHA256(),
                label=None
            )
        )

    @staticmethod
    def aes_encrypt(key: bytes, plaintext: bytes) -> tuple[str, str, str]:
        """AES-256-GCM 加密, 返回 (iv_b64, ciphertext_b64, tag_b64)"""
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
        """AES-256-GCM 解密"""
        iv = base64.b64decode(iv_b64)
        ct = base64.b64decode(payload_b64)
        tag = base64.b64decode(tag_b64)
        aesgcm = AESGCM(key)
        return aesgcm.decrypt(iv, ct + tag, None)

# 会话存储 (生产环境可换 Redis)
# Dict[session_id -> { "key": bytes, "created_at": datetime }]
sessions: dict = {}
```

#### app/api/auth.py — 注册 / 登录

```python
# GET /api/auth/public-key
# 响应:
{
    "public_key": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBg..."
}

# POST /api/auth/register
# 请求 (密码用 RSA 公钥加密):
{
    "username": "shopowner1",
    "password_encrypted": "<base64 RSA 加密的密码>",
    "email": "owner@example.com"         # 可选
}
# 响应:
{
    "id": 1,
    "username": "shopowner1",
    "status": "pending",                 # 待管理员审核
    "message": "注册成功, 请等待管理员审核激活"
}
# 服务端逻辑:
#   1. RSA 解密 password_encrypted → 明文密码
#   2. passlib.bcrypt 哈希密码
#   3. 创建 User (status=pending)
#   4. 记录 AuthLog (action=register, result=success)

# POST /api/auth/login
# 请求:
{
    "username": "admin",
    "password_encrypted": "<base64 RSA 加密的密码>"
}
# 响应 (成功):
{
    "access_token": "eyJ...",            # JWT token
    "token_type": "bearer",
    "user": {
        "id": 1,
        "username": "admin",
        "role": "admin",
        "status": "active"
    }
}
# 响应 (失败): HTTP 401
{
    "detail": "用户名或密码错误"          # 或 "账号未激活"
}
```

#### app/api/verify.py — 网维服务器验证

```python
# POST /api/verify/handshake
# 请求:
{
    "encrypted_session_key": "<base64, RSA加密的32字节AES密钥>"
}
# 服务端逻辑:
#   1. RSA 解密 → 得到 32 字节 AES session_key
#   2. 生成 session_id (uuid4)
#   3. 存入 sessions dict: sessions[session_id] = {"key": session_key, "created_at": now}
# 响应:
{
    "session_id": "550e8400-e29b-41d4-a716-446655440000"
}

# POST /api/verify
# 请求 (AES 加密的):
{
    "session_id": "550e8400-...",
    "iv": "<base64, 12字节>",
    "payload": "<base64, AES密文>",
    "tag": "<base64, 16字节>"
}
# 服务端逻辑:
#   1. 根据 session_id 查找 session_key
#   2. AES-GCM 解密 → 得到明文 JSON:
#      { "account": "shopowner1", "password": "xxx", "machine_code": "a1b2c3..." }
#   3. 验证账号密码 (bcrypt verify)
#   4. 检查用户状态 == active
#   5. 检查授权:
#      - license_type 不为空
#      - 如果非 permanent, 检查 license_expire_at > now
#   6. 检查机器码绑定:
#      a. 查询 machine_bindings WHERE user_id=X AND status=active
#      b. 无记录 → 首次绑定: INSERT 新记录
#      c. 有记录且 machine_code 匹配 → 更新 last_verified_at
#      d. 有记录但 machine_code 不匹配 → 拒绝 (需管理员解绑)
#   7. 记录 AuthLog
#   8. 构造明文响应 → AES 加密 → 返回
#
# 解密后的明文响应:
{
    "authorized": true,
    "license_type": "yearly",
    "expire_at": "2027-03-28T00:00:00Z",  // permanent 时为 null
    "message": "验证成功"
}
# 实际 HTTP 响应 (AES 加密):
{
    "iv": "<base64>",
    "payload": "<base64>",
    "tag": "<base64>"
}

# 失败情况的明文响应:
# 账号密码错误:   {"authorized":false, "message":"账号或密码错误", ...}
# 账号未激活:     {"authorized":false, "message":"账号未激活, 请联系管理员", ...}
# 授权过期:       {"authorized":false, "message":"授权已过期, 请续费", ...}
# 机器码不匹配:   {"authorized":false, "message":"机器码不匹配, 请联系管理员解绑", ...}
```

#### app/api/admin.py — 管理后台 API

所有接口需要 JWT 认证, 且 role == admin。

```python
# GET /api/admin/users?page=1&size=20&status=pending
# 响应:
{
    "total": 42,
    "items": [
        {
            "id": 1,
            "username": "shopowner1",
            "email": "owner@example.com",
            "role": "user",
            "status": "pending",
            "license_type": null,
            "license_expire_at": null,
            "created_at": "2026-03-28T10:00:00Z"
        }
    ]
}

# PATCH /api/admin/users/{id}
# 请求 (部分更新, 只传需要改的字段):
{
    "status": "active",                    # 审核通过
    "license_type": "yearly",              # 设置授权类型
    "license_expire_at": "2027-03-28T00:00:00Z"  # 设置到期时间
}
# 响应: 更新后的完整用户对象

# GET /api/admin/bindings?page=1&size=20&user_id=1
# 响应:
{
    "total": 1,
    "items": [
        {
            "id": 1,
            "user_id": 1,
            "username": "shopowner1",
            "machine_code": "a1b2c3d4e5f6...",
            "bound_at": "2026-03-28T10:30:00Z",
            "last_verified_at": "2026-03-28T15:00:00Z",
            "status": "active"
        }
    ]
}

# DELETE /api/admin/bindings/{id}
# 逻辑: 将 status 改为 "unbound" (软删除)
# 响应: { "message": "解绑成功" }
# 效果: 该用户下次登录网维服务器时会重新绑定新机器码

# GET /api/admin/logs?page=1&size=50&username=shopowner1&action=verify
# 响应:
{
    "total": 100,
    "items": [
        {
            "id": 1,
            "user_id": 1,
            "username": "shopowner1",
            "machine_code": "a1b2c3...",
            "action": "verify",
            "ip_address": "123.45.67.89",
            "result": "success",
            "detail": null,
            "created_at": "2026-03-28T10:30:00Z"
        }
    ]
}
```

#### app/main.py

```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.api import auth, verify, admin
from app.database import engine, Base
from app.crypto import CryptoManager
from app.config import RSA_KEY_DIR

# 创建表
Base.metadata.create_all(bind=engine)

app = FastAPI(title="HWT 网维系统云端", version="1.0.0")

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

# 启动事件: 确保至少有一个 admin 用户
@app.on_event("startup")
async def init_admin():
    # 检查是否有 admin 用户, 没有则创建默认:
    # username: admin, password: admin123 (首次登录后应修改)
```

### 6.2 前端 (cloud/frontend/)

#### vite.config.ts

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { obfuscator } from 'rollup-obfuscator'  // JS 混淆

export default defineConfig({
  plugins: [
    vue(),
    // 生产构建时启用 JS 混淆
    process.env.NODE_ENV === 'production' && obfuscator({
      compact: true,
      controlFlowFlattening: true,          // 控制流扁平化
      controlFlowFlatteningThreshold: 0.75,
      deadCodeInjection: true,              // 死代码注入
      deadCodeInjectionThreshold: 0.4,
      debugProtection: true,                // 禁用 debugger
      disableConsoleOutput: true,           // 禁用 console
      stringArray: true,                    // 字符串数组混淆
      stringArrayThreshold: 0.75,
      stringArrayEncoding: ['base64'],
    }),
  ].filter(Boolean),
  server: {
    proxy: {
      '/api': 'http://localhost:8000',      // 开发时代理到后端
    },
  },
})
```

#### src/api/crypto.ts — RSA 加密封装

```typescript
import JSEncrypt from 'jsencrypt'

let publicKey: string | null = null

// 从服务器获取 RSA 公钥 (缓存)
export async function getPublicKey(): Promise<string> {
  if (publicKey) return publicKey
  const resp = await fetch('/api/auth/public-key')
  const data = await resp.json()
  publicKey = data.public_key
  return publicKey!
}

// RSA 加密密码
export async function encryptPassword(password: string): Promise<string> {
  const pem = await getPublicKey()
  const encrypt = new JSEncrypt()
  encrypt.setPublicKey(pem)
  const encrypted = encrypt.encrypt(password)
  if (!encrypted) throw new Error('RSA 加密失败')
  return encrypted  // base64 字符串
}
```

#### src/api/auth.ts

```typescript
import request from './request'
import { encryptPassword } from './crypto'

export async function login(username: string, password: string) {
  const password_encrypted = await encryptPassword(password)
  return request.post('/api/auth/login', { username, password_encrypted })
}

export async function register(username: string, password: string, email?: string) {
  const password_encrypted = await encryptPassword(password)
  return request.post('/api/auth/register', { username, password_encrypted, email })
}
```

#### src/api/request.ts — axios 封装

```typescript
import axios from 'axios'

const request = axios.create({ baseURL: '/', timeout: 10000 })

// 请求拦截: 自动加 JWT token
request.interceptors.request.use(config => {
  const token = localStorage.getItem('token')
  if (token) config.headers.Authorization = `Bearer ${token}`
  return config
})

// 响应拦截: 401 跳转登录
request.interceptors.response.use(
  response => response.data,
  error => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

export default request
```

#### 前端页面说明

| 页面 | 路由 | 功能 |
|------|------|------|
| Login.vue | /login | 登录表单, RSA 加密密码, 存 JWT 到 localStorage |
| Dashboard.vue | / | 统计卡片: 总用户数、活跃用户、今日验证次数、在线绑定数 |
| Users.vue | /users | 表格: 用户列表, 搜索/筛选, 操作: 审核激活/禁用/设置授权类型+到期时间 |
| Bindings.vue | /bindings | 表格: 机器码绑定列表, 操作: 解绑 (确认弹窗) |
| Logs.vue | /logs | 表格: 验证日志, 按用户名/操作类型/时间筛选 |

---

## 七、加密常量汇总

| 参数 | 值 |
|------|-----|
| RSA 密钥长度 | 2048-bit |
| RSA Padding | OAEP-SHA256 |
| AES 算法 | AES-256-GCM |
| AES 密钥长度 | 256-bit (32 bytes) |
| AES IV/Nonce 长度 | 96-bit (12 bytes), 每次随机生成 |
| AES GCM TAG 长度 | 128-bit (16 bytes) |
| TCP 帧头 | 4 bytes big-endian u32 (payload 总长) |
| AES 会话有效期 | 24 小时, 过期重新握手 |
| 心跳间隔 | 5 分钟 |
| 云端重验间隔 | 60 分钟 |
| 扫描端口 | TCP 19800 |
| 扫描超时 | 200ms / IP |
| 扫描并发 | 64 |

---

## 八、构建与部署

### 8.1 Rust 交叉编译 (Linux → Windows)

```bash
# 安装工具链
rustup target add x86_64-pc-windows-gnu
apt install gcc-mingw-w64-x86-64

# Workspace Cargo.toml (项目根目录)
[workspace]
members = ["protocol", "client", "server"]

# 编译全部
cargo build --release --target x86_64-pc-windows-gnu

# 产出:
# target/x86_64-pc-windows-gnu/release/hwt-client.exe   (~2-4 MB)
# target/x86_64-pc-windows-gnu/release/hwt-server.exe   (~8-12 MB, 含 GUI)

# strip 符号 (进一步防逆向)
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/hwt-client.exe
x86_64-w64-mingw32-strip target/x86_64-pc-windows-gnu/release/hwt-server.exe
```

### 8.2 云端部署

```bash
# 后端
cd cloud/backend
pip install -r requirements.txt
# 开发:
uvicorn app.main:app --reload --host 0.0.0.0 --port 8000
# 生产:
uvicorn app.main:app --host 0.0.0.0 --port 8000 --workers 4

# 前端
cd cloud/frontend
npm install
npm run dev          # 开发 (Vite dev server, 代理 /api → localhost:8000)
npm run build        # 生产构建 (含 JS 混淆) → dist/
# 将 dist/ 部署到 nginx 或让 FastAPI 服务静态文件
```

### 8.3 Makefile

```makefile
TARGET = x86_64-pc-windows-gnu
STRIP = x86_64-w64-mingw32-strip

.PHONY: all client server cloud-backend cloud-frontend clean

all: client server

client:
	cargo build --release --target $(TARGET) -p hwt-client
	$(STRIP) target/$(TARGET)/release/hwt-client.exe

server:
	cargo build --release --target $(TARGET) -p hwt-server
	$(STRIP) target/$(TARGET)/release/hwt-server.exe

cloud-backend:
	cd cloud/backend && pip install -r requirements.txt

cloud-frontend:
	cd cloud/frontend && npm install && npm run build

clean:
	cargo clean
	rm -rf cloud/frontend/dist
```

---

## 九、实施顺序

| 阶段 | 内容 | 产出 |
|------|------|------|
| Phase 1 | protocol/ (消息类型 + crypto + frame) | 可编译的 lib crate |
| Phase 2 | cloud/backend/ (全部 API) | 可运行的 FastAPI 服务, Swagger 可测试 |
| Phase 3 | server/ (GUI + 机器码 + 云端验证 + TCP 监听) | Windows EXE, 可登录并监听 |
| Phase 4 | client/ (扫描 + 握手 + 注册表清理 + Setup API + Windows Service) | Windows EXE, 可安装为服务 |
| Phase 5 | cloud/frontend/ (Vue 管理后台) | 可访问的 Web 管理界面 |
| Phase 6 | 联调 + 防逆向加固 | 端到端测试通过 |

---

## 十、Windows 部署步骤 (给网吧使用)

### 网维服务器 (店老板操作):
1. 将 `hwt-server.exe` 复制到网维主机
2. 双击运行, 输入账号密码登录
3. 首次登录自动绑定机器码
4. 看到 "已授权" + TCP 监听已启动 → 就绪

### 客户端 (所有工作站):
1. 以管理员身份打开 cmd
2. `hwt-client.exe install`
3. 服务自动启动, 后续每次开机自动运行
4. 卸载: `hwt-client.exe uninstall`

### 云端 (你的服务器):
1. 部署 FastAPI + Vue
2. 首次启动自动创建 admin 账号 (admin / admin123)
3. 登录后台审核用户、设置授权
