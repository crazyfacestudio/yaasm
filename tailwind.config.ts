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
				primary: '#acc954',
				'primary-light': '#d6f47b',
				secondary: '#2a365c',
				'secondary-light': '#5b6db3',
				alert: '#d50707',
				'alert-light': '#f99090',
				yaasm: {
					100: '#212121',
					200: '#1f1f1f',
					300: '#1d1d1d',
					400: '#1b1b1b',
					500: '#1a1a1a',
					600: '#181818',
					700: '#161616',
					800: '#141414',
					900: '#121212'
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
