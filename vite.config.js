import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [sveltekit()],
	
	clearScreen: false,
	
	server: {
		host: host || '127.0.0.1',
		port: 5173,
		strictPort: true,
		hmr: host
			? {
					protocol: 'ws',
					host: host,
					port: 5183
				}
			: undefined,
		fs: {
			allow: ['..']
		},
		watch: {
			ignored: ['**/src-tauri/**']
		}
	},
	
	envPrefix: ['VITE_', 'TAURI_'],
	
	build: {
		target: ['es2021', 'chrome100', 'safari13'],
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_DEBUG,
		rollupOptions: {
			output: {
				manualChunks: {
					vendor: ['svelte']
				}
			}
		},
		cssCodeSplit: true,
		chunkSizeWarningLimit: 1000,
		reportCompressedSize: false
	},
	
	optimizeDeps: {
		include: ['@tauri-apps/api', '@tauri-apps/plugin-shell'],
		exclude: []
	},
	
	resolve: {
		alias: {
			$lib: '/src/lib'
		}
	},
	
	define: {
		__APP_VERSION__: JSON.stringify(process.env.npm_package_version)
	}
});