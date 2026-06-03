@echo off
chcp 65001 >nul
title AT 客户端卸载

REM ============================================================
REM  AT 网维客户端 卸载（停止并删除服务）
REM  双击运行即可（会自动请求管理员权限）。
REM ============================================================

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo 正在请求管理员权限...
    powershell -Command "Start-Process -FilePath '%~f0' -Verb RunAs"
    exit /b
)

cd /d "%~dp0"

if not exist "at-client.exe" (
    echo [错误] 当前目录找不到 at-client.exe
    echo.
    pause
    exit /b 1
)

echo 正在卸载 AT 启动任务...
echo.
at-client.exe uninstall
echo.
echo 卸载完成。
echo.
pause
