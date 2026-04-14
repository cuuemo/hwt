use crate::crypto::{aes_encrypt, generate_aes_key, public_key_from_pem, rsa_encrypt};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Error, ErrorKind, Result, Write};
use std::path::Path;
use std::sync::Mutex;

const MAGIC: &[u8; 4] = b"ATLG";
const VERSION: u16 = 1;

pub struct EncryptedLogWriter {
    inner: Mutex<Inner>,
    key: [u8; 32],
}

struct Inner {
    file: BufWriter<File>,
}

impl EncryptedLogWriter {
    pub fn create<P: AsRef<Path>>(path: P, cloud_public_key_pem: &str) -> Result<Self> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let pub_key = public_key_from_pem(cloud_public_key_pem)?;
        let key = generate_aes_key();
        let wrapped = rsa_encrypt(&pub_key, &key)?;
        if wrapped.len() > u16::MAX as usize {
            return Err(Error::new(ErrorKind::Other, "wrapped key too large"));
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let mut w = BufWriter::new(file);

        w.write_all(MAGIC)?;
        w.write_all(&VERSION.to_be_bytes())?;
        w.write_all(&(wrapped.len() as u16).to_be_bytes())?;
        w.write_all(&wrapped)?;
        w.flush()?;

        Ok(Self {
            inner: Mutex::new(Inner { file: w }),
            key,
        })
    }

    pub fn write_line(&self, line: &str) -> Result<()> {
        let frame = aes_encrypt(&self.key, line.as_bytes())?;
        if frame.len() > u32::MAX as usize {
            return Err(Error::new(ErrorKind::Other, "frame too large"));
        }
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| Error::new(ErrorKind::Other, "log mutex poisoned"))?;
        guard.file.write_all(&(frame.len() as u32).to_be_bytes())?;
        guard.file.write_all(&frame)?;
        guard.file.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{aes_decrypt, generate_rsa_keypair, public_key_to_pem, rsa_decrypt};
    use std::io::Read;

    #[test]
    fn test_encrypted_log_roundtrip() {
        let (priv_key, pub_key) = generate_rsa_keypair();
        let pem = public_key_to_pem(&pub_key);

        let tmp = std::env::temp_dir().join("at_test_log.enc");
        let writer = EncryptedLogWriter::create(&tmp, &pem).unwrap();
        writer.write_line("hello world").unwrap();
        writer.write_line("second line 中文").unwrap();
        drop(writer);

        let mut buf = Vec::new();
        File::open(&tmp).unwrap().read_to_end(&mut buf).unwrap();

        assert_eq!(&buf[..4], MAGIC);
        let version = u16::from_be_bytes([buf[4], buf[5]]);
        assert_eq!(version, 1);
        let key_len = u16::from_be_bytes([buf[6], buf[7]]) as usize;
        let wrapped = &buf[8..8 + key_len];
        let aes_key_vec = rsa_decrypt(&priv_key, wrapped).unwrap();
        let mut aes_key = [0u8; 32];
        aes_key.copy_from_slice(&aes_key_vec);

        let mut pos = 8 + key_len;
        let mut lines = Vec::new();
        while pos < buf.len() {
            let flen = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]])
                as usize;
            pos += 4;
            let pt = aes_decrypt(&aes_key, &buf[pos..pos + flen]).unwrap();
            lines.push(String::from_utf8(pt).unwrap());
            pos += flen;
        }
        assert_eq!(lines, vec!["hello world", "second line 中文"]);
        let _ = std::fs::remove_file(&tmp);
    }
}
