use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use hwt_protocol::crypto::{aes_decrypt, aes_encrypt, generate_aes_key, public_key_from_pem, rsa_encrypt};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};

const DEFAULT_CLOUD_BASE_URL: &str = "http://43.165.169.50:10000";

/// Response from cloud verify endpoint after decryption.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyResponse {
    pub authorized: bool,
    pub license_type: String,
    pub expire_at: Option<String>,
    pub message: String,
}

/// Register a new account on the cloud server.
pub async fn cloud_register(
    client: &reqwest::Client,
    base_url: &str,
    username: &str,
    password: &str,
) -> Result<String> {
    let base_url = if base_url.is_empty() { DEFAULT_CLOUD_BASE_URL } else { base_url };

    // Get public key for RSA encryption
    let url = format!("{}/api/auth/public-key", base_url);
    let resp = client.get(&url).send().await
        .map_err(|e| Error::new(ErrorKind::Other, format!("GET public-key failed: {e}")))?;
    let body: serde_json::Value = resp.json().await
        .map_err(|e| Error::new(ErrorKind::Other, format!("parse public-key JSON: {e}")))?;
    let pem_str = body["public_key"].as_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing public_key field"))?;

    let public_key = public_key_from_pem(pem_str)?;
    let encrypted = rsa_encrypt(&public_key, password.as_bytes())?;
    let password_encrypted = BASE64.encode(&encrypted);

    #[derive(Serialize)]
    struct RegisterRequest { username: String, password_encrypted: String }

    let resp = client.post(&format!("{}/api/auth/register", base_url))
        .json(&RegisterRequest { username: username.to_string(), password_encrypted })
        .send().await
        .map_err(|e| Error::new(ErrorKind::Other, format!("POST register failed: {e}")))?;

    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        return Err(Error::new(ErrorKind::Other,
            body["detail"].as_str().unwrap_or("注册失败").to_string()));
    }

    Ok("注册成功，请联系管理员获取授权后登录".to_string())
}

/// Perform RSA handshake with the cloud server to establish an AES session.
///
/// Returns (session_id, session_key).
pub async fn cloud_handshake(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(String, [u8; 32])> {
    let base_url = if base_url.is_empty() {
        DEFAULT_CLOUD_BASE_URL
    } else {
        base_url
    };

    // 1. GET /api/auth/public-key to retrieve cloud RSA public key
    let url = format!("{}/api/auth/public-key", base_url);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("GET public-key failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("GET public-key status: {}", resp.status()),
        ));
    }

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("parse public-key JSON: {e}")))?;

    let pem_str = body["public_key"]
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing public_key field"))?;

    let cloud_public_key = public_key_from_pem(pem_str)?;

    // 2. Generate random AES-256 session key
    let session_key = generate_aes_key();

    // 3. RSA encrypt the session key
    let encrypted = rsa_encrypt(&cloud_public_key, &session_key)?;
    let encoded = BASE64.encode(&encrypted);

    // 4. POST /api/verify/handshake
    let handshake_url = format!("{}/api/verify/handshake", base_url);

    #[derive(Serialize)]
    struct HandshakeRequest {
        encrypted_session_key: String,
    }

    let handshake_req = HandshakeRequest {
        encrypted_session_key: encoded,
    };

    let resp = client
        .post(&handshake_url)
        .json(&handshake_req)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("POST handshake failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("POST handshake status: {}", resp.status()),
        ));
    }

    let handshake_body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("parse handshake JSON: {e}")))?;

    let session_id = handshake_body["session_id"]
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing session_id field"))?
        .to_string();

    Ok((session_id, session_key))
}

/// Verify account + machine code via the cloud AES-encrypted channel.
pub async fn cloud_verify(
    client: &reqwest::Client,
    base_url: &str,
    session_id: &str,
    session_key: &[u8; 32],
    account: &str,
    password: &str,
    machine_code: &str,
) -> Result<VerifyResponse> {
    let base_url = if base_url.is_empty() {
        DEFAULT_CLOUD_BASE_URL
    } else {
        base_url
    };

    // 1. Build plaintext JSON payload
    let plaintext = serde_json::json!({
        "account": account,
        "password": password,
        "machine_code": machine_code,
    });
    let json_bytes = serde_json::to_vec(&plaintext)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize verify request: {e}")))?;

    // 2. AES-256-GCM encrypt
    let encrypted = aes_encrypt(session_key, &json_bytes)?;
    // encrypted format: [12 IV][ciphertext][16 TAG]
    // We need to split into iv, payload (ciphertext without tag), tag
    if encrypted.len() < 12 + 16 {
        return Err(Error::new(ErrorKind::Other, "encrypted data too short"));
    }
    let iv = &encrypted[..12];
    let payload_and_tag = &encrypted[12..];
    let tag = &payload_and_tag[payload_and_tag.len() - 16..];
    let payload = &payload_and_tag[..payload_and_tag.len() - 16];

    // 3. POST /api/verify
    let verify_url = format!("{}/api/verify", base_url);

    #[derive(Serialize)]
    struct VerifyRequest {
        session_id: String,
        iv: String,
        payload: String,
        tag: String,
    }

    let req = VerifyRequest {
        session_id: session_id.to_string(),
        iv: BASE64.encode(iv),
        payload: BASE64.encode(payload),
        tag: BASE64.encode(tag),
    };

    let resp = client
        .post(&verify_url)
        .json(&req)
        .send()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("POST verify failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("POST verify status: {}", resp.status()),
        ));
    }

    // 4. Response is also AES encrypted: { "iv": "...", "payload": "...", "tag": "..." }
    let resp_body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("parse verify response: {e}")))?;

    let resp_iv = BASE64
        .decode(
            resp_body["iv"]
                .as_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing iv in response"))?,
        )
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("decode iv: {e}")))?;

    let resp_payload = BASE64
        .decode(
            resp_body["payload"]
                .as_str()
                .ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "missing payload in response")
                })?,
        )
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("decode payload: {e}")))?;

    let resp_tag = BASE64
        .decode(
            resp_body["tag"]
                .as_str()
                .ok_or_else(|| Error::new(ErrorKind::InvalidData, "missing tag in response"))?,
        )
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("decode tag: {e}")))?;

    // Reassemble into the format aes_decrypt expects: [12 IV][ciphertext][16 TAG]
    let mut combined = Vec::with_capacity(resp_iv.len() + resp_payload.len() + resp_tag.len());
    combined.extend_from_slice(&resp_iv);
    combined.extend_from_slice(&resp_payload);
    combined.extend_from_slice(&resp_tag);

    let decrypted = aes_decrypt(session_key, &combined)?;

    let verify_resp: VerifyResponse = serde_json::from_slice(&decrypted)
        .map_err(|e| Error::new(ErrorKind::Other, format!("deserialize verify response: {e}")))?;

    Ok(verify_resp)
}
