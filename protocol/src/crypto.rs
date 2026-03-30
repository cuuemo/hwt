use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::rngs::OsRng;
use rsa::pkcs8::{DecodePublicKey, EncodePublicKey, LineEnding};
use rsa::sha2;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use std::io::{Error, ErrorKind, Result};

/// 生成 RSA 2048-bit 密钥对
pub fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate RSA key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

/// RSA 公钥 → PEM 字符串
pub fn public_key_to_pem(key: &RsaPublicKey) -> String {
    key.to_public_key_pem(LineEnding::LF)
        .expect("failed to encode public key to PEM")
}

/// PEM 字符串 → RSA 公钥
pub fn public_key_from_pem(pem: &str) -> Result<RsaPublicKey> {
    RsaPublicKey::from_public_key_pem(pem)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("invalid PEM: {e}")))
}

/// RSA 加密 (OAEP-SHA256), 输入 ≤ 190 bytes (2048-bit key)
pub fn rsa_encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>> {
    let mut rng = OsRng;
    let padding = Oaep::new::<sha2::Sha256>();
    public_key
        .encrypt(&mut rng, padding, data)
        .map_err(|e| Error::new(ErrorKind::Other, format!("RSA encrypt error: {e}")))
}

/// RSA 解密
pub fn rsa_decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>> {
    let padding = Oaep::new::<sha2::Sha256>();
    private_key
        .decrypt(padding, ciphertext)
        .map_err(|e| Error::new(ErrorKind::Other, format!("RSA decrypt error: {e}")))
}

/// 生成随机 AES-256 密钥 (32 bytes)
pub fn generate_aes_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    rand::RngCore::fill_bytes(&mut OsRng, &mut key);
    key
}

/// AES-256-GCM 加密, 返回 [12 bytes IV][N bytes ciphertext][16 bytes tag] 拼接的 bytes
pub fn aes_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| Error::new(ErrorKind::Other, format!("AES key error: {e}")))?;

    let mut nonce_bytes = [0u8; 12];
    rand::RngCore::fill_bytes(&mut OsRng, &mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| Error::new(ErrorKind::Other, format!("AES encrypt error: {e}")))?;

    // 格式: [12 IV][N ciphertext][16 tag]
    // aes-gcm crate 的 encrypt 返回的是 ciphertext || tag 拼接
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// AES-256-GCM 解密, 输入为 aes_encrypt 的输出
pub fn aes_decrypt(key: &[u8; 32], encrypted: &[u8]) -> Result<Vec<u8>> {
    if encrypted.len() < 12 + 16 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "encrypted data too short",
        ));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| Error::new(ErrorKind::Other, format!("AES key error: {e}")))?;

    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext_and_tag = &encrypted[12..];

    cipher
        .decrypt(nonce, ciphertext_and_tag)
        .map_err(|e| Error::new(ErrorKind::Other, format!("AES decrypt error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_roundtrip() {
        let (priv_key, pub_key) = generate_rsa_keypair();
        let plaintext = b"Hello, HWT protocol!";
        let ct = rsa_encrypt(&pub_key, plaintext).unwrap();
        let pt = rsa_decrypt(&priv_key, &ct).unwrap();
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn test_rsa_pem_roundtrip() {
        let (priv_key, pub_key) = generate_rsa_keypair();
        let pem = public_key_to_pem(&pub_key);
        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----"));
        let restored = public_key_from_pem(&pem).unwrap();
        // Verify restored key works
        let ct = rsa_encrypt(&restored, b"pem test").unwrap();
        let pt = rsa_decrypt(&priv_key, &ct).unwrap();
        assert_eq!(pt, b"pem test");
    }

    #[test]
    fn test_aes_roundtrip() {
        let key = generate_aes_key();
        let plaintext = b"AES-256-GCM test for HWT network protocol";
        let encrypted = aes_encrypt(&key, plaintext).unwrap();
        assert!(encrypted.len() >= 12 + 16); // IV + tag minimum
        let decrypted = aes_decrypt(&key, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_tamper_detection() {
        let key = generate_aes_key();
        let mut encrypted = aes_encrypt(&key, b"tamper test data").unwrap();
        // Flip a byte in the ciphertext
        if encrypted.len() > 15 {
            encrypted[14] ^= 0xFF;
        }
        assert!(aes_decrypt(&key, &encrypted).is_err());
    }

    #[test]
    fn test_aes_wrong_key() {
        let key1 = generate_aes_key();
        let key2 = generate_aes_key();
        let encrypted = aes_encrypt(&key1, b"wrong key test").unwrap();
        assert!(aes_decrypt(&key2, &encrypted).is_err());
    }

    #[test]
    fn test_aes_short_input() {
        let key = generate_aes_key();
        assert!(aes_decrypt(&key, &[0u8; 10]).is_err()); // < 12+16 bytes
        assert!(aes_decrypt(&key, &[]).is_err());
    }
}
