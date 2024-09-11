import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: ['./pkg']
		},
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
});
