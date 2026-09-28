import { Component, For, onCleanup, onMount } from "solid-js";
import { cn } from "../../lib/utils";
import { SettingsStore } from "../../store/settings-store";
import { useLanguage } from "../provider/language-provider";

const isEditable = (target: EventTarget | null) =>
  target instanceof HTMLElement &&
  (target.isContentEditable ||
    ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName));

export const SettingsSidebar: Component = () => {
  const { t } = useLanguage();

  // Arrow keys belong to the field being edited, so only navigate outside of one.
  const handleKeyNavigation = (e: KeyboardEvent) => {
    if (isEditable(e.target)) return;

    const tabs = SettingsStore.tabs();
    const currentIndex = tabs.findIndex((tab) => tab.current);

    if (e.key === "ArrowUp") {
      e.preventDefault();
      const newIndex = currentIndex > 0 ? currentIndex - 1 : tabs.length - 1;
      SettingsStore.setCurrentTab(tabs[newIndex].name);
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      const newIndex = currentIndex < tabs.length - 1 ? currentIndex + 1 : 0;
      SettingsStore.setCurrentTab(tabs[newIndex].name);
    }
  };

  onMount(() => window.addEventListener("keydown", handleKeyNavigation));
  onCleanup(() => window.removeEventListener("keydown", handleKeyNavigation));

  return (
    <nav class="bg-secondary flex w-40 shrink-0 flex-col gap-1 overflow-y-auto p-2">
      <For each={SettingsStore.tabs()}>
        {(tab) => (
          <button
            type="button"
            class={cn(
              "flex w-full cursor-pointer items-center gap-2.5 rounded-md px-3 py-2 text-left text-sm font-medium",
              tab.current
                ? "bg-primary text-primary-foreground"
                : "text-muted-foreground hover:bg-background/60 hover:text-foreground",
            )}
            onClick={() => SettingsStore.setCurrentTab(tab.name)}
            title={t(tab.name)}
          >
            <tab.Icon class="shrink-0 text-base" />
            <span class="truncate">{t(tab.name)}</span>
          </button>
        )}
      </For>
    </nav>
  );
};
