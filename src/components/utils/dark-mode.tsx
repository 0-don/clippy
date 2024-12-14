import { Component, createEffect } from "solid-js";
import AppStore from "../../store/app-store";
import SettingsStore from "../../store/settings-store";
import { Toggle } from "../elements/toggle";

interface DarkModeProps {}

export const DarkMode: Component<DarkModeProps> = ({}) => {
  const { settings, updateSettings } = SettingsStore;
  const { darkMode } = AppStore;

  createEffect(darkMode);

  return (
    <Toggle
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
