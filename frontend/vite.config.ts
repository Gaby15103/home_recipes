import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import path from 'node:path'

export default defineConfig(({ mode }) => {
  // Load env from the parent directory (..) as per your envDir setting
  const env = loadEnv(mode, path.resolve(__dirname, '..'), '')

  // Use 8088 to match your BIND_ADDRESS in .env
  const backendTarget = env.VITE_BACKEND_URL || 'http://localhost:8088'

  return {
    plugins: [vue(), tailwindcss()],
    envDir: path.resolve(__dirname, '..'),
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src'),
      },
    },
    server: {
      proxy: {
        '/api': {
          target: backendTarget,
          changeOrigin: true,
        },
        '/notifications/ws': {
          target: backendTarget.replace(/^http/, 'ws'),
          ws: true,
          changeOrigin: true,
        },
      },
    },
  }
})