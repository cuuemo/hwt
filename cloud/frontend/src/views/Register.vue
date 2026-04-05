<template>
  <div class="auth-bg">
    <div class="auth-card">
      <div class="auth-logo">
        <span class="logo-icon">🛡</span>
        <h1>注册账号</h1>
      </div>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="0" @keyup.enter="handleRegister">
        <el-form-item prop="username">
          <el-input v-model="form.username" placeholder="用户名" :prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item prop="email">
          <el-input v-model="form.email" placeholder="邮箱（选填）" :prefix-icon="Message" size="large" />
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" placeholder="密码" :prefix-icon="Lock" size="large" show-password />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="large" style="width:100%" :loading="loading" @click="handleRegister">
            注 册
          </el-button>
        </el-form-item>
      </el-form>
      <div class="auth-footer">
        已有账号？<router-link to="/login">立即登录</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { User, Lock, Message } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { register } from '../api/auth'

const router = useRouter()
const formRef = ref<FormInstance>()
const loading = ref(false)
const form = reactive({ username: '', password: '', email: '' })

const rules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 2, max: 64, message: '用户名长度为 2-64 个字符', trigger: 'blur' },
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少 6 个字符', trigger: 'blur' },
  ],
}

async function handleRegister() {
  if (!formRef.value) return
  const valid = await formRef.value.validate().catch(() => false)
  if (!valid) return
  loading.value = true
  try {
    await register(form.username, form.password, form.email || undefined)
    ElMessage.success('注册成功，请等待管理员审核激活')
    router.push('/login')
  } catch {
    // handled by interceptor
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.auth-bg {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);
}
.auth-card {
  width: 400px;
  padding: 48px 40px 36px;
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 16px;
  backdrop-filter: blur(20px);
  box-shadow: 0 8px 32px rgba(0,0,0,0.4);
}
.auth-logo {
  text-align: center;
  margin-bottom: 36px;
}
.logo-icon { font-size: 40px; }
.auth-logo h1 {
  margin: 8px 0 0;
  font-size: 22px;
  font-weight: 600;
  color: #e2e8f0;
  letter-spacing: 1px;
}
.auth-footer {
  text-align: center;
  margin-top: 16px;
  font-size: 14px;
  color: #94a3b8;
}
.auth-footer a { color: #60a5fa; text-decoration: none; }
.auth-footer a:hover { text-decoration: underline; }
:deep(.el-input__wrapper) {
  background: rgba(255,255,255,0.08) !important;
  border: 1px solid rgba(255,255,255,0.15) !important;
  box-shadow: none !important;
}
:deep(.el-input__inner) { color: #e2e8f0 !important; }
:deep(.el-input__inner::placeholder) { color: #64748b !important; }
:deep(.el-input__prefix-inner .el-icon) { color: #64748b; }
</style>
