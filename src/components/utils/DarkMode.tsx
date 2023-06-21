import { Component, createEffect } from "solid-js";
import SettingsStore from "../../store/SettingsStore";
import SwitchField from "../elements/SwitchField";

interface DarkModeProps {}

export const DarkMode: Component<DarkModeProps> = ({}) => {
  const { settings, updateSettings, darkMode } = SettingsStore;

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
