/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		screens: {
			sm: '480px',
			md: '768px',
			lg: '976px',
			xl: '1440px',
		},
		colors: {
			'primary': '#444c64',
			'secondary': '#e1f2fc',
			'primary-btn': '#ffb247',
			'secondary-btn': '#ced5ee',
			'accent': '#31a1ff',
			'white': '#fff',
			'black': '#000',

			'chess-white': '#fff',
			'chess-black': '#444c64',
			
			'blue': '#1fb6ff',
			'purple': '#7e5bef',
			'pink': '#ff49db',
			'orange': '#ff7849',
			'green': '#13ce66',
			'yellow': '#ffc82c',
			'gray-dark': '#273444',
			'gray': '#8492a6',
			'gray-light': '#d3dce6',
		},
		extend: {
			spacing: {
				'128': '32rem',
				'144': '36rem',
			},
			borderRadius: {
				'4xl': '2rem',
			}
		}
	},
	plugins: [],
	safelist: ["justify-center"],
}
