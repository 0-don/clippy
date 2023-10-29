import AppStore from "@/store/AppStore";
import SettingsStore from "@/store/SettingsStore";
import { Component, createEffect } from "solid-js";
import SwitchField from "../elements/SwitchField";

interface DarkModeProps {}

export const DarkMode: Component<DarkModeProps> = ({}) => {
  const { settings, updateSettings } = SettingsStore;
  const { darkMode } = AppStore;

  createEffect(darkMode);

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
