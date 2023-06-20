import { Component, createEffect } from "solid-js";
import SettingsStore from "../../store/SettingsStore";
import SwitchField from "../elements/SwitchField";

interface DarkModeProps {}

export const DarkMode: Component<DarkModeProps> = ({}) => {
  const { settings, updateSettings } = SettingsStore;

  createEffect(() => {
    if (settings()?.dark_mode) {
      document.querySelector("html")?.classList?.add?.("dark");
    } else {
      document.querySelector("html")?.classList?.remove?.("dark");
    }
  });

  return (
    <SwitchField
      checked={settings()?.dark_mode || false}
      onChange={() =>
        updateSettings({
          ...settings()!,
          dark_mode: !settings()?.dark_mode,
        })
      }
    />
  );
};
