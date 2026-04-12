<template>
  <div class="auth-bg">
    <div class="app-bg-glow"></div>
    <div class="auth-card">
      <div class="auth-logo">
        <el-icon :size="48" color="#2159ff"><Monitor /></el-icon>
        <h1>{{ $t('common.title') }}</h1>
        <p class="text-secondary">{{ $t('auth.register') }}</p>
      </div>
      
      <el-form ref="formRef" :model="form" :rules="rules" label-width="0" @keyup.enter="handleRegister">
        <el-form-item prop="username">
          <el-input v-model="form.username" :placeholder="$t('auth.username')" :prefix-icon="User" size="large" />
        </el-form-item>
        <el-form-item prop="email">
          <el-input v-model="form.email" :placeholder="$t('auth.email')" :prefix-icon="Message" size="large" />
        </el-form-item>
        <el-form-item prop="password">
          <el-input v-model="form.password" type="password" :placeholder="$t('auth.password')" :prefix-icon="Lock" size="large" show-password />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" size="large" class="w-full" :loading="loading" @click="handleRegister">
            {{ $t('auth.register') }}
          </el-button>
        </el-form-item>
      </el-form>
      
      <div class="auth-footer">
        {{ $t('auth.hasAccount') }} <router-link to="/login">{{ $t('auth.login') }}</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import { User, Lock, Message, Monitor } from '@element-plus/icons-vue'
import type { FormInstance, FormRules } from 'element-plus'
import { register } from '../api/auth'

const router = useRouter()
const { t } = useI18n()
const formRef = ref<FormInstance>()
const loading = ref(false)
const form = reactive({ username: '', password: '', email: '' })

const rules = computed<FormRules>(() => ({
  username: [
    { required: true, message: t('auth.pleaseEnterUsername'), trigger: 'blur' },
    { min: 2, max: 64, trigger: 'blur' },
  ],
  password: [
    { required: true, message: t('auth.pleaseEnterPassword'), trigger: 'blur' },
    { min: 6, trigger: 'blur' },
  ],
}))

async function handleRegister() {
  if (!formRef.value) return
  const valid = await formRef.value.validate().catch(() => false)
  if (!valid) return
  loading.value = true
  try {
    await register(form.username, form.password, form.email || undefined)
    ElMessage.success(t('auth.regSuccess'))
    router.push('/login')
  } catch (err) {
    console.error('Registration failed:', err)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
@import "../styles/auth.css";
</style>
