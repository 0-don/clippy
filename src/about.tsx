import { createSignal, onMount } from "solid-js";
import { render } from "solid-js/web";
import icon from "./assets/clippy.png";
import "./styles.css";
import { InvokeCommand } from "./types/tauri-invoke";
import { invokeCommand } from "./utils/tauri";

const About = () => {
  const [version, setVersion] = createSignal("0.0.0");

  onMount(async () => setVersion(await invokeCommand(InvokeCommand.GetAppVersion)));

  return (
    <div class="absolute flex h-full w-full flex-col items-center justify-center space-y-2 bg-white text-black dark:bg-dark dark:text-white">
      <img src={icon} alt="logo" width="300px" />
      <h1 class="text-xl font-bold">{version()}</h1>
      <h2 class="text-base">No updates currently available</h2>
      <a
        href="#"
        onClick={() => invokeCommand(InvokeCommand.OpenBrowserUrl, { url: "https://github.com/0-don/clippy" })}
        class="inline-flex w-32 items-center justify-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-bold text-zinc-950 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2"
      >
        Github
      </a>
      <a
        href="#"
        onClick={() => invokeCommand(InvokeCommand.OpenBrowserUrl, { url: "https://github.com/0-don/clippy" })}
        class="inline-flex w-32 items-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-bold !text-zinc-950 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2"
      >
        Official Website
      </a>
      <p class="text-xs">Developed by 0-don. Powered by Tauri.</p>
      <p class="text-xs text-gray-500">Copyright(C) DC. All right reserved.</p>
    </div>
  );
};

render(() => <About />, document.getElementById("root") as HTMLElement);
