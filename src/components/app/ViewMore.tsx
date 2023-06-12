import { Component } from "solid-js";

interface ViewMoreProps {}

export const ViewMore: Component<ViewMoreProps> = ({}) => {
  const { settings, updateSettings, hotkeys, globalHotkeyEvent } =
    useSettingsStore();

  useEffect(() => {
    const syncClipboardHistory = window.electron.on(
      "syncClipboardHistory",
      async () => {
        await window.electron.toggleSyncClipboardHistory();
      }
    );

    const preferences = window.electron.on("preferences", () =>
      window.electron.createSettingsWindow()
    );

    const about = window.electron.on("about", () =>
      window.electron.createAboutWindow()
    );

    const exit = window.electron.on("exit", () => window.electron.exit());

    return () => {
      syncClipboardHistory();
      preferences();
      about();
      exit();
    };
  }, [updateSettings, settings]);

  const createButton = (name: ViewMoreName, onClick: () => void) => {
    const hotkey = hotkeys.find((key) => key.name === name) as ExtendedHotKey;

    return (
      <button
        type="button"
        className="w-full cursor-pointer px-3 hover:bg-neutral-700"
        onClick={onClick}
      >
        <div className="flex items-center justify-between py-4">
          <div className="flex items-center ">
            <div className="relative">
              <FontAwesomeIcon
                icon={JSON.parse(hotkey.icon)}
                className="text-2xl"
              />
              {globalHotkeyEvent && hotkey.status && (
                <div className="absolute left-0 top-0 -ml-2 -mt-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
                  {hotkey.key}
                </div>
              )}
            </div>
            <p className="px-4 text-base font-semibold">{name}</p>
          </div>
          {name === "Sync Clipboard History" && (
            <SwitchField checked={settings.synchronize} onChange={undefined} />
          )}
        </div>
        <hr className="border-zinc-700" />
      </button>
    );
  };

  return (
    <>
      {/* Sync Clipboard History  */}
      {createButton("Sync Clipboard History", async () => {
        await window.electron.toggleSyncClipboardHistory();
      })}

      {/* Preferences */}
      {createButton("Preferences", () =>
        window.electron.createSettingsWindow()
      )}

      {/* About */}
      {createButton("About", () => window.electron.createAboutWindow())}

      {/* Exit */}
      {createButton("Exit", () => window.electron.exit())}
    </>
  );
};
