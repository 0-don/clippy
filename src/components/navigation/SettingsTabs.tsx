import { Component, For } from "solid-js";
import SettingsStore from "../../store/SettingsStore";

interface TabsProps {}

export const Tabs: Component<TabsProps> = ({}) => {
  const { tabs, setCurrentTab } = SettingsStore;

  return (
    <div class="border-b border-gray-500">
      <nav class="-mb-px flex justify-center space-x-8">
        <For each={tabs()}>
          {({ Icon, current, name }) => (
            <button
              type="button"
              class={`${
                current
                  ? "dark:border-white dark:text-white border-zinc-600 text-zinc-600"
                  : "border-transparent hover:border-zinc-600 hover:text-zinc-600 dark:text-gray-500 dark:hover:border-white dark:hover:text-white"
              } group inline-flex items-center border-b-2 px-3 py-4 text-sm font-medium`}
              onClick={() => setCurrentTab(name)}
            >
              <Icon class="text-1xl mr-2 dark:text-white" />
              <span>{name}</span>
            </button>
          )}
        </For>
      </nav>
    </div>
  );
};
