import { Tabs } from "@kobalte/core";
import { render } from "solid-js/web";

import "./styles.css";

const Settings = () => {
  const tabs = useSettingsStore((state) => state.tabs);
  const currentTab = tabs.find((tab) => tab.current)?.name;

  return (
    <div className="absolute flex h-full w-full flex-col overflow-hidden bg-white text-black dark:bg-dark dark:text-white">
      <Tabs />
      <div className="p-5 dark:text-white">
        {currentTab === "General" && <General />}
        {currentTab === "Account" && <Account />}
        {currentTab === "History" && <History />}
        {currentTab === "Hotkeys" && <Hotkeys />}
      </div>
    </div>
  );
};

render(() => <Settings />, document.getElementById("root") as HTMLElement);
