// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  srcDir: 'src',
  ssr: false,
  experimental: {
    payloadExtraction: false
  },
  app: {
    pageTransition: { name: "page", mode: "out-in" },
    head: {
      meta: [
        { name: "viewport", content: "width=device-width, initial-scale=1" },
      ],
      htmlAttrs: {
        "data-bs-theme": "dark"
      },
      link: [
        {
          rel: "stylesheet",
          href: "/css/bootstrap.min.css",
        },
        {
          rel: "stylesheet",
          href: "/css/application.css",
        },
        {
          rel: "icon",
          type: "image/png",
          href: "/favicon.png",
        },
      ],
      script: [
        {
          src: "/js/bootstrap.bundle.min.js",
        },
      ],
    },
  },
})
