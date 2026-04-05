import request from './request'
import { encryptPassword } from './crypto'

export interface UserInfo {
  id: number
  username: string
  email: string | null
  role: string
  status: string
  license_type: string | null
  license_expire_at: string | null
  created_at: string | null
  updated_at: string | null
}

export interface LoginResult {
  access_token: string
  token_type: string
  user: UserInfo
}

export async function login(username: string, password: string): Promise<LoginResult> {
  const passwordEncrypted = await encryptPassword(password)
  const res = await request.post('/api/auth/login', {
    username,
    password_encrypted: passwordEncrypted,
  })
  return res.data
}

export async function register(username: string, password: string, email?: string): Promise<{ id: number; username: string; status: string; message: string }> {
  const passwordEncrypted = await encryptPassword(password)
  const res = await request.post('/api/auth/register', {
    username,
    password_encrypted: passwordEncrypted,
    email: email || null,
  })
  return res.data
}
