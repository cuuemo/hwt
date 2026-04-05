pub mod crypto;
pub mod frame;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    // 握手阶段
    #[serde(rename = "handshake")]
    Handshake,

    #[serde(rename = "handshake_response")]
    HandshakeResponse { public_key: String },

    #[serde(rename = "key_exchange")]
    KeyExchange { encrypted_key: String },

    #[serde(rename = "key_exchange_ok")]
    KeyExchangeOk,

    // 业务阶段 (加密传输)
    #[serde(rename = "auth_request")]
    AuthRequest {
        client_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_mac: Option<String>,
    },

    #[serde(rename = "auth_response")]
    AuthResponse {
        authorized: bool,
        message: String,
        server_time: u64,
    },

    #[serde(rename = "heartbeat")]
    Heartbeat,

    #[serde(rename = "heartbeat_ack")]
    HeartbeatAck,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_serialization() {
        let msg = Message::Handshake;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"handshake\""));
        let decoded: Message = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded, Message::Handshake));
    }

    #[test]
    fn test_auth_response_serialization() {
        let msg = Message::AuthResponse {
            authorized: true,
            message: "ok".to_string(),
            server_time: 1711612800,
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"authorized\":true"));
        assert!(json.contains("\"server_time\":1711612800"));
        let decoded: Message = serde_json::from_str(&json).unwrap();
        match decoded {
            Message::AuthResponse {
                authorized,
                message,
                server_time,
            } => {
                assert!(authorized);
                assert_eq!(message, "ok");
                assert_eq!(server_time, 1711612800);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_all_message_variants() {
        let variants: Vec<Message> = vec![
            Message::Handshake,
            Message::HandshakeResponse {
                public_key: "pk".into(),
            },
            Message::KeyExchange {
                encrypted_key: "ek".into(),
            },
            Message::KeyExchangeOk,
            Message::AuthRequest {
                client_id: "host".into(),
                client_mac: Some("mac".into()),
            },
            Message::AuthRequest {
                client_id: "host".into(),
                client_mac: None,
            },
            Message::AuthResponse {
                authorized: false,
                message: "no".into(),
                server_time: 0,
            },
            Message::Heartbeat,
            Message::HeartbeatAck,
        ];
        for msg in variants {
            let json = serde_json::to_vec(&msg).unwrap();
            let _decoded: Message = serde_json::from_slice(&json).unwrap();
        }
    }
}
