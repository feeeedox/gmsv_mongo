export default defineAppConfig({
    ui: {
        colors: {
            primary: 'emerald',
            neutral: 'gray'
        },
        footer: {
            slots: {
                root: 'border-t border-default',
                left: 'text-sm text-muted'
            }
        }
    },
    seo: {
        siteName: 'gmsv_mongo'
    },
    header: {
        title: 'gmsv_mongo',
        to: '/',
        logo: {
            alt: '',
            light: '',
            dark: ''
        },
        search: true,
        colorMode: true,
        links: [{
            'icon': 'i-simple-icons-github',
            'to': 'https://github.com/feeeedox/gmsv_mongo',
            'target': '_blank',
            'aria-label': 'GitHub'
        }]
    },
    footer: {
        credits: `gmsv_mongo • © ${new Date().getFullYear()}`,
        colorMode: true,
        links: [
            {
                'icon': 'i-simple-icons-github',
                'to': 'https://github.com/feeeedox/gmsv_mongo',
                'target': '_blank',
                'aria-label': 'gmsv_mongo on GitHub'
            },
            {
                "target": "_blank",
                "aria-label": "Imprint",
                "to": "https://fedox.ovh/imprint",
                "label": "Imprint"
            }
        ]
    },
    toc: {
        title: 'Table of Contents',
        bottom: {
            title: 'Community',
            edit: 'https://github.com/feeeedox/gmsv_mongo/edit/master/docs/content',
            links: [{
                icon: 'i-lucide-star',
                label: 'Star on GitHub',
                to: 'https://github.com/feeeedox/gmsv_mongo',
                target: '_blank'
            }, {
                icon: 'i-lucide-book-open',
                label: 'View Docs Source',
                to: 'https://github.com/feeeedox/gmsv_mongo/tree/master/docs',
                target: '_blank'
            }, {
                label: "Garry's Mod Wiki",
                icon: 'i-lucide-globe',
                to: 'https://gmodwiki.com',
                target: '_blank'
            }]
        }
    }
})
