import catppuccin from '@catppuccin/tailwindcss';
/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		fontFamily: {
			mono: ['Jetbrains Mono', 'monospace']
		},
		extend: {}
	},
	plugins: [catppuccin()]
};
