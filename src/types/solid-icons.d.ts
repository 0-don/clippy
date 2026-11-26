// solid-icons 2.0.0 still imports JSX from "solid-js", which moved to "@solidjs/web" in Solid 2.
import type { JSX as WebJSX } from "@solidjs/web";

declare module "solid-js" {
  export import JSX = WebJSX;
}
