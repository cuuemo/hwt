# 镜像分析与客户端注入指南

## 一、镜像分析结果

### 文件信息
```
文件名: SW_Win10_1909 v2023.0903.zip
实际格式: 7z SFX 自解压 PE32 (非 zip)
大小: 3.4 GB (压缩)
自解压路径: E:\Image
```

### 内部结构
```
Win10_1909/
├── IMG000001650F4DE10B.img    # 原始磁盘镜像 (11.3 GB 解压后)
├── IMG000001650F4DE10B.map    # 块映射文件 (689 KB)
├── 000001650F4DE299.dcp       # 配置段 (空)
├── 000001650F4DE299.map       # 配置段映射 (空)
└── Win10_1909.s3db            # SQLite 元数据 (5 KB)
```

### 镜像格式 (s3db 元数据)
```
DiskId: 301
DiskSize: 102406 MB (~100 GB)
DiskTypeFormat: 1 (原始磁盘镜像)
OSVersion: 655424 (Windows 10 1909 / 18363)
DiskName: Win10_1909
来源路径: D:\Image
```

这是**网吧无盘/还原系统**的磁盘镜像格式（类似赛维/深影/顺网/易乐游等网维系统使用的格式）。

### 分区结构 (.img)
```
分区表类型: MBR
引导代码: Windows 7 MBR

分区1 (唯一):
  类型: NTFS (0x07)
  状态: Active (可引导)
  起始扇区: 2048
  扇区数: 209,723,392
  大小: ~100 GB
  偏移: 2048 * 512 = 1,048,576 bytes (1 MB)
```

---

## 二、注入方案

### 目标
将 `hwt-client.exe` 内置到镜像中，使其：
- 开机自动运行
- 以 SYSTEM 权限运行（有权限操作 HKLM 注册表和 Setup API）
- 不依赖用户登录

### 方案 A: 离线注入到磁盘镜像 (Linux 操作)

**最推荐** — 直接修改 .img 文件，不需要开 Windows 虚拟机。

#### 步骤 1: 解压镜像
```bash
# 需要约 15 GB 空闲磁盘空间
7z e "SW_Win10_1909 v2023.0903.zip" "Win10_1909/IMG000001650F4DE10B.img" -o/path/to/workdir -y

# 如果空间不够，可以用外部存储
```

#### 步骤 2: 挂载 NTFS 分区
```bash
IMG=/path/to/workdir/IMG000001650F4DE10B.img

# 计算分区偏移: 起始扇区 2048 * 512 = 1048576
OFFSET=1048576

# 挂载
mkdir -p /mnt/winimg
mount -o loop,offset=$OFFSET,rw $IMG /mnt/winimg

# 验证
ls /mnt/winimg/Windows/System32/
# 应该能看到 Windows 系统文件
```

#### 步骤 3: 复制客户端 EXE
```bash
# 复制到 Windows 系统目录（不会被普通用户删除）
cp hwt-client.exe /mnt/winimg/Windows/System32/hwt-client.exe

# 或者创建专用目录
mkdir -p /mnt/winimg/HWT
cp hwt-client.exe /mnt/winimg/HWT/hwt-client.exe
```

#### 步骤 4: 离线注册 Windows 服务（修改注册表）

Windows 服务的本质是在注册表 `HKLM\SYSTEM\CurrentControlSet\Services\` 下创建一个键。

```bash
# 安装离线注册表编辑工具
apt install chntpw -y

# 加载 SYSTEM 注册表 hive
# Windows 注册表文件位置: Windows/System32/config/SYSTEM
chntpw -e /mnt/winimg/Windows/System32/config/SYSTEM
```

在 chntpw 交互模式下执行：
```
# 导航到 ControlSet001\Services (或 CurrentControlSet, 离线时用 ControlSet001)
cd ControlSet001\Services

# 创建服务键
nk HwtCleanupService

# 进入服务键
cd HwtCleanupService

# 设置服务参数:
# Type = 0x10 (SERVICE_WIN32_OWN_PROCESS)
ed Type
# 输入: 16 (十进制) = 0x10

# Start = 0x02 (SERVICE_AUTO_START 开机自启)
ed Start
# 输入: 2

# ErrorControl = 0x01 (SERVICE_ERROR_NORMAL)
ed ErrorControl
# 输入: 1

# ImagePath = EXE 路径
ed ImagePath
# 输入: C:\Windows\System32\hwt-client.exe
# 或: C:\HWT\hwt-client.exe

# DisplayName
ed DisplayName
# 输入: HWT Device Cleanup Service

# ObjectName = LocalSystem (SYSTEM 权限)
ed ObjectName
# 输入: LocalSystem

# Description (可选)
ed Description
# 输入: HWT Network Maintenance Device Cleanup Service

# 保存退出
q
y
```

**注意**: chntpw 的交互操作比较原始。更好的方案是用 Python 的 `regipy` 库或 `hivex` 工具：

```bash
# 使用 hivex (更可靠)
apt install libhivex-bin python3-hivex -y

# Python 脚本方式注册服务
python3 << 'PYEOF'
import hivex
import struct

SYSTEM_HIVE = "/mnt/winimg/Windows/System32/config/SYSTEM"
h = hivex.Hivex(SYSTEM_HIVE, write=True)

# 找到 ControlSet001\Services
root = h.root()
cs1 = h.node_get_child(root, "ControlSet001")
services = h.node_get_child(cs1, "Services")

# 创建 HwtCleanupService 键
svc = h.node_add_child(services, "HwtCleanupService")

# 设置值
# REG_DWORD = 4, REG_SZ = 1, REG_EXPAND_SZ = 2

# Type: SERVICE_WIN32_OWN_PROCESS (0x10 = 16)
h.node_set_value(svc, {
    "key": "Type",
    "t": 4,  # REG_DWORD
    "value": struct.pack("<I", 0x10)
})

# Start: SERVICE_AUTO_START (2)
h.node_set_value(svc, {
    "key": "Start",
    "t": 4,
    "value": struct.pack("<I", 2)
})

# ErrorControl: SERVICE_ERROR_NORMAL (1)
h.node_set_value(svc, {
    "key": "ErrorControl",
    "t": 4,
    "value": struct.pack("<I", 1)
})

# ImagePath: EXE 路径 (REG_EXPAND_SZ, UTF-16LE 编码)
exe_path = "C:\\Windows\\System32\\hwt-client.exe"
h.node_set_value(svc, {
    "key": "ImagePath",
    "t": 2,  # REG_EXPAND_SZ
    "value": (exe_path + "\0").encode("utf-16le")
})

# DisplayName
h.node_set_value(svc, {
    "key": "DisplayName",
    "t": 1,  # REG_SZ
    "value": ("HWT Device Cleanup Service\0").encode("utf-16le")
})

# ObjectName: LocalSystem
h.node_set_value(svc, {
    "key": "ObjectName",
    "t": 1,
    "value": ("LocalSystem\0").encode("utf-16le")
})

# Description
h.node_set_value(svc, {
    "key": "Description",
    "t": 1,
    "value": ("HWT 网维设备清理服务\0").encode("utf-16le")
})

h.commit(SYSTEM_HIVE)
print("服务注册成功: HwtCleanupService")
PYEOF
```

#### 步骤 5: 卸载并重新打包
```bash
# 卸载
umount /mnt/winimg

# 重新用 7z 打包为原格式
# 需要保持原有的目录结构和元数据文件
cd /path/to/workdir
mkdir -p Win10_1909
mv IMG000001650F4DE10B.img Win10_1909/
# 复制原来的 .map .dcp .s3db 文件回来
cp /tmp/imgmeta/* Win10_1909/

# 压缩 (使用与原始相同的参数)
7z a -t7z -mx=9 -m0=LZMA2:d=256m Win10_1909_modified.7z Win10_1909/

# 如果需要重新制作 SFX:
# 需要原 SFX 模块头 (前 141011 字节) + 新 7z
dd if="SW_Win10_1909 v2023.0903.zip" bs=1 count=141011 of=sfx_header.bin
cat sfx_header.bin Win10_1909_modified.7z > "SW_Win10_1909_modified.zip"
```

**重要提醒**: 修改 .img 后，如果网维系统使用了块级校验（.map 文件），可能需要：
- 删除旧的 .map 文件让网维系统重新生成
- 或在网维系统中重新导入修改后的镜像

---

### 方案 B: 虚拟机中安装后重新封装 (最可靠)

**最稳妥** — 不破坏任何元数据，适合不熟悉 Linux 的操作者。

#### 步骤
1. 用网维系统将镜像部署到一台测试机（或虚拟机）
2. 启动 Windows，以管理员身份打开 CMD
3. 执行安装：
   ```cmd
   copy hwt-client.exe C:\Windows\System32\
   hwt-client.exe install
   ```
4. 验证服务已注册：
   ```cmd
   sc query HwtCleanupService
   ```
5. 用网维系统重新采集/封装该机器的镜像
6. 将新镜像替换旧镜像

---

### 方案 C: 利用网维超管模式下发 (不修改镜像)

**最简单** — 不动镜像，利用网维系统自身的管理功能。

#### 原理
大多数网维系统（顺网、易乐游、云更新等）都有"超级管理员"或"开超级"功能，可以在还原保护下持久化修改。

#### 步骤
1. 在网维服务器上"开超级"（关闭还原保护）
2. 将 `hwt-client.exe` 复制到工作站的 `C:\Windows\System32\`
3. 以管理员 CMD 运行：
   ```cmd
   hwt-client.exe install
   ```
4. "关闭超级"（重新开启还原保护）
5. 此时服务已注册，重启后保留

#### 批量部署脚本 (deploy.bat)
```batch
@echo off
:: 批量部署脚本 - 在网维超管模式下运行

:: 复制 EXE
if not exist "C:\Windows\System32\hwt-client.exe" (
    copy /Y "\\网维服务器IP\share\hwt-client.exe" "C:\Windows\System32\hwt-client.exe"
)

:: 检查服务是否已安装
sc query HwtCleanupService >nul 2>&1
if %errorlevel% neq 0 (
    :: 安装服务
    "C:\Windows\System32\hwt-client.exe" install
    echo [%date% %time%] 服务安装成功 >> C:\HWT\install.log
) else (
    echo [%date% %time%] 服务已存在 >> C:\HWT\install.log
)
```

可通过网维系统的"批处理执行"或"远程命令"功能批量推送。

---

### 方案 D: 计划任务方式 (备选, 不依赖 Windows Service)

如果 Windows Service 注册在某些网维环境下有问题，可以改用计划任务：

```batch
@echo off
:: 创建开机运行的计划任务 (SYSTEM 权限)
schtasks /Create ^
    /TN "HWT\DeviceCleanup" ^
    /TR "C:\Windows\System32\hwt-client.exe run" ^
    /SC ONSTART ^
    /RU "SYSTEM" ^
    /RL HIGHEST ^
    /F

echo 计划任务创建成功
```

计划任务的优势：
- 不需要实现 Windows Service API（简化客户端代码）
- SYSTEM 权限 + 开机触发
- 网维超管模式下注册后持久生效

---

## 三、方案对比

| 方案 | 难度 | 可靠性 | 适用场景 |
|------|------|--------|---------|
| A: 离线注入 .img | 中 | 高 | 需要制作全新镜像发放 |
| B: VM 封装 | 低 | 最高 | 首次制作标准镜像 |
| C: 超管模式下发 | 最低 | 高 | 已部署的机器追加安装 |
| D: 计划任务 | 低 | 中 | 备选方案 |

---

## 四、推荐组合策略

1. **新镜像制作**: 用方案 B（VM 封装），在标准 Windows 中安装好服务后重新采集镜像
2. **已部署机器**: 用方案 C（超管模式），通过网维系统批量推送安装脚本
3. **未来更新**: 用方案 C 推送新版本 EXE 并重启服务

---

## 五、关于 SYSTEM 权限的说明

客户端需要 SYSTEM 权限来：
1. 删除 `HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY` 注册表子键
2. 调用 Setup API 移除幽灵设备（`SetupDiCallClassInstaller(DIF_REMOVE)`）

获取 SYSTEM 权限的方式：
- **Windows Service** (方案 A/B/C): `ObjectName = LocalSystem` → 天然 SYSTEM 权限
- **计划任务** (方案 D): `/RU "SYSTEM"` → SYSTEM 权限
- **网维超管模式**: 通常以 Administrator 运行，也有足够权限

两种方式都能满足权限需求。Windows Service 更稳定（SCM 管理、自动恢复），计划任务更简单。

---

## 六、客户端代码适配

如果选择方案 D（计划任务替代 Windows Service），客户端的 `main.rs` 已支持 `run` 子命令直接前台运行：
```
hwt-client.exe run    # 前台运行，不注册为服务
```

可以新增 `schedule` 子命令来注册计划任务：
```rust
// main.rs 新增
Some("schedule") => {
    // 调用 schtasks 命令注册计划任务
    std::process::Command::new("schtasks")
        .args(&["/Create", "/TN", "HWT\\DeviceCleanup",
                "/TR", &format!("\"{}\" run", std::env::current_exe()?.display()),
                "/SC", "ONSTART", "/RU", "SYSTEM", "/RL", "HIGHEST", "/F"])
        .status()?;
}
```
