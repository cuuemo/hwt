import axios, { type AxiosError } from 'axios'
import { ElMessage } from 'element-plus'
import router from '../router'
import i18n from '../i18n'
import { invalidatePublicKeyCache } from './crypto'

const request = axios.create({
  baseURL: '/',
  timeout: 15000,
})

request.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => Promise.reject(error),
)

function messageFromDetail(detail: unknown): string {
  const t = i18n.global.t
  if (typeof detail === 'string') return detail
  if (Array.isArray(detail)) {
    const parts = detail
      .map((d) =>
        d && typeof d === 'object' && 'msg' in d ? String((d as { msg: unknown }).msg) : null,
      )
      .filter((s): s is string => !!s)
    return parts.length ? parts.join('; ') : t('error.validation')
  }
  return t('error.requestFailed')
}

request.interceptors.response.use(
  (response) => response,
  (error: AxiosError<{ detail?: unknown }>) => {
    const t = i18n.global.t
    if (error.response) {
      const { status, data, config } = error.response
      if (status === 401) {
        localStorage.removeItem('token')
        localStorage.removeItem('user')
        if (router.currentRoute.value.path !== '/login') {
          router.push('/login')
          ElMessage.error(t('error.sessionExpired'))
        }
      } else {
        // Auth endpoints may fail if the cached RSA public key is stale
        // (backend key rotated). Drop the cache so the next retry refetches.
        const url = typeof config?.url === 'string' ? config.url : ''
        if (status === 400 && /\/api\/auth\/(login|register)$/.test(url)) {
          invalidatePublicKeyCache()
        }
        ElMessage.error(messageFromDetail(data?.detail))
      }
    } else {
      ElMessage.error(t('error.networkError'))
    }
    return Promise.reject(error)
  },
)

export default request
