import { Component, createEffect } from "solid-js";
import AppStore from "../../store/AppStore";
import SettingsStore from "../../store/SettingsStore";
import { Toggle } from "../elements/Toggle";

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
