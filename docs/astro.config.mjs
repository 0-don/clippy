// @ts-check
import node from "@astrojs/node";
import sitemap from "@astrojs/sitemap";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  output: "server",
  site: "https://clippy.coding.global",
  integrations: [
    starlight({
      title: "Clippy Docs",
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/0-don/clippy" },
        { icon: "discord", label: "Discord", href: "https://discord.gg/coding" },
      ],
      sidebar: [
        {
          label: "Quick Start",
          items: [
            // Each item here is one entry in the navigation menu.
            { label: "Installation Guide", slug: "guides/installation" },
            {
              label: "Features",
              items: [
                { label: "Clipboard History", slug: "features/clipboard-history" },
                { label: "Global Hotkeys", slug: "features/hotkeys" },
                { label: "Cloud Sync", slug: "features/cloud-sync" },
                { label: "File Support", slug: "features/file-support" },
              ],
            },
          ],
        },
        { label: "Reference", autogenerate: { directory: "reference" } },
        {
          label: "Legal",
          items: [
            { label: "Privacy Policy", slug: "legal/privacy-policy" },
            { label: "Terms of Service", slug: "legal/terms-of-service" },
          ],
        },
      ],
    }),
    sitemap(),
  ],

  adapter: node({ mode: "standalone" }),

  vite: { plugins: [tailwindcss()] },
});
