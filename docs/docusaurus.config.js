// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import { themes as prismThemes } from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'did-web-server',
  tagline: 'did:web for identities',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://dws.identinet.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'identinet', // Usually your GitHub org/user name.
  projectName: 'did-web-server', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      // '@docusaurus/preset-classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: 'https://github.com/identinet/did-web-server/tree/main/docs/',
        },
        // blog: {
        //   showReadingTime: true,
        //   // Please change this to your repo.
        //   // Remove this to remove the "edit this page" links.
        //   editUrl:
        //     'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
        // },
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
    // Check out: https://github.com/cloud-annotations/docusaurus-openapi
    // [
    //   "docusaurus-preset-openapi",
    //   /** @type {import('docusaurus-preset-openapi').Options} */
    //   ({
    //     api: {
    //       path: "openapi/openapi.yaml",
    //       routeBasePath: "/api",
    //     },
    //     docs: {
    //       sidebarPath: './sidebars.js',
    //       // Please change this to your repo.
    //       // Remove this to remove the "edit this page" links.
    //       editUrl:
    //         'https://github.com/identinet/did-web-server/tree/main/docs/',
    //     },
    //     theme: {
    //       customCss: './src/css/custom.css',
    //     },
    //   })
    // ],
    [
      // Redocusaurus config
      'redocusaurus',
      /** @type {import('redocusaurus').PresetOptions} */
      ({
        // Plugin Options for loading OpenAPI files
        specs: [
          // Pass it a path to a local OpenAPI YAML file
          {
            // Redocusaurus will automatically bundle your spec into a single file during the build
            spec: 'openapi/openapi.yaml',
            route: '/api',
          },
          // You can also pass it a OpenAPI spec URL
          // {
          //   spec: 'https://redocly.github.io/redoc/openapi.yaml',
          //   route: '/openapi/',
          // },
        ],
        // Theme Options for modifying how redoc renders them
        theme: {
          // Change with your site colors
          primaryColor: '#039BE5',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/docusaurus-social-card.jpg',
      navbar: {
        title: 'did-web-server',
        logo: {
          alt: 'identinet logo',
          src: 'img/logo.svg',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Docs',
          },
          {
            to: 'api',
            position: 'left',
            label: 'API',
          },
          // { to: '/blog', label: 'Blog', position: 'left' },
          {
            href: 'https://github.com/identinet/did-web-server',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          // {
          //   title: 'Docs',
          //   items: [
          //     {
          //       label: 'Tutorial',
          //       to: '/docs/intro',
          //     },
          //   ],
          // },
          // {
          //   title: 'Community',
          //   items: [
          //     {
          //       label: 'Stack Overflow',
          //       href: 'https://stackoverflow.com/questions/tagged/docusaurus',
          //     },
          //     {
          //       label: 'Discord',
          //       href: 'https://discordapp.com/invite/docusaurus',
          //     },
          //     {
          //       label: 'Twitter',
          //       href: 'https://twitter.com/docusaurus',
          //     },
          //   ],
          // },
          // {
          //   title: 'More',
          //   items: [
          //     {
          //       label: 'Blog',
          //       to: '/blog',
          //     },
          //     {
          //       label: 'GitHub',
          //       href: 'https://github.com/facebook/docusaurus',
          //     },
          //   ],
          // },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} identinet GmbH. Built with Docusaurus.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
      },
    }),
  plugins: [
    [
      // https://github.com/praveenn77/docusaurus-lunr-search
      'docusaurus-lunr-search',
      {}
    ],
    // [
    //   '@docusaurus/plugin-client-redirects',
    //   {
    //     fromExtensions: ['html', 'htm'], // /myPage.html -> /myPage
    //     toExtensions: ['exe', 'zip'], // /myAsset -> /myAsset.zip (if latter exists)
    //     redirects: [
    //       // Start in the docs folder
    //       {
    //         to: '/',
    //         from: '/docs',
    //       },
    //     ],
    //   },
    // ],
  ],
};

export default config;
