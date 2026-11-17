// GitHub Pages project site (docs/adr/0012-astro-website-github-pages.md).
import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://juanestban.github.io",
  base: "/panda-launcher",
  trailingSlash: "always",
});
