import forge from 'node-forge'
import request from './request'

// Backend decrypts with RSA-OAEP + MGF1(SHA-256). We use node-forge so
// encryption also works in non-secure contexts (plain HTTP on a LAN IP),
// where window.crypto.subtle is unavailable.

let cachedPublicKey: forge.pki.rsa.PublicKey | null = null

async function getPublicKey(): Promise<forge.pki.rsa.PublicKey> {
  if (cachedPublicKey) return cachedPublicKey
  const res = await request.get('/api/auth/public-key')
  cachedPublicKey = forge.pki.publicKeyFromPem(res.data.public_key) as forge.pki.rsa.PublicKey
  return cachedPublicKey
}

export async function encryptPassword(password: string): Promise<string> {
  const key = await getPublicKey()
  const cipher = key.encrypt(forge.util.encodeUtf8(password), 'RSA-OAEP', {
    md: forge.md.sha256.create(),
    mgf1: { md: forge.md.sha256.create() },
  })
  return forge.util.encode64(cipher)
}
