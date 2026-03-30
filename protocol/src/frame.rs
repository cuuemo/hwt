use crate::crypto::{aes_decrypt, aes_encrypt};
use crate::Message;
use std::io::{Error, ErrorKind, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// 从 TcpStream 读取一帧 (明文)
/// 帧格式: [4 bytes big-endian u32 长度][payload]
pub async fn read_frame(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let len = stream.read_u32().await? as usize;
    if len == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "frame length is zero"));
    }
    // 防止恶意超大帧
    if len > 16 * 1024 * 1024 {
        return Err(Error::new(ErrorKind::InvalidData, "frame too large"));
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    Ok(buf)
}

/// 写入一帧 (自动加长度前缀)
/// 帧格式: [4 bytes big-endian u32 长度][payload]
pub async fn write_frame(stream: &mut TcpStream, data: &[u8]) -> Result<()> {
    let len = data.len() as u32;
    stream.write_u32(len).await?;
    stream.write_all(data).await?;
    stream.flush().await?;
    Ok(())
}

/// 加密并写入一帧
/// 1. 将 Message 序列化为 JSON
/// 2. 用 AES-256-GCM 加密 (结果格式: [12 IV][密文][16 TAG])
/// 3. 以长度前缀帧写入
pub async fn write_encrypted(
    stream: &mut TcpStream,
    key: &[u8; 32],
    msg: &Message,
) -> Result<()> {
    let json = serde_json::to_vec(msg)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {e}")))?;
    let encrypted = aes_encrypt(key, &json)?;
    write_frame(stream, &encrypted).await
}

/// 读取并解密一帧
/// 1. 读取长度前缀帧
/// 2. 用 AES-256-GCM 解密 (输入格式: [12 IV][密文][16 TAG])
/// 3. 反序列化 JSON 为 Message
pub async fn read_encrypted(stream: &mut TcpStream, key: &[u8; 32]) -> Result<Message> {
    let encrypted = read_frame(stream).await?;
    let json = aes_decrypt(key, &encrypted)?;
    serde_json::from_slice(&json)
        .map_err(|e| Error::new(ErrorKind::Other, format!("deserialize error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_aes_key;
    use crate::Message;
    use tokio::net::TcpListener;

    async fn tcp_pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let client = TcpStream::connect(addr).await.unwrap();
        let (server, _) = listener.accept().await.unwrap();
        (client, server)
    }

    #[tokio::test]
    async fn test_frame_roundtrip() {
        let (mut c, mut s) = tcp_pair().await;
        let data = b"hello HWT frame";
        write_frame(&mut c, data).await.unwrap();
        let received = read_frame(&mut s).await.unwrap();
        assert_eq!(received, data);
    }

    #[tokio::test]
    async fn test_encrypted_frame_roundtrip() {
        let (mut c, mut s) = tcp_pair().await;
        let key = generate_aes_key();
        let msg = Message::AuthRequest {
            client_id: "test-pc".to_string(),
            client_mac: Some("aa:bb:cc".to_string()),
        };
        write_encrypted(&mut c, &key, &msg).await.unwrap();
        let received = read_encrypted(&mut s, &key).await.unwrap();
        match received {
            Message::AuthRequest { client_id, client_mac } => {
                assert_eq!(client_id, "test-pc");
                assert_eq!(client_mac.unwrap(), "aa:bb:cc");
            }
            _ => panic!("wrong message type"),
        }
    }

    #[tokio::test]
    async fn test_multiple_encrypted_frames() {
        let (mut c, mut s) = tcp_pair().await;
        let key = generate_aes_key();
        write_encrypted(&mut c, &key, &Message::Handshake).await.unwrap();
        write_encrypted(&mut c, &key, &Message::Heartbeat).await.unwrap();
        write_encrypted(&mut c, &key, &Message::HeartbeatAck).await.unwrap();
        assert!(matches!(read_encrypted(&mut s, &key).await.unwrap(), Message::Handshake));
        assert!(matches!(read_encrypted(&mut s, &key).await.unwrap(), Message::Heartbeat));
        assert!(matches!(read_encrypted(&mut s, &key).await.unwrap(), Message::HeartbeatAck));
    }
}
