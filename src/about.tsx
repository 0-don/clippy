import { render } from "@solidjs/web";
import { createSignal, onSettled } from "solid-js";
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

  onSettled(() => {
    void invokeCommand(InvokeCommand.GetAppVersion).then(setVersion);
  });

  return (
    <div class="bg-background text-foreground absolute flex h-full w-full flex-col items-center justify-center space-y-2">
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
        class="border-border bg-card text-foreground hover:bg-muted inline-flex w-32 items-center justify-center rounded-sm border px-2.5 py-1.5 text-xs font-bold shadow-xs focus:ring-2 focus:ring-offset-2 focus:outline-hidden"
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
        class="border-border bg-card text-foreground! hover:bg-muted inline-flex w-32 items-center justify-center rounded-sm border px-2.5 py-1.5 text-xs font-bold shadow-xs focus:ring-2 focus:ring-offset-2 focus:outline-hidden"
      >
        {t("ABOUT.OFFICIAL_WEBSITE")}
      </a>
      <p class="text-xs">{t("ABOUT.DEVELOPED_BY")}</p>
      <p class="text-muted-foreground text-xs">{t("ABOUT.COPYRIGHT")}</p>
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
