import type { Config } from 'tailwindcss';
import flowbitePlugin from 'flowbite/plugin';
import flowbiteTypography from 'flowbite-typography';
import yaasmThem from '../tailwind.config';

export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	theme: {
		extend: {
			...yaasmThem.theme.extend
		}
	},
	darkMode: ['selector', '[data-theme=dark]'],
	plugins: [flowbiteTypography, flowbitePlugin]
} satisfies Config;
