import { Component, For } from "solid-js";
import SettingsStore from "../../store/SettingsStore";

interface TabsProps {}

export const Tabs: Component<TabsProps> = ({}) => {
  const { tabs, setCurrentTab } = SettingsStore;

  return (
    <div class="border-b border-gray-500">
      <nav class="-mb-px flex space-x-8" aria-label="Tabs">
        <For each={tabs()}>
          {({ Icon, current, name }) => (
            <button
              type="button"
              class={`${
                current
                  ? "border-white text-white"
                  : "border-transparent text-gray-500 hover:border-white hover:text-white"
              } group inline-flex items-center border-b-2 px-3 py-4 text-sm font-medium`}
              onClick={() => setCurrentTab(name)}
            >
              <Icon class="text-1xl mr-2 text-white" />
              <span>{name}</span>
            </button>
          )}
        </For>
      </nav>
    </div>
  );
};
