<template>
  <div class="logs-page">
    <el-card shadow="never">
      <template #header>
        <span style="font-weight: 600; font-size: 16px;">验证日志</span>
      </template>

      <!-- 筛选栏 -->
      <div class="filter-bar">
        <el-input
          v-model="queryParams.username"
          placeholder="搜索用户名"
          clearable
          style="width: 160px;"
          @clear="handleSearch"
          @keyup.enter="handleSearch"
        />
        <el-select
          v-model="queryParams.action"
          placeholder="操作类型"
          clearable
          style="width: 130px; margin-left: 12px;"
          @change="handleSearch"
        >
          <el-option label="登录" value="login" />
          <el-option label="注册" value="register" />
          <el-option label="验证" value="verify" />
          <el-option label="解绑" value="unbind" />
        </el-select>
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="至"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
          format="YYYY-MM-DD"
          value-format="YYYY-MM-DD"
          style="width: 260px; margin-left: 12px;"
          @change="handleDateChange"
        />
        <el-button type="primary" style="margin-left: 12px;" @click="handleSearch">
          <el-icon style="margin-right: 4px;"><Search /></el-icon>
          搜索
        </el-button>
      </div>

      <!-- 日志表格 -->
      <el-table :data="logs" v-loading="loading" stripe border style="width: 100%; margin-top: 16px;">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column prop="username" label="用户名" width="110">
          <template #default="{ row }">
            {{ row.username || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="machine_code" label="机器码" width="180">
          <template #default="{ row }">
            <el-tooltip v-if="row.machine_code" :content="row.machine_code" placement="top">
              <span class="machine-code">{{ row.machine_code }}</span>
            </el-tooltip>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="action" label="操作类型" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="actionTagType(row.action)" size="small">
              {{ actionLabel(row.action) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="ip_address" label="IP 地址" width="140">
          <template #default="{ row }">
            {{ row.ip_address || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="result" label="结果" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="row.result === 'success' ? 'success' : 'danger'" size="small">
              {{ row.result === 'success' ? '成功' : '失败' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="detail" label="详情" min-width="160">
          <template #default="{ row }">
            <el-tooltip v-if="row.detail" :content="row.detail" placement="top">
              <span class="detail-text">{{ row.detail }}</span>
            </el-tooltip>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="时间" width="160">
          <template #default="{ row }">
            {{ formatTime(row.created_at) }}
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-bar">
        <el-pagination
          v-model:current-page="queryParams.page"
          v-model:page-size="queryParams.size"
          :total="total"
          :page-sizes="[20, 50, 100, 200]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadLogs"
          @current-change="loadLogs"
        />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { Search } from '@element-plus/icons-vue'
import { getLogs } from '../api/admin'
import type { LogItem } from '../api/admin'

const loading = ref(false)
const logs = ref<LogItem[]>([])
const total = ref(0)
const dateRange = ref<[string, string] | null>(null)

const queryParams = reactive({
  page: 1,
  size: 50,
  username: '',
  action: '' as string,
  date_from: '',
  date_to: '',
})

function formatTime(time: string | null): string {
  if (!time) return '-'
  const d = new Date(time)
  return d.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
}

function actionTagType(action: string): string {
  const map: Record<string, string> = {
    login: '',
    register: 'warning',
    verify: 'success',
    unbind: 'danger',
  }
  return map[action] || 'info'
}

function actionLabel(action: string): string {
  const map: Record<string, string> = {
    login: '登录',
    register: '注册',
    verify: '验证',
    unbind: '解绑',
  }
  return map[action] || action
}

function handleDateChange(val: [string, string] | null) {
  if (val) {
    queryParams.date_from = val[0]
    queryParams.date_to = val[1]
  } else {
    queryParams.date_from = ''
    queryParams.date_to = ''
  }
  handleSearch()
}

function handleSearch() {
  queryParams.page = 1
  loadLogs()
}

async function loadLogs() {
  loading.value = true
  try {
    const params: Record<string, any> = {
      page: queryParams.page,
      size: queryParams.size,
    }
    if (queryParams.username) params.username = queryParams.username
    if (queryParams.action) params.action = queryParams.action
    if (queryParams.date_from) params.date_from = queryParams.date_from
    if (queryParams.date_to) params.date_to = queryParams.date_to

    const res = await getLogs(params)
    logs.value = res.data.items
    total.value = res.data.total
  } catch {
    // handled by interceptor
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadLogs()
})
</script>

<style scoped>
.filter-bar {
  display: flex;
  align-items: center;
}

.machine-code {
  font-family: monospace;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
  max-width: 160px;
  vertical-align: middle;
}

.detail-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
  max-width: 200px;
  vertical-align: middle;
}

.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
</style>
