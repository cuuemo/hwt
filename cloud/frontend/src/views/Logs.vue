<template>
  <div class="page-container">
    <el-card shadow="never" class="table-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ $t('common.logs') }}</span>
        </div>
      </template>

      <!-- 筛选栏 -->
      <div class="filter-bar">
        <el-input
          v-model="queryParams.username"
          :placeholder="$t('logs.user')"
          clearable
          class="filter-item input-search"
          @clear="handleSearch"
          @keyup.enter="handleSearch"
        />
        <el-select
          v-model="queryParams.action"
          :placeholder="$t('logs.action')"
          clearable
          class="filter-item select-action"
          @change="handleSearch"
        >
          <el-option :label="$t('logs.actions.login')" value="login" />
          <el-option :label="$t('logs.actions.register')" value="register" />
          <el-option :label="$t('logs.actions.verify')" value="verify" />
          <el-option :label="$t('logs.actions.unbind')" value="unbind" />
        </el-select>
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          :range-separator="$t('common.to')"
          :start-placeholder="$t('common.time')"
          :end-placeholder="$t('common.time')"
          format="YYYY-MM-DD"
          value-format="YYYY-MM-DD"
          class="filter-item date-picker"
          @change="handleDateChange"
        />
        <el-button type="primary" class="filter-item" @click="handleSearch">
          <el-icon><Search /></el-icon>
          <span class="hidden-xs-only">{{ $t('common.search') }}</span>
        </el-button>
      </div>

      <!-- 日志表格 -->
      <el-table :data="logs" v-loading="loading" stripe style="width: 100%; margin-top: 16px;">
        <el-table-column prop="username" :label="$t('logs.user')" width="110">
          <template #default="{ row }">
            {{ row.username || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="machine_code" :label="$t('bindings.machineCode')" min-width="180">
          <template #default="{ row }">
            <el-tooltip v-if="row.machine_code" :content="row.machine_code" placement="top">
              <span class="machine-code-small">{{ row.machine_code }}</span>
            </el-tooltip>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="action" :label="$t('logs.action')" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="actionTagType(row.action)" size="small">
              {{ $t(`logs.actions.${row.action}`) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="ip_address" :label="$t('logs.ip')" width="140">
          <template #default="{ row }">
            {{ row.ip_address || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="result" :label="$t('logs.result')" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="row.result === 'success' ? 'success' : 'danger'" size="small">
              {{ row.result === 'success' ? $t('common.success') : $t('common.failed') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="detail" :label="$t('logs.detail')" min-width="160" show-overflow-tooltip>
          <template #default="{ row }">
            {{ row.detail || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="created_at" :label="$t('common.time')" width="160">
          <template #default="{ row }">
            {{ formatTime(row.created_at, true) }}
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
import { formatTime, actionTagType } from '../utils/format'
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
  } catch (err) {
    console.error('Failed to load logs:', err)
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadLogs()
})
</script>

<style scoped>
.page-container {
  padding: 0;
}
.table-card {
  border: none;
  background: var(--at-bg-card);
}
.card-header .title {
  font-size: 18px;
  font-weight: 700;
  color: var(--at-text-primary);
}
.filter-bar {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 20px;
}
.filter-item { margin: 0; }
.input-search { width: 180px; }
.select-action { width: 130px; }
.date-picker { width: 280px !important; }

.machine-code-small {
  font-family: monospace;
  font-size: 11px;
  color: var(--at-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
  max-width: 160px;
}

.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 24px;
}

@media (max-width: 768px) {
  .input-search, .select-action, .date-picker {
    width: 100% !important;
  }
}
</style>
