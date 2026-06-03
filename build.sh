#!/bin/bash
set -e

TARGET=x86_64-pc-windows-gnu
RELEASE=target/$TARGET/release

PUBKEY_FILE="cloud/backend/keys/rsa_public.pem"
if [ ! -f "$PUBKEY_FILE" ]; then
    echo "ERROR: $PUBKEY_FILE not found — run cloud backend once to generate RSA keypair." >&2
    exit 1
fi
export CLOUD_PUBLIC_KEY_PEM="$(cat "$PUBKEY_FILE")"
echo "=== 嵌入云端公钥 ($PUBKEY_FILE) ==="

echo "=== 编译 client (公用) ==="
cargo build --release --target $TARGET -p at-client

echo "=== 编译 heartbeat (公用，普通用户心跳进程) ==="
cargo build --release --target $TARGET -p at-heartbeat

echo ""
echo "=== 编译 server [IP版] — http://159.195.77.25:10000 ==="
CLOUD_BASE_URL=http://159.195.77.25:10000 cargo build --release --target $TARGET -p at-server
mkdir -p dist/ip
cp $RELEASE/at-server.exe dist/ip/at-server.exe
cp $RELEASE/at-server.exe dist/ip/at-server-ip.exe
cp $RELEASE/at-client.exe dist/ip/at-client.exe
cp $RELEASE/at-heartbeat.exe dist/ip/at-heartbeat.exe
# Windows .bat must be GBK-encoded with CRLF line endings, otherwise a
# Chinese Windows cmd.exe mis-parses the file (comments run as commands).
bat_to_win() { iconv -f UTF-8 -t GBK "$1" | sed 's/$/\r/' > "$2"; }
bat_to_win installer/install.bat   dist/ip/install.bat
bat_to_win installer/uninstall.bat dist/ip/uninstall.bat
echo "  -> dist/ip/at-server.exe"
echo "  -> dist/ip/at-server-ip.exe"
echo "  -> dist/ip/at-client.exe"
echo "  -> dist/ip/at-heartbeat.exe"
echo "  -> dist/ip/install.bat"

echo ""
echo "=== 编译 server [域名版] — http://cuuemo.cn:10000 ==="
CLOUD_BASE_URL=http://cuuemo.cn:10000 cargo build --release --target $TARGET -p at-server
mkdir -p dist/domain
cp $RELEASE/at-server.exe dist/domain/at-server.exe
cp $RELEASE/at-server.exe dist/domain/at-server-domain.exe
cp $RELEASE/at-client.exe dist/domain/at-client.exe
cp $RELEASE/at-heartbeat.exe dist/domain/at-heartbeat.exe
bat_to_win installer/install.bat   dist/domain/install.bat
bat_to_win installer/uninstall.bat dist/domain/uninstall.bat
echo "  -> dist/domain/at-server.exe"
echo "  -> dist/domain/at-server-domain.exe"
echo "  -> dist/domain/at-client.exe"
echo "  -> dist/domain/at-heartbeat.exe"
echo "  -> dist/domain/install.bat"

echo ""
echo "=== 完成 ==="
ls -lh dist/ip/ dist/domain/
