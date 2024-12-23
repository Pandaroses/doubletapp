import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm'; 

export default defineConfig({
	plugins: [sveltekit(),wasm(),],
	server: {
		fs: {
			allow: ['./pkg']
		},
		proxy: {
			'/api': {
				target: 'http://0.0.0.0:3000',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
});
