<template>
  <el-container class="admin-container">
    <div class="app-bg-glow"></div>
    
    <el-aside :width="isCollapse ? '64px' : '240px'" class="sidebar">
      <div class="logo-wrapper">
        <el-icon :size="24" color="#2159ff"><Monitor /></el-icon>
        <h2 v-show="!isCollapse">{{ $t('common.title') }}</h2>
      </div>
      
      <el-menu
        :default-active="activeMenu"
        class="admin-menu"
        :collapse="isCollapse"
        router
      >
        <el-menu-item index="/dashboard">
          <el-icon><Odometer /></el-icon>
          <template #title>{{ $t('common.dashboard') }}</template>
        </el-menu-item>
        <el-menu-item index="/users">
          <el-icon><User /></el-icon>
          <template #title>{{ $t('common.users') }}</template>
        </el-menu-item>
        <el-menu-item index="/bindings">
          <el-icon><Link /></el-icon>
          <template #title>{{ $t('common.bindings') }}</template>
        </el-menu-item>
        <el-menu-item index="/logs">
          <el-icon><Document /></el-icon>
          <template #title>{{ $t('common.logs') }}</template>
        </el-menu-item>
      </el-menu>
    </el-aside>
    
    <el-container class="main-container">
      <el-header class="admin-header">
        <div class="header-left">
          <el-button link @click="isCollapse = !isCollapse">
            <el-icon :size="20">
              <Fold v-if="!isCollapse" />
              <Expand v-else />
            </el-icon>
          </el-button>
        </div>
        
        <div class="header-right">
          <el-dropdown @command="handleLangCommand">
            <el-button link class="lang-toggle">
              <el-icon><Menu /></el-icon>
              {{ currentLangName }}
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="zh">中文</el-dropdown-item>
                <el-dropdown-item command="en">English</el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          
          <div class="user-info">
            <el-avatar :size="32" class="user-avatar">
              <el-icon><User /></el-icon>
            </el-avatar>
            <span class="username">{{ currentUser }}</span>
          </div>
          
          <el-button type="danger" link @click="handleLogout" class="logout-btn">
            <el-icon><SwitchButton /></el-icon>
            <span class="hidden-xs-only">{{ $t('common.logout') }}</span>
          </el-button>
        </div>
      </el-header>
      
      <el-main class="admin-main">
        <router-view v-slot="{ Component }">
          <transition name="fade-transform" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Odometer, User, Link, Document, SwitchButton, Fold, Expand, Monitor, Menu } from '@element-plus/icons-vue'

const route = useRoute()
const router = useRouter()
const { locale } = useI18n()

const isCollapse = ref(false)
const activeMenu = computed(() => route.path)

const currentLangName = computed(() => locale.value === 'zh' ? '中文' : 'EN')

const currentUser = computed(() => {
  try {
    const userStr = localStorage.getItem('user')
    if (userStr) {
      const user = JSON.parse(userStr)
      return user.username || 'Admin'
    }
  } catch { /* localStorage parse error - expected when no user data */ }
  return 'Admin'
})

function handleLogout() {
  localStorage.removeItem('token')
  localStorage.removeItem('user')
  router.push('/login')
}

function handleLangCommand(lang: string) {
  locale.value = lang
  localStorage.setItem('lang', lang)
}
</script>

<style scoped>
.admin-container {
  height: 100vh;
  background-color: var(--at-bg-main);
}

.sidebar {
  background-color: var(--at-bg-sidebar);
  border-right: 1px solid var(--at-border);
  display: flex;
  flex-direction: column;
}

.logo-wrapper {
  height: 64px;
  display: flex;
  align-items: center;
  padding: 0 20px;
  gap: 12px;
  border-bottom: 1px solid var(--at-border);
}

.logo-wrapper h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 700;
  color: #fff;
  white-space: nowrap;
  letter-spacing: 1px;
}

.admin-menu {
  background: transparent;
  flex: 1;
}

.admin-menu :deep(.el-menu-item) {
  height: 50px;
  line-height: 50px;
  margin: 4px 8px;
  border-radius: 8px;
}

.admin-menu :deep(.el-menu-item.is-active) {
  background-color: var(--at-primary) !important;
  color: #fff !important;
  box-shadow: 0 4px 12px var(--at-primary-glow);
}

.admin-header {
  height: 64px;
  background-color: var(--at-bg-header);
  border-bottom: 1px solid var(--at-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.lang-toggle {
  color: var(--at-text-secondary);
  font-weight: 600;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.user-avatar {
  background-color: var(--at-primary);
}

.username {
  font-size: 14px;
  font-weight: 600;
  color: var(--at-text-primary);
}

.logout-btn {
  font-weight: 600;
}

.admin-main {
  padding: 24px;
  overflow-y: auto;
}

/* Transitions */
.fade-transform-enter-active,
.fade-transform-leave-active {
  transition: all 0.3s;
}

.fade-transform-enter-from {
  opacity: 0;
  transform: translateX(-30px);
}

.fade-transform-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

@media (max-width: 768px) {
  .sidebar {
    position: fixed;
    height: 100vh;
    z-index: 1000;
  }
  .sidebar.el-aside {
    width: v-bind("isCollapse ? '0' : '240px'") !important;
    overflow: hidden;
  }
  .hidden-xs-only {
    display: none;
  }
}
</style>
