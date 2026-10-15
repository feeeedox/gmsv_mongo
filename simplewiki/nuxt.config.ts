import tailwindcss from "@tailwindcss/vite";
import {type BundledLanguage, bundledLanguages} from "shiki";

export default defineNuxtConfig({
  ssr: true,
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },
  css: ["@/assets/css/main.css"],
  modules: [
    "@nuxt/content",
    "@nuxt/fonts",
    "@nuxt/image",
    "@nuxt/scripts",
    "@nuxtjs/color-mode",
    "nuxt-og-image",
  ],
  colorMode: {
    preference: "light",
    fallback: "light",
    globalName: "__NUXT_COLOR_MODE__",
    classPrefix: "",
    classSuffix: "",
    storage: "localStorage",
    storageKey: "gmsv_mongo-color-mode",
  },
  image: {
    provider: "none",
  },
  nitro: {
    preset: "github-pages",
    prerender: {
      routes: ["/", "/getting-started", "/crud-operations/operators"],
      crawlLinks: true,
      autoSubfolderIndex: true,
    },
  },
  experimental: {
    asyncContext: true,
  },
  app: {
    baseURL: "/gmsv_mongo/", // Change this to your desired base URL, or remove it if you want to serve from the root.
    cdnURL: "/gmsv_mongo/", // Change this to your desired CDN URL, or remove it if you want to serve from the root.
    head: {
      base: { href: "/gmsv_mongo/" }, // Change this to your desired base URL, or remove it if you want to serve from the root.
      link: [
        { rel: "icon", href: "favicon.ico", sizes: "32x32" },

        { rel: "icon", href: "icon.svg", type: "image/svg+xml" },

        { rel: "apple-touch-icon", href: "apple-touch-icon.png" },

        { rel: "manifest", href: "site.webmanifest" },
      ],
    },
  },
  ogImage: {
    compatibility: {
      prerender: {
        browser: "on-demand",
        takumi: "node",
      },
    },
  },
  content: {
    build: {
      markdown: {
        toc: {
          searchDepth: 1,
        },
        highlight: {
          theme: {
            default: "github-light",
            dark: "github-dark",
          },
          langs: Object.keys(bundledLanguages).filter(
            (lang) =>
                lang !== "mermaid" &&
              !lang.includes("+") &&
              !lang.includes("#") &&
              !lang.includes("文"),
          ) as BundledLanguage[],
        },
      },
    },
    experimental: {
      sqliteConnector: "native",
    },
  },
  vite: {
    plugins: [tailwindcss()],
  },
  scripts: {
    registry: {
      youtubePlayer: {
        trigger: "onNuxtReady",
      },
    },
  },
});
