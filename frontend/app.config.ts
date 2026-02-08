import react from '@vitejs/plugin-react'
import tsconfigPaths from 'vite-tsconfig-paths'
import { createApp } from 'vinxi'

export default createApp({
  server: {
    preset: 'node',
  },
  routers: [
    {
      name: 'public',
      type: 'static',
      dir: './public',
    },
    {
      name: 'ssr',
      type: 'http',
      handler: './app/server.tsx',
      target: 'server',
      plugins: () => [react(), tsconfigPaths()],
    },
    {
      name: 'client',
      type: 'client',
      handler: './app/client.tsx',
      target: 'browser',
      plugins: () => [react(), tsconfigPaths()],
      base: '/_build',
    },
  ],
})
