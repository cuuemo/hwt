/**
 * Format timestamp to a human readable string based on the current locale
 * @param time ISO timestamp or null
 * @param includeSeconds Whether to include seconds in the output
 * @returns Formatted time string
 */
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
  const lang = localStorage.getItem('lang') || 'zh'
  
  const options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  }
  
  if (includeSeconds) {
    options.second = '2-digit'
  }
  
  return d.toLocaleString(lang === 'zh' ? 'zh-CN' : 'en-US', options)
}
