#!/bin/bash
set -e

TARGET=x86_64-pc-windows-gnu
RELEASE=target/$TARGET/release

echo "=== 编译 client (公用) ==="
cargo build --release --target $TARGET -p at-client

echo ""
echo "=== 编译 server [IP版] — http://159.195.77.25:10000 ==="
CLOUD_BASE_URL=http://159.195.77.25:10000 cargo build --release --target $TARGET -p at-server
mkdir -p dist/ip
cp $RELEASE/at-server.exe dist/ip/at-server.exe
cp $RELEASE/at-server.exe dist/ip/at-server-ip.exe
cp $RELEASE/at-client.exe dist/ip/at-client.exe
echo "  -> dist/ip/at-server.exe"
echo "  -> dist/ip/at-server-ip.exe"
echo "  -> dist/ip/at-client.exe"

echo ""
echo "=== 编译 server [域名版] — http://cuuemo.cn:10000 ==="
CLOUD_BASE_URL=http://cuuemo.cn:10000 cargo build --release --target $TARGET -p at-server
mkdir -p dist/domain
cp $RELEASE/at-server.exe dist/domain/at-server.exe
cp $RELEASE/at-server.exe dist/domain/at-server-domain.exe
cp $RELEASE/at-client.exe dist/domain/at-client.exe
echo "  -> dist/domain/at-server.exe"
echo "  -> dist/domain/at-server-domain.exe"
echo "  -> dist/domain/at-client.exe"

echo ""
echo "=== 完成 ==="
ls -lh dist/ip/ dist/domain/
