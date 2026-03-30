<template>
  <div class="users-page">
    <el-card shadow="never">
      <template #header>
        <div style="display: flex; align-items: center; justify-content: space-between;">
          <span style="font-weight: 600; font-size: 16px;">用户管理</span>
        </div>
      </template>

      <!-- 筛选栏 -->
      <div class="filter-bar">
        <el-select
          v-model="queryParams.status"
          placeholder="状态筛选"
          clearable
          style="width: 140px;"
          @change="handleSearch"
        >
          <el-option label="待审核" value="pending" />
          <el-option label="已激活" value="active" />
          <el-option label="已禁用" value="disabled" />
        </el-select>
        <el-input
          v-model="queryParams.username"
          placeholder="搜索用户名"
          clearable
          style="width: 200px; margin-left: 12px;"
          @clear="handleSearch"
          @keyup.enter="handleSearch"
        />
        <el-button type="primary" style="margin-left: 12px;" @click="handleSearch">
          <el-icon style="margin-right: 4px;"><Search /></el-icon>
          搜索
        </el-button>
      </div>

      <!-- 用户表格 -->
      <el-table :data="users" v-loading="loading" stripe border style="width: 100%; margin-top: 16px;">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column prop="username" label="用户名" width="120" />
        <el-table-column prop="email" label="邮箱" min-width="160">
          <template #default="{ row }">
            {{ row.email || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="role" label="角色" width="80" align="center">
          <template #default="{ row }">
            <el-tag :type="row.role === 'admin' ? 'danger' : ''" size="small">
              {{ row.role === 'admin' ? '管理员' : '用户' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="statusTagType(row.status)" size="small">
              {{ statusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="license_type" label="授权类型" width="100" align="center">
          <template #default="{ row }">
            {{ licenseLabel(row.license_type) }}
          </template>
        </el-table-column>
        <el-table-column prop="license_expire_at" label="到期时间" width="160">
          <template #default="{ row }">
            {{ formatTime(row.license_expire_at) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="240" align="center" fixed="right">
          <template #default="{ row }">
            <el-button
              v-if="row.status === 'pending'"
              type="success"
              size="small"
              @click="handleApprove(row)"
            >
              审核通过
            </el-button>
            <el-button
              v-if="row.status === 'active'"
              type="warning"
              size="small"
              @click="handleDisable(row)"
            >
              禁用
            </el-button>
            <el-button
              v-if="row.status === 'disabled'"
              type="success"
              size="small"
              @click="handleEnable(row)"
            >
              启用
            </el-button>
            <el-button
              type="primary"
              size="small"
              @click="openEditDialog(row)"
            >
              编辑
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <div class="pagination-bar">
        <el-pagination
          v-model:current-page="queryParams.page"
          v-model:page-size="queryParams.size"
          :total="total"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="loadUsers"
          @current-change="loadUsers"
        />
      </div>
    </el-card>

    <!-- 编辑弹窗 -->
    <el-dialog v-model="editDialogVisible" title="编辑用户" width="500px" destroy-on-close>
      <el-form :model="editForm" label-width="100px">
        <el-form-item label="用户名">
          <el-input :model-value="editForm.username" disabled />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="editForm.email" placeholder="邮箱地址" />
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="editForm.status" style="width: 100%;">
            <el-option label="待审核" value="pending" />
            <el-option label="已激活" value="active" />
            <el-option label="已禁用" value="disabled" />
          </el-select>
        </el-form-item>
        <el-form-item label="授权类型">
          <el-select v-model="editForm.license_type" clearable placeholder="请选择" style="width: 100%;">
            <el-option label="月付" value="monthly" />
            <el-option label="年付" value="yearly" />
            <el-option label="永久" value="permanent" />
          </el-select>
        </el-form-item>
        <el-form-item label="到期时间">
          <el-date-picker
            v-model="editForm.license_expire_at"
            type="datetime"
            placeholder="选择到期时间"
            style="width: 100%;"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DDTHH:mm:ss"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" :loading="editLoading" @click="handleSaveEdit">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Search } from '@element-plus/icons-vue'
import { getUsers, updateUser } from '../api/admin'
import type { UserItem } from '../api/admin'

const loading = ref(false)
const users = ref<UserItem[]>([])
const total = ref(0)

const queryParams = reactive({
  page: 1,
  size: 20,
  status: '' as string,
  username: '',
})

const editDialogVisible = ref(false)
const editLoading = ref(false)
const editForm = reactive({
  id: 0,
  username: '',
  email: '',
  status: '',
  license_type: '' as string,
  license_expire_at: '' as string,
})

function statusTagType(status: string): string {
  const map: Record<string, string> = {
    pending: 'warning',
    active: 'success',
    disabled: 'danger',
  }
  return map[status] || 'info'
}

function statusLabel(status: string): string {
  const map: Record<string, string> = {
    pending: '待审核',
    active: '已激活',
    disabled: '已禁用',
  }
  return map[status] || status
}

function licenseLabel(type: string | null): string {
  if (!type) return '-'
  const map: Record<string, string> = {
    monthly: '月付',
    yearly: '年付',
    permanent: '永久',
  }
  return map[type] || type
}

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

async function loadUsers() {
  loading.value = true
  try {
    const params: Record<string, any> = {
      page: queryParams.page,
      size: queryParams.size,
    }
    if (queryParams.status) params.status = queryParams.status
    if (queryParams.username) params.username = queryParams.username

    const res = await getUsers(params)
    users.value = res.data.items
    total.value = res.data.total
  } catch {
    // handled by interceptor
  } finally {
    loading.value = false
  }
}

function handleSearch() {
  queryParams.page = 1
  loadUsers()
}

async function handleApprove(row: UserItem) {
  try {
    await updateUser(row.id, { status: 'active' })
    ElMessage.success('审核通过')
    loadUsers()
  } catch {
    // handled by interceptor
  }
}

async function handleDisable(row: UserItem) {
  try {
    await updateUser(row.id, { status: 'disabled' })
    ElMessage.success('已禁用')
    loadUsers()
  } catch {
    // handled by interceptor
  }
}

async function handleEnable(row: UserItem) {
  try {
    await updateUser(row.id, { status: 'active' })
    ElMessage.success('已启用')
    loadUsers()
  } catch {
    // handled by interceptor
  }
}

function openEditDialog(row: UserItem) {
  editForm.id = row.id
  editForm.username = row.username
  editForm.email = row.email || ''
  editForm.status = row.status
  editForm.license_type = row.license_type || ''
  editForm.license_expire_at = row.license_expire_at || ''
  editDialogVisible.value = true
}

async function handleSaveEdit() {
  editLoading.value = true
  try {
    const data: Record<string, any> = {
      status: editForm.status,
      email: editForm.email || undefined,
    }
    if (editForm.license_type) {
      data.license_type = editForm.license_type
    }
    if (editForm.license_expire_at) {
      data.license_expire_at = editForm.license_expire_at
    } else {
      data.license_expire_at = null
    }
    await updateUser(editForm.id, data)
    ElMessage.success('保存成功')
    editDialogVisible.value = false
    loadUsers()
  } catch {
    // handled by interceptor
  } finally {
    editLoading.value = false
  }
}

onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.filter-bar {
  display: flex;
  align-items: center;
}

.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
</style>
