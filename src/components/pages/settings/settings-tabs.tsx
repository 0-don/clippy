import { Component, For } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { useLanguage } from "../../provider/language-provider";

interface TabsProps {}

export const Tabs: Component<TabsProps> = ({}) => {
  const { t } = useLanguage();

  return (
    <div class="border-b border-gray-500">
      <nav class="-mb-px flex justify-center">
        <For each={SettingsStore.tabs()}>
          {({ Icon, current, name }) => (
            <button
              type="button"
              class={`${
                current
                  ? "border-zinc-600 text-zinc-600 dark:border-white dark:text-white"
                  : "border-transparent hover:border-zinc-600 hover:text-zinc-600 dark:text-gray-500 dark:hover:border-white dark:hover:text-white"
              } group inline-flex items-center border-b-2 px-2 py-4 text-sm font-medium`}
              onClick={() => SettingsStore.setCurrentTab(name)}
              title={t(name)}
            >
              <Icon class="text-1xl mr-2 dark:text-white" />
              <span class="max-w-20 truncate">{t(name)}</span>
            </button>
          )}
        </For>
      </nav>
    </div>
  );
};
