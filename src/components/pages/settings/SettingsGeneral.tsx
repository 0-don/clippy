import { Component } from "solid-js";
import SwitchField from "../../elements/SwitchField";

interface SettingsGeneralProps {}

export const SettingsGeneral: Component<SettingsGeneralProps> = ({}) => {
  const { hotkeys, settings, updateSettings } = useSettingsStore();
  const hotkey = hotkeys.find(
    (key) => key.event === "windowDisplayToggle"
  ) as ExtendedHotKey;

  return (
    <>
      <TextBlock icon={["far", "keyboard"]} title="Keyboard shortcut">
        <div className="mb-2 flex items-center space-x-2 px-5 pb-2.5">
          <Shortcut hotkey={hotkey} />
        </div>
      </TextBlock>

      <TextBlock icon="cog" title="System">
        <div className="flex items-center justify-between space-x-2 px-5 pb-5">
          <div className="flex items-center space-x-2 truncate">
            <FontAwesomeIcon icon="rocket" />
            <h6 className="text-sm">Start Clippy on system startup.</h6>
          </div>
          <div>
            <SwitchField
              checked={settings.startup}
              onChange={(check: boolean) =>
                updateSettings({ ...settings, startup: check })
              }
            />
          </div>
        </div>

        <div className="flex items-center justify-between space-x-2 px-5 pb-5">
          <div className="flex items-center space-x-2 truncate">
            <FontAwesomeIcon icon="bell" />
            <h6 className="text-sm">Show desktop notifications.</h6>
          </div>
          <div>
            <SwitchField
              checked={settings.notification}
              onChange={(check: boolean) =>
                updateSettings({ ...settings, notification: check })
              }
            />
          </div>
        </div>

        <div className="flex items-center justify-between space-x-2 px-5 pb-5">
          <div className="flex items-center space-x-2 truncate">
            <FontAwesomeIcon icon={["far", "moon"]} />
            <h6 className="text-sm">Switch Theme.</h6>
          </div>
          <div>
            <DarkMode />
          </div>
        </div>
      </TextBlock>
    </>
  );
};
