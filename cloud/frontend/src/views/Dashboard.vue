<template>
  <div class="dashboard-container">
    <el-row :gutter="20">
      <el-col :xs="24" :sm="12" :lg="6" class="mb-20">
        <el-card shadow="hover" class="stat-card primary">
          <div class="stat-content">
            <div class="stat-info">
              <div class="stat-label">{{ $t('dashboard.totalUsers') }}</div>
              <div class="stat-value">{{ stats.totalUsers }}</div>
            </div>
            <div class="stat-icon-wrapper">
              <el-icon :size="32"><User /></el-icon>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :xs="24" :sm="12" :lg="6" class="mb-20">
        <el-card shadow="hover" class="stat-card success">
          <div class="stat-content">
            <div class="stat-info">
              <div class="stat-label">{{ $t('dashboard.activeUsers') }}</div>
              <div class="stat-value">{{ stats.activeUsers }}</div>
            </div>
            <div class="stat-icon-wrapper">
              <el-icon :size="32"><CircleCheck /></el-icon>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :xs="24" :sm="12" :lg="6" class="mb-20">
        <el-card shadow="hover" class="stat-card warning">
          <div class="stat-content">
            <div class="stat-info">
              <div class="stat-label">{{ $t('dashboard.todayLogs') }}</div>
              <div class="stat-value">{{ stats.todayLogs }}</div>
            </div>
            <div class="stat-icon-wrapper">
              <el-icon :size="32"><Checked /></el-icon>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :xs="24" :sm="12" :lg="6" class="mb-20">
        <el-card shadow="hover" class="stat-card danger">
          <div class="stat-content">
            <div class="stat-info">
              <div class="stat-label">{{ $t('dashboard.activeBindings') }}</div>
              <div class="stat-value">{{ stats.activeBindings }}</div>
            </div>
            <div class="stat-icon-wrapper">
              <el-icon :size="32"><Connection /></el-icon>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-20">
      <el-col :xs="24" :lg="12" class="mb-20">
        <el-card shadow="hover" class="data-card">
          <template #header>
            <div class="card-header">
              <span>{{ $t('dashboard.pendingUsers') }}</span>
              <el-button type="primary" link @click="$router.push('/users')">{{ $t('common.operation') }}</el-button>
            </div>
          </template>
          <el-table :data="pendingUsers" stripe style="width: 100%">
            <el-table-column prop="username" :label="$t('users.username')" />
            <el-table-column prop="email" :label="$t('users.email')" show-overflow-tooltip />
            <el-table-column prop="created_at" :label="$t('users.regTime')">
              <template #default="{ row }">
                {{ formatTime(row.created_at) }}
              </template>
            </el-table-column>
          </el-table>
          <div v-if="pendingUsers.length === 0" class="empty-data">
            {{ $t('dashboard.noPending') }}
          </div>
        </el-card>
      </el-col>
      <el-col :xs="24" :lg="12" class="mb-20">
        <el-card shadow="hover" class="data-card">
          <template #header>
            <div class="card-header">
              <span>{{ $t('dashboard.recentLogs') }}</span>
              <el-button type="primary" link @click="$router.push('/logs')">{{ $t('common.operation') }}</el-button>
            </div>
          </template>
          <el-table :data="recentLogs" stripe style="width: 100%">
            <el-table-column prop="username" :label="$t('logs.user')" width="100" />
            <el-table-column prop="action" :label="$t('logs.action')" width="100">
              <template #default="{ row }">
                <el-tag :type="actionTagType(row.action)" size="small">
                  {{ $t(`logs.actions.${row.action}`) }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="result" :label="$t('logs.result')" width="100">
              <template #default="{ row }">
                <el-tag :type="row.result === 'success' ? 'success' : 'danger'" size="small">
                  {{ row.result === 'success' ? $t('common.success') : $t('common.failed') }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="created_at" :label="$t('common.time')">
              <template #default="{ row }">
                {{ formatTime(row.created_at) }}
              </template>
            </el-table-column>
          </el-table>
          <div v-if="recentLogs.length === 0" class="empty-data">
            {{ $t('dashboard.noLogs') }}
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { User, CircleCheck, Checked, Connection } from '@element-plus/icons-vue'
import { getUsers, getBindings, getLogs } from '../api/admin'
import { formatTime, actionTagType } from '../utils/format'
import type { UserItem, LogItem } from '../api/admin'

const stats = reactive({
  totalUsers: 0,
  activeUsers: 0,
  todayLogs: 0,
  activeBindings: 0,
})

const pendingUsers = ref<UserItem[]>([])
const recentLogs = ref<LogItem[]>([])

async function loadStats() {
  try {
    const [usersRes, activeRes, bindingsRes, logsRes, pendingRes] = await Promise.all([
      getUsers({ page: 1, size: 1 }),
      getUsers({ page: 1, size: 1, status: 'active' }),
      getBindings({ page: 1, size: 1 }),
      getLogs({ page: 1, size: 8 }),
      getUsers({ page: 1, size: 8, status: 'pending' }),
    ])

    stats.totalUsers = usersRes.data.total
    stats.activeUsers = activeRes.data.total
    stats.activeBindings = bindingsRes.data.total
    pendingUsers.value = pendingRes.data.items
    recentLogs.value = logsRes.data.items

    const today = new Date()
    const dateStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`
    const todayLogsRes = await getLogs({ page: 1, size: 1, date_from: dateStr, date_to: dateStr })
    stats.todayLogs = todayLogsRes.data.total
  } catch (err) {
    console.error('Failed to load stats:', err)
  }
}

onMounted(() => {
  loadStats()
})
</script>

<style scoped>
.mb-20 { margin-bottom: 20px; }

.stat-card {
  border: none;
  background: var(--at-bg-card);
  position: relative;
  overflow: hidden;
}

.stat-card::after {
  content: "";
  position: absolute;
  top: -20px;
  right: -20px;
  width: 100px;
  height: 100px;
  background: currentColor;
  opacity: 0.05;
  border-radius: 50%;
}

.stat-card.primary { color: #409eff; border-left: 4px solid #409eff; }
.stat-card.success { color: #67c23a; border-left: 4px solid #67c23a; }
.stat-card.warning { color: #e6a23c; border-left: 4px solid #e6a23c; }
.stat-card.danger { color: #f56c6c; border-left: 4px solid #f56c6c; }

.stat-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.stat-label {
  font-size: 14px;
  color: var(--at-text-secondary);
  margin-bottom: 8px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--at-text-primary);
}

.stat-icon-wrapper {
  background: rgba(255, 255, 255, 0.05);
  width: 56px;
  height: 56px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: 600;
}

.empty-data {
  text-align: center;
  color: var(--at-text-secondary);
  padding: 40px 0;
}

.data-card :deep(.el-table) {
  background: transparent !important;
}
</style>
