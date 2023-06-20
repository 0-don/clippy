import { Component } from "solid-js";

interface SettingsHotkeysProps {}

export const SettingsHotkeys: Component<SettingsHotkeysProps> = ({}) => {
  const { hotkeys } = useSettingsStore();

  return (
    <>
      <TextBlock icon="key" title="Change your Hotkeys">
        <div className="h-64 overflow-auto px-5">
          {hotkeys.map((hotkey, index) => (
            <div key={hotkey.id} className="">
              <div className="flex items-center px-0.5 py-4">
                <Shortcut hotkey={hotkey} />
              </div>
              {hotkeys.length !== index + 1 && (
                <hr className="border-zinc-700" />
              )}
            </div>
          ))}
        </div>
      </TextBlock>
    </>
  );
};
