<template>
  <div class="page-container">
    <el-card shadow="never" class="table-card">
      <template #header>
        <div class="card-header">
          <span class="title">{{ $t('common.bindings') }}</span>
        </div>
      </template>

      <!-- 绑定表格 -->
      <el-table :data="bindings" v-loading="loading" stripe style="width: 100%;">
        <el-table-column prop="username" :label="$t('users.username')" width="140">
          <template #default="{ row }">
            {{ row.username || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="machine_code" :label="$t('bindings.machineCode')" min-width="300">
          <template #default="{ row }">
            <el-tooltip :content="row.machine_code" placement="top">
              <span class="machine-code">{{ row.machine_code }}</span>
            </el-tooltip>
          </template>
        </el-table-column>
        <el-table-column prop="bound_at" :label="$t('bindings.boundAt')" width="160">
          <template #default="{ row }">
            {{ formatTime(row.bound_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="last_verified_at" :label="$t('bindings.lastVerified')" width="160">
          <template #default="{ row }">
            {{ formatTime(row.last_verified_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="status" :label="$t('common.status')" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="row.status === 'active' ? 'success' : 'info'" size="small">
              {{ row.status === 'active' ? $t('users.status.active') : $t('users.status.disabled') }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="$t('common.operation')" width="100" align="center" fixed="right">
          <template #default="{ row }">
            <el-popconfirm
              :title="$t('bindings.unbindConfirm')"
              :confirm-button-text="$t('common.confirm')"
              :cancel-button-text="$t('common.cancel')"
              @confirm="handleUnbind(row.id)"
            >
              <template #reference>
                <el-button
                  type="danger"
                  link
                  size="small"
                  :disabled="row.status !== 'active'"
                >
                  {{ $t('bindings.unbind') }}
                </el-button>
              </template>
            </el-popconfirm>
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
          @size-change="loadBindings"
          @current-change="loadBindings"
        />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import { getBindings, deleteBinding } from '../api/admin'
import { formatTime } from '../utils/format'
import type { BindingItem } from '../api/admin'

const { t } = useI18n()
const loading = ref(false)
const bindings = ref<BindingItem[]>([])
const total = ref(0)

const queryParams = reactive({
  page: 1,
  size: 20,
})

async function loadBindings() {
  loading.value = true
  try {
    const res = await getBindings({
      page: queryParams.page,
      size: queryParams.size,
    })
    bindings.value = res.data.items
    total.value = res.data.total
  } catch (err) {
    console.error('Failed to load bindings:', err)
  } finally {
    loading.value = false
  }
}

async function handleUnbind(id: number) {
  try {
    await deleteBinding(id)
    ElMessage.success(t('common.success'))
    loadBindings()
  } catch (err) {
    console.error('Unbind failed:', err)
  }
}

onMounted(() => {
  loadBindings()
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
.machine-code {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 13px;
  color: var(--at-secondary);
  background: rgba(0, 198, 255, 0.05);
  padding: 2px 6px;
  border-radius: 4px;
}
.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 24px;
}
</style>
