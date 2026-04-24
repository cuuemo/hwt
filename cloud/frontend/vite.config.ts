import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// Note: rollup-obfuscator was removed — it silently dropped dynamic-import
// chunks (Login/Users/ClientLogs/AdminLayout) from the production build
// and mangled Vue's mount target. If obfuscation is needed again, evaluate
// a different tool (javascript-obfuscator webpack-style, or a postbuild
// step that runs after Rollup) rather than a rollup plugin that interferes
// with chunk graph generation.
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://159.195.77.25:10000',
        changeOrigin: true,
      },
    },
  },
})
