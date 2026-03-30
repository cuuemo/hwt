<template>
  <div class="dashboard">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #409eff;">
              <el-icon :size="28"><User /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.totalUsers }}</div>
              <div class="stat-label">总用户数</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #67c23a;">
              <el-icon :size="28"><CircleCheck /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.activeUsers }}</div>
              <div class="stat-label">活跃用户</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #e6a23c;">
              <el-icon :size="28"><Checked /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.todayLogs }}</div>
              <div class="stat-label">今日验证</div>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card shadow="hover" class="stat-card">
          <div class="stat-content">
            <div class="stat-icon" style="background-color: #f56c6c;">
              <el-icon :size="28"><Connection /></el-icon>
            </div>
            <div class="stat-info">
              <div class="stat-value">{{ stats.activeBindings }}</div>
              <div class="stat-label">在线绑定</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="12">
        <el-card shadow="hover">
          <template #header>
            <span style="font-weight: 600;">最近待审核用户</span>
          </template>
          <el-table :data="pendingUsers" stripe size="small" style="width: 100%">
            <el-table-column prop="username" label="用户名" />
            <el-table-column prop="email" label="邮箱" />
            <el-table-column prop="created_at" label="注册时间">
              <template #default="{ row }">
                {{ formatTime(row.created_at) }}
              </template>
            </el-table-column>
          </el-table>
          <div v-if="pendingUsers.length === 0" style="text-align: center; color: #999; padding: 20px;">
            暂无待审核用户
          </div>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card shadow="hover">
          <template #header>
            <span style="font-weight: 600;">最近操作日志</span>
          </template>
          <el-table :data="recentLogs" stripe size="small" style="width: 100%">
            <el-table-column prop="username" label="用户" width="100" />
            <el-table-column prop="action" label="操作" width="80">
              <template #default="{ row }">
                <el-tag :type="actionTagType(row.action)" size="small">{{ actionLabel(row.action) }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="result" label="结果" width="80">
              <template #default="{ row }">
                <el-tag :type="row.result === 'success' ? 'success' : 'danger'" size="small">
                  {{ row.result === 'success' ? '成功' : '失败' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="created_at" label="时间">
              <template #default="{ row }">
                {{ formatTime(row.created_at) }}
              </template>
            </el-table-column>
          </el-table>
          <div v-if="recentLogs.length === 0" style="text-align: center; color: #999; padding: 20px;">
            暂无日志记录
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
import type { UserItem, LogItem } from '../api/admin'

const stats = reactive({
  totalUsers: 0,
  activeUsers: 0,
  todayLogs: 0,
  activeBindings: 0,
})

const pendingUsers = ref<UserItem[]>([])
const recentLogs = ref<LogItem[]>([])

function formatTime(time: string | null): string {
  if (!time) return '-'
  const d = new Date(time)
  return d.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
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

async function loadStats() {
  try {
    const [usersRes, activeRes, bindingsRes, logsRes, pendingRes] = await Promise.all([
      getUsers({ page: 1, size: 1 }),
      getUsers({ page: 1, size: 1, status: 'active' }),
      getBindings({ page: 1, size: 1 }),
      getLogs({ page: 1, size: 10 }),
      getUsers({ page: 1, size: 5, status: 'pending' }),
    ])

    stats.totalUsers = usersRes.data.total
    stats.activeUsers = activeRes.data.total
    stats.activeBindings = bindingsRes.data.total
    pendingUsers.value = pendingRes.data.items
    recentLogs.value = logsRes.data.items

    // 今日验证数：获取今天的日志总数
    const today = new Date()
    const dateStr = `${today.getFullYear()}-${String(today.getMonth() + 1).padStart(2, '0')}-${String(today.getDate()).padStart(2, '0')}`
    const todayLogsRes = await getLogs({ page: 1, size: 1, date_from: dateStr, date_to: dateStr })
    stats.todayLogs = todayLogsRes.data.total
  } catch {
    // errors handled by interceptor
  }
}

onMounted(() => {
  loadStats()
})
</script>

<style scoped>
.stat-card {
  height: 120px;
}

.stat-card :deep(.el-card__body) {
  padding: 20px;
  height: 100%;
  box-sizing: border-box;
  display: flex;
  align-items: center;
}

.stat-content {
  display: flex;
  align-items: center;
  width: 100%;
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;
}

.stat-info {
  margin-left: 16px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: #303133;
  line-height: 1.2;
}

.stat-label {
  font-size: 14px;
  color: #909399;
  margin-top: 4px;
}
</style>
