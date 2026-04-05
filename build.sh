#!/bin/bash
set -e

TARGET=x86_64-pc-windows-gnu
RELEASE=target/$TARGET/release

echo "=== 编译 client (公用) ==="
cargo build --release --target $TARGET -p hwt-client

echo ""
echo "=== 编译 server [IP版] — http://159.195.77.25:10000 ==="
CLOUD_BASE_URL=http://159.195.77.25:10000 cargo build --release --target $TARGET -p hwt-server
mkdir -p dist/ip
cp $RELEASE/hwt-server.exe dist/ip/hwt-server.exe
cp $RELEASE/hwt-server.exe dist/ip/hwt-server-ip.exe
cp $RELEASE/hwt-client.exe dist/ip/hwt-client.exe
echo "  -> dist/ip/hwt-server.exe"
echo "  -> dist/ip/hwt-server-ip.exe"
echo "  -> dist/ip/hwt-client.exe"

echo ""
echo "=== 编译 server [域名版] — http://cuuemo.cn:10000 ==="
CLOUD_BASE_URL=http://cuuemo.cn:10000 cargo build --release --target $TARGET -p hwt-server
mkdir -p dist/domain
cp $RELEASE/hwt-server.exe dist/domain/hwt-server.exe
cp $RELEASE/hwt-server.exe dist/domain/hwt-server-domain.exe
cp $RELEASE/hwt-client.exe dist/domain/hwt-client.exe
echo "  -> dist/domain/hwt-server.exe"
echo "  -> dist/domain/hwt-server-domain.exe"
echo "  -> dist/domain/hwt-client.exe"

echo ""
echo "=== 完成 ==="
ls -lh dist/ip/ dist/domain/
