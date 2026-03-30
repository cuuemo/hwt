import JSEncrypt from 'jsencrypt'
import request from './request'

let cachedPublicKey: string | null = null

export async function getPublicKey(): Promise<string> {
  if (cachedPublicKey) {
    return cachedPublicKey
  }
  const res = await request.get('/api/auth/public-key')
  cachedPublicKey = res.data.public_key
  return cachedPublicKey!
}

export async function encryptPassword(password: string): Promise<string> {
  const publicKey = await getPublicKey()
  const encrypt = new JSEncrypt()
  encrypt.setPublicKey(publicKey)
  const encrypted = encrypt.encrypt(password)
  if (!encrypted) {
    throw new Error('密码加密失败')
  }
  return encrypted
}
