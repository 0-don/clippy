// @ts-check
import node from "@astrojs/node";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwind from "@astrojs/tailwind";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  output: "server",
  site: "https://clippy.coding.global",
  integrations: [
    starlight({
      title: "Clippy Docs",
      social: {
        github: "https://github.com/0-don/clippy",
        discord: "https://discord.gg/coding",
      },
      sidebar: [
        {
          label: "Installation",
          items: [
            // Each item here is one entry in the navigation menu.
            { label: "Example Guide", slug: "guides/example" },
          ],
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
      ],
    }),
    tailwind(),
    sitemap(),
  ],

  adapter: node({ mode: "standalone" }),
});
