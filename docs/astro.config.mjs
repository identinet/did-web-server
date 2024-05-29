import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

import starlightOpenAPI, { openAPISidebarGroups } from 'starlight-openapi';

// https://astro.build/config
export default defineConfig({
	site: 'https://dws.identinet.io',
	integrations: [
		starlight({
			title: 'did-web-server',
			favicon: '/favicon.svg',
			customCss: [
				"./src/styles/custom.css"
			],
			head: [
				// Add ICO favicon fallback for Safari.
				{
					tag: 'link',
					attrs: {
						rel: 'icon',
						href: '/favicon.ico',
						sizes: '32x32',
					},
				},
			],
			logo: {
				light: './src/assets/logo-light.svg',
				dark: './src/assets/logo-dark.svg'
			},
			social: {
				github: 'https://github.com/identinet/did-web-server',
				"x.com": 'https://x.com/identinet',
			},
			editLink: {
				baseUrl: 'https://github.com/identinet/did-web-server/tree/main/docs/',
			},
			plugins: [
				// Generate the OpenAPI documentation pages.
				// Documentation: https://starlight-openapi.vercel.app/configuration/
				starlightOpenAPI([
					{
						base: 'api',
						label: 'API',
						schema: 'public/openapi.yaml',
						collapsed: false,
					},
				]),
			],
			sidebar: [
				{ label: 'Introduction', link: '/' },
				{ label: 'Getting Started', link: '/getting-started' },
				{ label: 'Configuration', link: '/configuration' },
				{
					label: 'Deployment',
					autogenerate: { directory: 'deployment' },
				},
				// {
				// 	label: 'DID Management',
				// 	autogenerate: { directory: 'did-management' },
				// },
				{ label: 'Congratulations', link: '/congratulations' },
				...openAPISidebarGroups
			],
		}),
	],
});
