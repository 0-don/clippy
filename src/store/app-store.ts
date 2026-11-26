import { BsStarFill } from "solid-icons/bs";
import { CgMore } from "solid-icons/cg";
import { TbOutlineSearch } from "solid-icons/tb";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { invokeCommand } from "../lib/tauri";
import { Settings, Tabs } from "../types";
import { HotkeyEvent, PasswordAction } from "../types/enums";
import { InvokeCommand } from "../types/tauri-invoke";
import {
  LANGUAGES,
  LANGUAGE_KEY,
  Language,
  TAB_NAMES,
  Tab,
} from "../utils/constants";
import { SettingsStore } from "./settings-store";

function createAppStore() {
  const [passwordLock, setPasswordLock] = createSignal<PasswordAction>();
  const storedLocale = localStorage.getItem(LANGUAGE_KEY);
  const detectedLocale: Language =
    LANGUAGES.find((language) => language === storedLocale) ?? LANGUAGES[0];

  const [locale, setLocale] = createSignal<Language>(detectedLocale);
  const refreshLocale = async () =>
    setLocale(
      (await invokeCommand(InvokeCommand.GetSettings))?.language ||
        detectedLocale,
    );

  const [tabs, setTabs] = createSignal<Tabs[]>([
    {
      name: TAB_NAMES[0],
      Icon: VsHistory,
      current: true,
      id: HotkeyEvent.RecentClipboards,
    },
    {
      name: TAB_NAMES[1],
      Icon: BsStarFill,
      current: false,
      id: HotkeyEvent.StarredClipboards,
    },
    {
      name: TAB_NAMES[2],
      Icon: TbOutlineSearch,
      current: false,
      id: HotkeyEvent.History,
    },
    {
      name: TAB_NAMES[3],
      Icon: CgMore,
      current: false,
      id: HotkeyEvent.ViewMore,
    },
  ]);

  const changeTab = (id: Tab) =>
    setTabs((prev) => prev.map((s) => ({ ...s, current: s.id === id })));
  const getCurrentTab = () => tabs().find((s) => s.current)!;

  // OS is needed to gate the glass feature (Windows/macOS only). Resolved once.
  const [os, setOs] = createSignal<string>();
  invokeCommand(InvokeCommand.GetOs)
    .then(setOs)
    .catch(() => setOs(""));
  void refreshLocale();

  // Apply the full appearance to <html> from the current settings:
  // - `.dark` class -> light vs dark within the chosen theme
  // - `data-theme`  -> which named palette
  // - `data-glass`  -> translucent surfaces for the native window blur
  //   (gated off on Linux, where native glass is unsupported)
  // Takes the settings explicitly: a setter is not visible to reads until the next flush.
  const applyAppearance = (
    settings: Settings | undefined = SettingsStore.settings(),
    platform = os(),
  ) => {
    const html = document.querySelector("html");
    if (!html) return;

    settings?.dark_mode
      ? html.classList.add("dark")
      : html.classList.remove("dark");

    html.setAttribute("data-theme", settings?.theme || "neutral");

    platform !== "linux" && settings?.glass
      ? html.setAttribute("data-glass", "on")
      : html.removeAttribute("data-glass");

    // Glass tint: push the slider value (0..1) onto <html> as a CSS var. styles.css
    // derives the surface alphas from it via calc(), so dragging the slider restyles
    // the glass live. Fall back to the DB default if the setting is missing.
    html.style.setProperty(
      "--glass-opacity",
      `${settings?.glass_opacity ?? 0.55}`,
    );
  };

  return {
    locale,
    refreshLocale,
    os,
    passwordLock,
    setPasswordLock,
    tabs,
    setTabs,
    changeTab,
    getCurrentTab,
    applyAppearance,
  };
}

export const AppStore = createRoot(createAppStore);
export type AppStore = ReturnType<typeof createAppStore>;
