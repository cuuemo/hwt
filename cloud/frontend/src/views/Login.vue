<template>
  <div class="auth-bg">
    <div class="app-bg-glow"></div>
    <div class="auth-card">
      <div class="auth-logo">
        <el-icon :size="48" color="#2159ff"><Monitor /></el-icon>
        <h1>{{ $t('common.title') }}</h1>
        <p class="text-secondary">{{ $t('auth.login') }}</p>
      </div>
      
      <el-form ref="formRef" :model="form" :rules="rules" label-width="0" @keyup.enter="handleLogin">
        <el-form-item prop="username">
          <el-input v-model="form.username" :placeholder="$t('auth.username')" :prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" :placeholder="$t('auth.password')" :prefix-icon="Lock" size="large" show-password />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="large" class="w-full" :loading="loading" @click="handleLogin">
            {{ $t('auth.login') }}
          </el-button>
        </el-form-item>
      </el-form>
      
      <div class="auth-footer">
        {{ $t('auth.noAccount') }} <router-link to="/register">{{ $t('auth.register') }}</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import { User, Lock, Monitor } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { login } from '../api/auth'

const router = useRouter()
const { t } = useI18n()
const formRef = ref<FormInstance>()
const loading = ref(false)
const form = reactive({ username: '', password: '' })

const rules = computed<FormRules>(() => ({
  username: [{ required: true, message: t('auth.pleaseEnterUsername'), trigger: 'blur' }],
  password: [{ required: true, message: t('auth.pleaseEnterPassword'), trigger: 'blur' }],
}))

async function handleLogin() {
  if (!formRef.value) return
  const valid = await formRef.value.validate().catch(() => false)
  if (!valid) return
  loading.value = true
  try {
    const result = await login(form.username, form.password)
    localStorage.setItem('token', result.access_token)
    localStorage.setItem('user', JSON.stringify(result.user))
    ElMessage.success(t('auth.loginSuccess'))
    router.push('/dashboard')
  } catch (err: unknown) {
    // Server errors already get a toast from the response interceptor; only
    // surface client-side failures (network, crypto.subtle missing, etc.) —
    // otherwise the button goes back to idle with no feedback at all.
    const e = err as { response?: unknown; message?: string }
    if (!e.response) {
      ElMessage.error(`${t('auth.loginFailed')}: ${e.message || String(err)}`)
    }
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
@import "../styles/auth.css";
</style>
