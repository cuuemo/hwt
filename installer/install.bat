@echo off
chcp 936 >nul
title AT client install

REM Need admin; if not elevated, relaunch elevated.
net session >nul 2>&1
if errorlevel 1 (
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

cd /d "%~dp0"

if not exist "at-client.exe" (
    echo [错误] 当前目录找不到 at-client.exe
    echo 请把本脚本与 at-client.exe 放在同一个文件夹再运行。
    pause
    exit /b 1
)

if not exist "at-heartbeat.exe" (
    echo [警告] 当前目录找不到 at-heartbeat.exe，心跳将无法启动。
    pause
)

echo 正在安装 AT 启动任务（以 SYSTEM 权限运行）...
at-client.exe install
echo.
echo 安装完成。可以直接打包镜像。
echo 启动任务会随镜像保留，每次开机自动运行。
echo.
pause
