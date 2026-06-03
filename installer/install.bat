@echo off
chcp 65001 >nul
title AT 客户端安装

REM ============================================================
REM  AT 网维客户端 一键安装
REM  使用方法：把本脚本与 at-client.exe、at-heartbeat.exe 放在
REM  同一目录，双击运行即可（会自动请求管理员权限）。
REM
REM  做镜像流程：
REM    1) 虚拟机装好系统并开机
REM    2) 把三个文件放同一目录，双击本脚本安装
REM    3) 直接打包镜像（服务已注册，会保留在镜像里）
REM    4) 镜像发到还原机后，每次开机自动运行，无需再装
REM ============================================================

REM —— 自动提权（非管理员则以管理员身份重新启动自己）——
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo 正在请求管理员权限...
    powershell -Command "Start-Process -FilePath '%~f0' -Verb RunAs"
    exit /b
)

cd /d "%~dp0"

if not exist "at-client.exe" (
    echo [错误] 当前目录找不到 at-client.exe
    echo        请把本脚本和 at-client.exe 放在同一个文件夹再运行。
    echo.
    pause
    exit /b 1
)

if not exist "at-heartbeat.exe" (
    echo [警告] 当前目录找不到 at-heartbeat.exe
    echo        心跳进程将无法启动。请确认它与 at-client.exe 在同一目录。
    echo.
    pause
)

echo 正在安装 AT 服务...
echo.
at-client.exe install
echo.
echo ============================================================
echo  安装完成。
echo  现在可以直接打包镜像，服务会随镜像保留并在每次开机自动运行。
echo ============================================================
echo.
pause
