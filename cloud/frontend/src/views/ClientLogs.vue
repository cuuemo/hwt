<template>
  <div class="page-container">
    <el-card shadow="never" class="table-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ $t('clientLogs.title') }}</span>
        </div>
      </template>

      <!-- Upload zone -->
      <el-upload
        drag
        :auto-upload="false"
        :show-file-list="false"
        :on-change="handleFileChange"
        accept=".enc"
        class="upload-zone"
      >
        <el-icon class="el-icon--upload"><UploadFilled /></el-icon>
        <div class="el-upload__text">
          {{ $t('clientLogs.upload.hint') }}
        </div>
        <template #tip>
          <div class="el-upload__tip">{{ $t('clientLogs.upload.accept') }}</div>
        </template>
      </el-upload>

      <el-skeleton v-if="loading" :rows="5" animated class="loading-skeleton" />

      <!-- Result pane -->
      <div v-if="result && !loading" class="result-pane">
        <div class="summary">
          <span class="filename">{{ result.filename }}</span>
          <el-tag size="small" type="info">
            {{ $t('clientLogs.summary.lines', { n: result.total_lines }) }}
          </el-tag>
          <el-tag v-if="result.truncated" size="small" type="warning">
            {{ $t('clientLogs.truncatedWarning', { n: result.total_lines }) }}
          </el-tag>
          <div class="spacer" />
          <el-input
            v-model="query"
            :placeholder="$t('clientLogs.searchPlaceholder')"
            clearable
            class="search-input"
          >
            <template #prefix><el-icon><Search /></el-icon></template>
          </el-input>
          <el-button type="primary" @click="downloadTxt">
            <el-icon><Download /></el-icon>
            <span>{{ $t('clientLogs.download') }}</span>
          </el-button>
        </div>

        <pre class="log-view">{{ filteredText }}</pre>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { UploadFile } from 'element-plus'
import { ElMessage } from 'element-plus'
import { UploadFilled, Search, Download } from '@element-plus/icons-vue'
import { decryptClientLog, type DecryptedLogResponse } from '../api/admin'

const { t } = useI18n()
const loading = ref(false)
const result = ref<DecryptedLogResponse | null>(null)
const query = ref('')

async function handleFileChange(uploadFile: UploadFile) {
  if (!uploadFile.raw) return
  loading.value = true
  result.value = null
  query.value = ''
  try {
    const resp = await decryptClientLog(uploadFile.raw)
    result.value = resp.data
  } catch (err) {
    // request.ts interceptor already shows an error toast
    console.error('decrypt failed:', err)
  } finally {
    loading.value = false
  }
}

const filteredLines = computed(() => {
  if (!result.value) return []
  const q = query.value.trim().toLowerCase()
  if (!q) return result.value.lines
  return result.value.lines.filter((l) => l.toLowerCase().includes(q))
})

const filteredText = computed(() =>
  filteredLines.value
    .map((l, i) => String(i + 1).padStart(5, ' ') + '  ' + l)
    .join('\n'),
)

function downloadTxt() {
  if (!result.value) return
  const blob = new Blob([result.value.lines.join('\n')], {
    type: 'text/plain;charset=utf-8',
  })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = result.value.filename.replace(/\.enc$/, '') + '.txt'
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  ElMessage.success(t('common.success'))
}
</script>

<style scoped>
.page-container { padding: 0; }
.table-card {
  border: none;
  background: var(--at-bg-card);
}
.card-header .title {
  font-size: 18px;
  font-weight: 700;
  color: var(--at-text-primary);
}
.upload-zone { margin-top: 8px; }
.loading-skeleton { margin-top: 16px; }
.result-pane { margin-top: 20px; }
.summary {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
  margin-bottom: 12px;
}
.filename {
  font-family: monospace;
  font-weight: 600;
  color: var(--at-text-primary);
}
.spacer { flex: 1; }
.search-input { width: 240px; }
.log-view {
  background: var(--at-bg-main, #0f1420);
  color: var(--at-text-primary, #e6e8f0);
  border: 1px solid var(--at-border, #2a3142);
  border-radius: 6px;
  padding: 12px 16px;
  max-height: 60vh;
  overflow: auto;
  font-family: Menlo, Consolas, monospace;
  font-size: 12px;
  line-height: 1.55;
  white-space: pre;
  margin: 0;
}
@media (max-width: 768px) {
  .search-input { width: 100%; }
}
</style>
