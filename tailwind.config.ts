import type { Config } from 'tailwindcss';
import flowbitePlugin from 'flowbite/plugin';
import flowbiteTypography from 'flowbite-typography';
// import typography from '@tailwindcss/typography';

export default {
	content: [
		'./src/**/*.{html,js,svelte,ts,postcss}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte-icons/**/*.{html,js,svelte,ts}'
	],

	// make sure to safelist these classes when using purge
	safelist: [
		'w-64',
		'w-1/2',
		'rounded-l-lg',
		'rounded-r-lg',
		'bg-gray-200',
		'grid-cols-4',
		'grid-cols-7',
		'h-6',
		'leading-6',
		'h-9',
		'leading-9',
		'shadow-lg'
	],

	theme: {
		extend: {
			colors: {
				primary: {
					100: '#65e0fb',
					200: '#53d7fb',
					300: '#42cffc',
					400: '#2fc5fc',
					500: '#1cbcfc',
					600: '#08b2fc',
					700: '#00a9fb',
					800: '#0b9ef9',
					900: '#1e94f6'
				},
				yaasm: {
					100: '#2d394c',
					200: '#2a3649',
					300: '#283345',
					400: '#263041',
					500: '#232d3d',
					600: '#212a39',
					700: '#1e2835',
					800: '#1c2531',
					900: '#1a222d'
				},
				premium: {
					100: '#ffd933',
					200: '#e7c22c',
					300: '#ceac25',
					400: '#b6961f',
					500: '#9f8218',
					600: '#886d12',
					700: '#715a0c',
					800: '#5b4706',
					900: '#463501'
				},
				discord: { DEFAULT: '#5865F2', alt: '#848ef5' },
				alerts: {
					'warning-alt': '#f4a261'
				}
			}
		}
	},
	darkMode: ['selector', '[data-theme=dark]'],
	plugins: [flowbiteTypography, flowbitePlugin]
	// flowbiteConfig: { charts: true, datatables: true }
} satisfies Config;
