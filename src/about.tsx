import { createSignal, onMount } from "solid-js";
import { render } from "solid-js/web";
import icon from "./assets/clippy.png";
import LanguageProvider, {
  useLanguage,
} from "./components/provider/language-provider";
import { invokeCommand } from "./lib/tauri";
import "./styles.css";
import { InvokeCommand } from "./types/tauri-invoke";

const About = () => {
  const { t } = useLanguage();
  const [version, setVersion] = createSignal("0.0.0");

  onMount(async () =>
    setVersion(await invokeCommand(InvokeCommand.GetAppVersion)),
  );

  return (
    <div class="absolute flex h-full w-full flex-col items-center justify-center space-y-2 bg-background text-foreground">
      <img src={icon} alt="logo" width="300px" />
      <h1 class="text-xl font-bold">{version()}</h1>
      <h2 class="text-base">{t("ABOUT.NO_UPDATES_CURRENTLY_AVAILABLE")}</h2>
      <a
        href="#"
        onClick={() =>
          invokeCommand(InvokeCommand.OpenBrowserUrl, {
            url: "https://github.com/0-don/clippy",
          })
        }
        class="inline-flex w-32 items-center justify-center rounded-sm border border-border bg-card px-2.5 py-1.5 text-xs font-bold text-foreground shadow-xs hover:bg-muted focus:ring-2 focus:ring-offset-2 focus:outline-hidden"
      >
        Github
      </a>
      <a
        href="#"
        onClick={() =>
          invokeCommand(InvokeCommand.OpenBrowserUrl, {
            url: "https://github.com/0-don/clippy",
          })
        }
        class="inline-flex w-32 items-center rounded-sm border border-border bg-card px-2.5 py-1.5 text-xs font-bold text-foreground! shadow-xs hover:bg-muted focus:ring-2 focus:ring-offset-2 focus:outline-hidden"
      >
        {t("ABOUT.OFFICIAL_WEBSITE")}
      </a>
      <p class="text-xs">{t("ABOUT.DEVELOPED_BY")}</p>
      <p class="text-xs text-muted-foreground">{t("ABOUT.COPYRIGHT")}</p>
    </div>
  );
};

render(
  () => (
    <LanguageProvider>
      <About />
    </LanguageProvider>
  ),
  document.getElementById("root") as HTMLElement,
);
