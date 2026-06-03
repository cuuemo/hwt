@echo off
chcp 936 >nul
title AT client uninstall

REM Need admin; if not elevated, relaunch elevated.
net session >nul 2>&1
if errorlevel 1 (
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

cd /d "%~dp0"

if not exist "at-client.exe" (
    echo [错误] 当前目录找不到 at-client.exe
    pause
    exit /b 1
)

echo 正在卸载 AT 启动任务...
at-client.exe uninstall
echo.
echo 卸载完成。
echo.
pause
