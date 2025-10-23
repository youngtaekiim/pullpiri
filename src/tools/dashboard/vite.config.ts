import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // Avoid read-only node_modules issue by moving cache directory to /tmp
  cacheDir: '/tmp/vite-cache',
  optimizeDeps: {
    // Dependency free bundles can be disabled when node_modules write restrictions occur in a container environment.
    disabled: process.env.VITE_DISABLE_OPTIMIZE_DEPS === '1'
  },
  server: {
    host: true,
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:5174',
        changeOrigin: true
      }
    }
  },
  resolve: {
    alias: { '@': '/src'}
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Split vendor libraries into a separate chunk
          vendor: ['react', 'react-dom']
        }
      }
    },
    chunkSizeWarningLimit: 1000 // Adjust the warning limit to 1000 kB
  }
})
