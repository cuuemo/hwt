<template>
  <div class="bindings-page">
    <el-card shadow="never">
      <template #header>
        <span style="font-weight: 600; font-size: 16px;">机器码绑定管理</span>
      </template>

      <!-- 绑定表格 -->
      <el-table :data="bindings" v-loading="loading" stripe border style="width: 100%;">
        <el-table-column prop="id" label="ID" width="70" align="center" />
        <el-table-column prop="username" label="用户名" width="120">
          <template #default="{ row }">
            {{ row.username || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="machine_code" label="机器码" min-width="260">
          <template #default="{ row }">
            <el-tooltip :content="row.machine_code" placement="top">
              <span class="machine-code">{{ row.machine_code }}</span>
            </el-tooltip>
          </template>
        </el-table-column>
        <el-table-column prop="bound_at" label="绑定时间" width="160">
          <template #default="{ row }">
            {{ formatTime(row.bound_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="last_verified_at" label="最后验证" width="160">
          <template #default="{ row }">
            {{ formatTime(row.last_verified_at) }}
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="row.status === 'active' ? 'success' : 'info'" size="small">
              {{ row.status === 'active' ? '已绑定' : '已解绑' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="120" align="center" fixed="right">
          <template #default="{ row }">
            <el-popconfirm
              title="确定要解绑该机器码吗？"
              confirm-button-text="确定"
              cancel-button-text="取消"
              @confirm="handleUnbind(row.id)"
            >
              <template #reference>
                <el-button
                  type="danger"
                  size="small"
                  :disabled="row.status !== 'active'"
                >
                  解绑
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
import { getBindings, deleteBinding } from '../api/admin'
import type { BindingItem } from '../api/admin'

const loading = ref(false)
const bindings = ref<BindingItem[]>([])
const total = ref(0)

const queryParams = reactive({
  page: 1,
  size: 20,
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
  })
}

async function loadBindings() {
  loading.value = true
  try {
    const res = await getBindings({
      page: queryParams.page,
      size: queryParams.size,
    })
    bindings.value = res.data.items
    total.value = res.data.total
  } catch {
    // handled by interceptor
  } finally {
    loading.value = false
  }
}

async function handleUnbind(id: number) {
  try {
    await deleteBinding(id)
    ElMessage.success('解绑成功')
    loadBindings()
  } catch {
    // handled by interceptor
  }
}

onMounted(() => {
  loadBindings()
})
</script>

<style scoped>
.machine-code {
  font-family: monospace;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: inline-block;
  max-width: 240px;
  vertical-align: middle;
}

.pagination-bar {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
}
</style>
