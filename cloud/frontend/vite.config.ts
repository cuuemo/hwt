import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { obfuscator } from 'rollup-obfuscator'
import path from 'path'

export default defineConfig(({ mode }) => ({
  plugins: [
    vue(),
    ...(mode === 'production'
      ? [
          obfuscator({
            include: ['**/src/**/*.{ts,js,vue}'],
            exclude: ['**/src/main.ts'],
            compact: true,
            stringArray: true,
            stringArrayEncoding: ['base64'],
            stringArrayThreshold: 0.6,
            disableConsoleOutput: true,
          }),
        ]
      : []),
  ],
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
}))
