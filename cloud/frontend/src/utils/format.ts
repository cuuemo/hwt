import i18n from '../i18n'

export function actionTagType(action: string): string {
  const map: Record<string, string> = {
    login: '',
    register: 'warning',
    verify: 'success',
    unbind: 'danger',
  }
  return map[action] || 'info'
}

export function formatTime(time: string | null, includeSeconds: boolean = false): string {
  if (!time) return '-'
  const d = new Date(time)
  const lang = String(i18n.global.locale.value || 'zh')

  const options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  }

  if (includeSeconds) {
    options.second = '2-digit'
  }

  return d.toLocaleString(lang === 'zh' ? 'zh-CN' : 'en-US', options)
}
