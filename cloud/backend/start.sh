#!/bin/bash
# HWT 网维系统云端启动脚本
# 监听 0.0.0.0:10000，IP 和域名都可访问
# 用法: bash start.sh [--reload]

cd "$(dirname "$0")"

# 安装依赖（首次运行）
if ! python3 -c "import fastapi" 2>/dev/null; then
    echo "[INFO] 安装依赖..."
    pip3 install -r requirements.txt -q
fi

# 生成 RSA 密钥（首次运行）
mkdir -p keys

echo "[INFO] 启动 HWT 云端服务，监听 0.0.0.0:10000"
echo "[INFO] IP 访问:     http://159.195.77.25:10000"
echo "[INFO] 域名访问:    http://cuuemo.cn:10000"
echo "[INFO] 管理后台:    http://159.195.77.25:10000/docs"

exec uvicorn app.main:app \
    --host 0.0.0.0 \
    --port 10000 \
    --workers 2 \
    --access-log \
    "$@"
