import { Component, createEffect } from "solid-js";
import { AppStore } from "../../store/app-store";
import { SettingsStore } from "../../store/settings-store";
import { Toggle } from "../elements/toggle";

interface DarkModeProps {}

export const DarkMode: Component<DarkModeProps> = ({}) => {
  createEffect(AppStore.darkMode);

  return (
    <Toggle
      checked={SettingsStore.settings()?.dark_mode}
      onChange={(dark_mode) =>
        SettingsStore.updateSettings({
          ...SettingsStore.settings()!,
          dark_mode,
        })
      }
    />
  );
};
