import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";


export default defineConfig({
	assetsInclude: ['**/*.wasm'],
	plugins: [sveltekit(), wasm(), topLevelAwait()],
	server: {
		port: 3000,
		fs: {
			allow: ['..'], // allow serving files from parent dirs
		}
	}
});
