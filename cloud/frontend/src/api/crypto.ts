import request from './request'

let cachedPublicKey: CryptoKey | null = null

function pemToArrayBuffer(pem: string): ArrayBuffer {
  const b64 = pem
    .replace(/-----BEGIN [^-]+-----/, '')
    .replace(/-----END [^-]+-----/, '')
    .replace(/\s+/g, '')
  const bin = atob(b64)
  const buf = new Uint8Array(bin.length)
  for (let i = 0; i < bin.length; i++) buf[i] = bin.charCodeAt(i)
  return buf.buffer
}

function arrayBufferToBase64(buf: ArrayBuffer): string {
  let s = ''
  const bytes = new Uint8Array(buf)
  for (let i = 0; i < bytes.length; i++) s += String.fromCharCode(bytes[i])
  return btoa(s)
}

async function getPublicKey(): Promise<CryptoKey> {
  if (cachedPublicKey) return cachedPublicKey
  const res = await request.get('/api/auth/public-key')
  const pem = res.data.public_key as string
  cachedPublicKey = await crypto.subtle.importKey(
    'spki',
    pemToArrayBuffer(pem),
    { name: 'RSA-OAEP', hash: 'SHA-256' },
    false,
    ['encrypt'],
  )
  return cachedPublicKey
}

export async function encryptPassword(password: string): Promise<string> {
  const key = await getPublicKey()
  const encoded = new TextEncoder().encode(password)
  const cipher = await crypto.subtle.encrypt({ name: 'RSA-OAEP' }, key, encoded)
  return arrayBufferToBase64(cipher)
}
