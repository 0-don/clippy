// @ts-check
import node from "@astrojs/node";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  output: "server",
  integrations: [
    starlight({
      title: "Clippy Docs",
      social: {
        github: "https://github.com/0-don/clippy",
        discord: "https://discord.gg/coding",
      },
      sidebar: [
        {
          label: "Guides",
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
  ],

  adapter: node({ mode: "standalone" }),
});
