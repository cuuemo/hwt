import request from './request'

export interface PaginatedResponse<T> {
  total: number
  items: T[]
}

export interface UserItem {
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

export interface UserUpdateData {
  status?: string
  role?: string
  license_type?: string
  license_expire_at?: string | null
  email?: string
}

export interface BindingItem {
  id: number
  user_id: number
  username: string | null
  machine_code: string
  bound_at: string | null
  last_verified_at: string | null
  status: string
}

export interface LogItem {
  id: number
  user_id: number | null
  username: string | null
  machine_code: string | null
  action: string
  ip_address: string | null
  result: string
  detail: string | null
  created_at: string | null
}

export interface UserQueryParams {
  page?: number
  size?: number
  status?: string
  username?: string
}

export interface BindingQueryParams {
  page?: number
  size?: number
  user_id?: number
}

export interface LogQueryParams {
  page?: number
  size?: number
  username?: string
  action?: string
  date_from?: string
  date_to?: string
}

export function getUsers(params: UserQueryParams): Promise<{ data: PaginatedResponse<UserItem> }> {
  return request.get('/api/admin/users', { params })
}

export function updateUser(id: number, data: UserUpdateData): Promise<{ data: UserItem }> {
  return request.patch(`/api/admin/users/${id}`, data)
}

export function getBindings(params: BindingQueryParams): Promise<{ data: PaginatedResponse<BindingItem> }> {
  return request.get('/api/admin/bindings', { params })
}

export function deleteBinding(id: number): Promise<{ data: { message: string } }> {
  return request.delete(`/api/admin/bindings/${id}`)
}

export function getLogs(params: LogQueryParams): Promise<{ data: PaginatedResponse<LogItem> }> {
  return request.get('/api/admin/logs', { params })
}

export interface DecryptedLogResponse {
  filename: string
  total_lines: number
  truncated: boolean
  lines: string[]
}

export function decryptClientLog(
  file: File,
): Promise<{ data: DecryptedLogResponse }> {
  const form = new FormData()
  form.append('file', file)
  return request.post('/api/admin/logs/decrypt', form, {
    headers: { 'Content-Type': 'multipart/form-data' },
    timeout: 60_000,
  })
}
