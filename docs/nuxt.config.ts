// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    modules: [
        '@nuxt/eslint',
        '@nuxt/image',
        '@nuxt/ui',
        '@nuxt/content',
        'nuxt-og-image',
        'nuxt-llms',
        '@nuxtjs/mcp-toolkit'
    ],

    devtools: {
        enabled: true
    },

    css: ['~/assets/css/main.css'],

    content: {
        build: {
            markdown: {
                toc: {
                    searchDepth: 1
                },
                highlight: {
                    langs: ['lua']
                }
            },

        },
    },

    experimental: {
        asyncContext: true
    },

    compatibilityDate: '2024-07-11',

    nitro: {
        prerender: {
            routes: [
                '/'
            ],
            crawlLinks: true,
            autoSubfolderIndex: false
        }
    },

    eslint: {
        config: {
            stylistic: {
                commaDangle: 'never',
                braceStyle: '1tbs'
            }
        }
    },

    icon: {
        provider: 'iconify'
    },

    llms: {
        domain: 'https://docs-template.nuxt.dev/',
        title: 'gmsv_mongo',
        description: 'A MongoDB module for Garry\'s Mod.',
        full: {
            title: 'gmsv_mongo - Full Documentation',
            description: 'This is the full documentation for gmsv_mongo.'
        },
        sections: [
            {
                title: 'Getting Started',
                contentCollection: 'docs',
                contentFilters: [
                    {field: 'path', operator: 'LIKE', value: '/getting-started%'}
                ]
            }
        ]
    },

    mcp: {
        enabled: false,
        name: 'gmsv_mongo'
    }
})
