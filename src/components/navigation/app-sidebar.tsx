import { Dynamic } from "@solidjs/web";
import { Component, For, Show } from "solid-js";
import { AppStore } from "../../store/app-store";
import { HotkeyStore } from "../../store/hotkey-store";
import { useLanguage } from "../provider/language-provider";

interface AppSidebarProps {}

export const AppSidebar: Component<AppSidebarProps> = ({}) => {
  const { t } = useLanguage();

  return (
    <Show when={HotkeyStore.hotkeys().length}>
      <For each={AppStore.tabs()} keyed={(tab) => tab.id}>
        {(tab) => {
          const hotkey = () =>
            HotkeyStore.hotkeys().find((key) => key.name === tab().name);

          return (
            <div
              class={[
                "hover:text-foreground relative flex h-6 w-full cursor-pointer items-center justify-center py-5 text-xl select-none",
                tab().current ? "text-foreground" : "text-muted-foreground",
              ]}
              title={t(tab().name)}
              onClick={() => AppStore.changeTab(tab().id)}
            >
              <Dynamic component={tab().Icon} title={t(tab().name)} />
              <Show when={HotkeyStore.globalHotkeyEvent() && hotkey()}>
                {(key) => (
                  <div class="bg-secondary text-secondary-foreground absolute -top-0.5 left-1 rounded-xs px-1 py-1 text-xs font-semibold">
                    {key().key}
                  </div>
                )}
              </Show>
            </div>
          );
        }}
      </For>
      {/* {import.meta.env.DEV && (
        <button onClick={() => invokeCommand(InvokeCommand.AuthGoogleDrive)} class="">
          AUTH
        </button>
      )} */}
    </Show>
  );
};
