@echo off
chcp 936 > nul
setlocal EnableDelayedExpansion

:: 设置窗口标题和颜色
title AT清理工具 - 专业无盘解决方案
mode con cols=80 lines=35

:: ========== 精美广告页 ==========
cls
echo.
echo.
echo                          ╔════════════════════════════════════╗
echo                          ║                                    ║
color 0C
echo                          ║        █████╗ ████████╗            ║
echo                          ║       ██╔══██╗╚══██╔══╝            ║
echo                          ║       ███████║   ██║               ║
echo                          ║       ██╔══██║   ██║               ║
echo                          ║       ██║  ██║   ██║               ║
echo                          ║       ╚═╝  ╚═╝   ╚═╝               ║
echo                          ║                                    ║
color 0E
echo                          ║        清理工具 █ 正在运行         ║
color 0A
echo                          ║                                    ║
echo                          ║    ╔══════════════════════════╗    ║
echo                          ║    ║                          ║    ║
echo                          ║    ║    无盘认准咸鱼AT无盘镜像║    ║
echo                          ║    ║                          ║    ║
echo                          ║    ║      微信：ATKJ_DZ       ║    ║
echo                          ║    ║                          ║    ║
echo                          ║    ╚══════════════════════════╝    ║
echo                          ║                                    ║
color 0B
echo                          ║        专业 █ 稳定 █ 可靠          ║
color 0A
echo                          ║                                    ║
echo                          ╚════════════════════════════════════╝
echo.
echo.
echo                           按任意键开始设备ID随机化...
pause > nul

:: ========== 清屏准备执行 ==========
cls
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                                                              ║
color 0C
echo ║                AT清理工具 - 设备ID与产品ID随机化程序         ║
color 0A
echo ║                                                              ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.
ping -n 2 127.0.0.1 > nul

:: ========== 步骤1：生成随机标识符 ==========
echo.
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 1/7]  正在生成随机标识符...         │
echo └──────────────────────────────────────────────────────────────┘
ping -n 2 127.0.0.1 > nul

:: 生成4个不同的随机GUID
for /f "delims={} tokens=1" %%i in ('powershell -command "[System.Guid]::NewGuid().ToString().ToUpper()"') do set "GUID1=%%i"
for /f "delims={} tokens=1" %%i in ('powershell -command "[System.Guid]::NewGuid().ToString().ToUpper()"') do set "GUID2=%%i"
for /f "delims={} tokens=1" %%i in ('powershell -command "[System.Guid]::NewGuid().ToString().ToUpper()"') do set "GUID3=%%i"
for /f "delims={} tokens=1" %%i in ('powershell -command "[System.Guid]::NewGuid().ToString().ToUpper()"') do set "GUID4=%%i"

:: 生成随机产品ID
for /f %%i in ('powershell -command "$chars='ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'; -join ((1..5) | %% { $chars[(Get-Random -Minimum 0 -Maximum $chars.Length)] })"') do set "PID1=%%i"
for /f %%i in ('powershell -command "$chars='ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'; -join ((1..5) | %% { $chars[(Get-Random -Minimum 0 -Maximum $chars.Length)] })"') do set "PID2=%%i"
for /f %%i in ('powershell -command "$chars='ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'; -join ((1..5) | %% { $chars[(Get-Random -Minimum 0 -Maximum $chars.Length)] })"') do set "PID3=%%i"
for /f %%i in ('powershell -command "$chars='ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'; -join ((1..5) | %% { $chars[(Get-Random -Minimum 0 -Maximum $chars.Length)] })"') do set "PID4=%%i"
for /f %%i in ('powershell -command "$chars='ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789'; -join ((1..5) | %% { $chars[(Get-Random -Minimum 0 -Maximum $chars.Length)] })"') do set "PID5=%%i"
set "PRODUCT_ID=%PID1%-%PID2%-%PID3%-%PID4%-%PID5%"

:: 显示生成的标识符
echo.
echo      █ 设备访问ID: {!GUID1!}
echo      █ 机器ID:     {!GUID2!}
echo      █ SQM ID:     {!GUID3!}
echo      █ 诊断追踪ID: {!GUID4!}
echo      █ 产品ID:     !PRODUCT_ID!
echo.
ping -n 2 127.0.0.1 > nul

:: ========== 步骤2：创建注册表文件 ==========
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 2/7]  正在创建临时注册表文件...     │
echo └──────────────────────────────────────────────────────────────┘
ping -n 2 127.0.0.1 > nul
(
    echo Windows Registry Editor Version 5.00
    echo.
    echo [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\DeviceAccess\Global]
    echo "DeviceId"="{!GUID1!}"
    echo.
    echo [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\SQMClient]
    echo "MachineId"="{!GUID2!}"
    echo "WindowsSQMId"="{!GUID3!}"
    echo.
    echo [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack]
    echo "DeviceId"="{!GUID4!}"
    echo.
    echo [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion]
    echo "ProductId"="!PRODUCT_ID!"
) > %temp%\random_device.reg

if exist %temp%\random_device.reg (
    echo       临时文件创建成功
    echo       %temp%\random_device.reg
) else (
    color 0C
    echo       临时文件创建失败！
    color 0A
)
echo.
ping -n 2 127.0.0.1 > nul

:: ========== 步骤3：导入注册表 ==========
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 3/7]  正在导入注册表文件...         │
echo └──────────────────────────────────────────────────────────────┘
ping -n 2 127.0.0.1 > nul
echo      正在导入，请稍候...
regedit /s %temp%\random_device.reg
if %errorlevel% equ 0 (
    echo       注册表导入成功！
) else (
    color 0C
    echo      注册表导入可能失败，错误码：%errorlevel%
    color 0A
)
echo.
ping -n 2 127.0.0.1 > nul

:: ========== 步骤4：验证修改结果 ==========
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 4/7]  验证注册表修改...             │
echo └──────────────────────────────────────────────────────────────┘
ping -n 2 127.0.0.1 > nul
echo      正在读取当前注册表值...
echo.

:: 验证设备访问ID
for /f "tokens=3*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\DeviceAccess\Global" /v "DeviceId" 2^>nul ^| find "DeviceId"') do set "CURRENT_GUID1=%%a %%b"
echo      █ 当前设备访问ID: !CURRENT_GUID1!

:: 验证机器ID
for /f "tokens=3*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\SQMClient" /v "MachineId" 2^>nul ^| find "MachineId"') do set "CURRENT_GUID2=%%a %%b"
echo      █ 当前机器ID: !CURRENT_GUID2!

:: 验证SQM ID
for /f "tokens=3*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\SQMClient" /v "WindowsSQMId" 2^>nul ^| find "WindowsSQMId"') do set "CURRENT_GUID3=%%a %%b"
echo      █ 当前SQM ID: !CURRENT_GUID3!

:: 验证诊断追踪ID
for /f "tokens=3*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack" /v "DeviceId" 2^>nul ^| find "DeviceId"') do set "CURRENT_GUID4=%%a %%b"
echo      █ 当前诊断追踪ID: !CURRENT_GUID4!

:: 验证产品ID
for /f "tokens=3*" %%a in ('reg query "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion" /v "ProductId" 2^>nul ^| find "ProductId"') do set "CURRENT_PRODUCT_ID=%%a %%b"
echo      █ 当前产品ID: !CURRENT_PRODUCT_ID!

echo.
ping -n 2 127.0.0.1 > nul

:: ========== 步骤5：清理临时文件 ==========
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 5/7]  正在清理临时文件...           │
echo └──────────────────────────────────────────────────────────────┘
ping -n 2 127.0.0.1 > nul
if exist %temp%\random_device.reg (
    del %temp%\random_device.reg
    if !errorlevel! equ 0 (
        echo      临时文件删除成功
    ) else (
        color 0C
        echo      临时文件删除失败！
        color 0A
    )
) else (
    echo      临时文件不存在，无需删除
)
echo.
ping -n 2 127.0.0.1 > nul

:: ========== 步骤6：显示完成信息 ==========
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 6/7]  操作完成！                    │
echo └──────────────────────────────────────────────────────────────┘
ping -n 2 127.0.0.1 > nul
echo.
color 0B
echo      ╔════════════════════════════════════════════════════╗
echo      ║                                                    ║
echo      ║          设备ID已成功随机化！                      ║
echo      ║          产品ID已成功随机化！                      ║
echo      ║                                                    ║
echo      ╚════════════════════════════════════════════════════╝
color 0A
echo.
ping -n 3 127.0.0.1 > nul

:: ========== 步骤7：倒计时关闭 ==========
echo ┌──────────────────────────────────────────────────────────────┐
echo │                    [步骤 7/7]  程序即将退出                  │
echo └──────────────────────────────────────────────────────────────┘
echo.
set /a count=4
:countdown
if %count% gtr 0 (
    echo      剩余 %count% 秒后自动关闭...
    ping -n 2 127.0.0.1 > nul
    set /a count-=1
    goto countdown
)

:: ========== 结束画面 ==========
cls
echo.
echo.
echo.
echo.
color 0C
echo                      ╔══════════════════════════╗
echo                      ║                          ║
echo                      ║   AT清理工具 执行完成    ║
echo                      ║                          ║
echo                      ╠══════════════════════════╣
echo                      ║                          ║
color 0E
echo                      ║   感谢使用咸鱼AT无盘镜像 ║
echo                      ║                          ║
echo                      ║   微信：ATKJ_DZ          ║
echo                      ║                          ║
color 0C
echo                      ╚══════════════════════════╝
color 0A
echo.
echo.
echo.
ping -n 5 127.0.0.1 > nul
exit