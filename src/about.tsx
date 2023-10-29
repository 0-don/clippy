import { getVersion } from "@tauri-apps/api/app";
import { createSignal, onMount } from "solid-js";
import { render } from "solid-js/web";
import icon from "./assets/clippy.png";
import "./styles.css";

const About = () => {
  const [version, setVersion] = createSignal("0.0.0");

  onMount(async () => {
    const appVersion = await getVersion();
    setVersion(appVersion);
  });

  return (
    <div class="absolute flex h-full w-full flex-col items-center justify-center space-y-2 bg-white text-black dark:bg-dark  dark:text-white">
      <img src={icon} alt="logo" width="300px" />
      <h1 class="text-xl font-bold">{version()}</h1>
      <h2 class="text-base">No updates currently available</h2>
      <a
        href="https://github.com/Don-Cryptus/clippy"
        target="_blank"
        rel="noreferrer"
        class="inline-flex w-28 items-center justify-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 "
      >
        Github
      </a>
      <a
        href="https://github.com/Don-Cryptus/clippy"
        target="_blank"
        rel="noreferrer"
        class="inline-flex w-28 items-center rounded border border-gray-300 bg-white px-2.5 py-1.5 text-xs font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 "
      >
        Official Website
      </a>
      <p class="text-xs">Developed by Don Cryptus. Powered by Tauri.</p>
      <p class="text-xs text-gray-500">Copyright(C) DC. All right reserved.</p>
    </div>
  );
};

render(() => <About />, document.getElementById("root") as HTMLElement);
