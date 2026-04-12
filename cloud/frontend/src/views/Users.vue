<template>
  <div class="page-container">
    <el-card shadow="never" class="table-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ $t('common.users') }}</span>
        </div>
      </template>

      <!-- 筛选栏 -->
      <div class="filter-bar">
        <el-select
          v-model="queryParams.status"
          :placeholder="$t('common.status')"
          clearable
          class="filter-item"
          @change="handleSearch"
        >
          <el-option :label="$t('users.status.pending')" value="pending" />
          <el-option :label="$t('users.status.active')" value="active" />
          <el-option :label="$t('users.status.disabled')" value="disabled" />
        </el-select>
        <el-input
          v-model="queryParams.username"
          :placeholder="$t('users.username')"
          clearable
          class="filter-item input-search"
          @clear="handleSearch"
          @keyup.enter="handleSearch"
        />
        <el-button type="primary" class="filter-item" @click="handleSearch">
          <el-icon><Search /></el-icon>
          <span class="hidden-xs-only">{{ $t('common.search') }}</span>
        </el-button>
      </div>

      <!-- 用户表格 -->
      <el-table :data="users" v-loading="loading" stripe style="width: 100%; margin-top: 16px;">
        <el-table-column prop="username" :label="$t('users.username')" min-width="120" />
        <el-table-column prop="email" :label="$t('users.email')" min-width="160" show-overflow-tooltip>
          <template #default="{ row }">
            {{ row.email || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="status" :label="$t('common.status')" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="statusTagType(row.status)" size="small">
              {{ $t(`users.status.${row.status}`) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="license_type" :label="$t('users.license')" width="100" align="center">
          <template #default="{ row }">
            <el-tag v-if="row.license_type" type="info" size="small" effect="plain">
              {{ $t(`users.licenseType.${row.license_type}`) }}
            </el-tag>
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="license_expire_at" :label="$t('users.expire')" min-width="150" show-overflow-tooltip>
          <template #default="{ row }">
            {{ formatTime(row.license_expire_at) }}
          </template>
        </el-table-column>
        <el-table-column :label="$t('common.operation')" width="200" align="center" fixed="right">
          <template #default="{ row }">
            <el-button
              v-if="row.status === 'pending'"
              type="success"
              link
              size="small"
              @click="handleApprove(row)"
            >
              {{ $t('users.approve') }}
            </el-button>
            <el-button
              v-if="row.status === 'active'"
              type="warning"
              link
              size="small"
              @click="handleDisable(row)"
            >
              {{ $t('users.disable') }}
            </el-button>
            <el-button
              v-if="row.status === 'disabled'"
              type="success"
              link
              size="small"
              @click="handleEnable(row)"
            >
              {{ $t('users.enable') }}
            </el-button>
            <el-button
              type="primary"
              link
              size="small"
              @click="openEditDialog(row)"
            >
              {{ $t('users.edit') }}
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
    <el-dialog v-model="editDialogVisible" :title="$t('users.edit')" width="500px" custom-class="at-dialog">
      <el-form :model="editForm" label-width="100px" label-position="top">
        <el-form-item :label="$t('users.username')">
          <el-input :model-value="editForm.username" disabled />
        </el-form-item>
        <el-form-item :label="$t('users.email')">
          <el-input v-model="editForm.email" />
        </el-form-item>
        <el-form-item :label="$t('common.status')">
          <el-select v-model="editForm.status" style="width: 100%;">
            <el-option :label="$t('users.status.pending')" value="pending" />
            <el-option :label="$t('users.status.active')" value="active" />
            <el-option :label="$t('users.status.disabled')" value="disabled" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('users.license')">
          <el-select v-model="editForm.license_type" clearable style="width: 100%;">
            <el-option :label="$t('users.licenseType.monthly')" value="monthly" />
            <el-option :label="$t('users.licenseType.yearly')" value="yearly" />
            <el-option :label="$t('users.licenseType.permanent')" value="permanent" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('users.expire')">
          <el-date-picker
            v-model="editForm.license_expire_at"
            type="datetime"
            style="width: 100%;"
            format="YYYY-MM-DD HH:mm:ss"
            value-format="YYYY-MM-DDTHH:mm:ss"
          />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" :loading="editLoading" @click="handleSaveEdit">{{ $t('common.confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Search } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'
import { getUsers, updateUser } from '../api/admin'
import { formatTime } from '../utils/format'
import type { UserItem } from '../api/admin'

const { t } = useI18n()
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
  } catch (err) {
    console.error('Failed to load users:', err)
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
    ElMessage.success(t('common.success'))
    loadUsers()
  } catch (err) {
    console.error('Approve failed:', err)
  }
}

async function handleDisable(row: UserItem) {
  try {
    await updateUser(row.id, { status: 'disabled' })
    ElMessage.success(t('common.success'))
    loadUsers()
  } catch (err) {
    console.error('Disable failed:', err)
  }
}

async function handleEnable(row: UserItem) {
  try {
    await updateUser(row.id, { status: 'active' })
    ElMessage.success(t('common.success'))
    loadUsers()
  } catch (err) {
    console.error('Enable failed:', err)
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
    if (editForm.license_type) data.license_type = editForm.license_type
    data.license_expire_at = editForm.license_expire_at || null
    await updateUser(editForm.id, data)
    ElMessage.success(t('common.success'))
    editDialogVisible.value = false
    loadUsers()
  } catch (err) {
    console.error('Save failed:', err)
  } finally {
    editLoading.value = false
  }
}

onMounted(() => {
  loadUsers()
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
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
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
.filter-item {
  margin: 0;
}
.input-search {
  width: 240px;
}
.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 24px;
}

@media (max-width: 768px) {
  .input-search {
    width: 100%;
  }
}
</style>
