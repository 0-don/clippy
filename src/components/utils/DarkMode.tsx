import { Component, createEffect } from "solid-js";
import SettingsStore from "../../store/SettingsStore";
import SwitchField from "../elements/SwitchField";

interface DarkModeProps {}

export const DarkMode: Component<DarkModeProps> = ({}) => {
  const { settings, updateSettings } = SettingsStore;

  createEffect(() => {
    console.log(settings());
    if (settings()?.dark_mode) {
      console;
      document.querySelector("body")?.classList?.add?.("dark");
    } else {
      document.querySelector("body")?.classList?.remove?.("dark");
    }
  });

  return (
    <SwitchField
      checked={settings()?.dark_mode}
      onChange={(dark_mode) =>
        updateSettings({
          ...settings()!,
          dark_mode,
        })
      }
    />
  );
};
