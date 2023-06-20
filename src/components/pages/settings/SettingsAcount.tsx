import { Component, createEffect, createSignal } from "solid-js";
import SettingsStore from "../../../store/SettingsStore";
import SwitchField from "../../elements/SwitchField";

interface SettingsAccountProps {}

export const SettingsAccount: Component<SettingsAccountProps> = ({}) => {
  const [url, setUrl] = createSignal<string>();
  const { settings, updateSettings } = SettingsStore;

  createEffect(() => {
    // const getUrl = async () => {
    //   const res = await window.electron.getDatbasePath();
    //   if (res) setUrl(res);
    // };
    // getUrl();
  });

  return (
    <>
      <TextBlock icon="sync" title="Sync">
        <div class="mb-2 flex items-center justify-between space-x-2 px-5 pb-2.5">
          <div class="flex items-center space-x-2 truncate">
            <FontAwesomeIcon icon={["far", "save"]} />
            <h6 class="text-sm">Synchronize clipboard history</h6>
          </div>
          <div>
            <SwitchField checked={settings?.synchronize} onChange={undefined} />
          </div>
        </div>
        {settings?.synchronize && (
          <div class="mb-2 flex items-center justify-between space-x-2 px-5 pb-2.5">
            <div class="flex items-center space-x-2 truncate">
              <FontAwesomeIcon icon={["far", "clock"]} />
              <h6 class="text-sm">Change backup time</h6>
            </div>
            <div class="flex items-center">
              <p class="text-sm">Minutes:&nbsp;</p>
              <Dropdown
                items={[1, 5, 10, 15, 30, 60]}
                value={settings.syncTime / 60}
                onChange={async (syncTime) => {
                  await updateSettings({
                    ...settings,
                    syncTime: (syncTime as number) * 60,
                  });
                }}
              />
            </div>
          </div>
        )}
      </TextBlock>

      {url && settings?.synchronize && (
        <TextBlock
          icon="globe-europe"
          title="Database Location"
          class="animate-fade"
        >
          <div class="list-disc px-5 pb-5 pt-2.5">
            <button
              type="button"
              class="group relative w-full cursor-pointer"
              onClick={async () => {
                const res = await window.electron.selectDatabasePath();
                if (res) setUrl(res);
              }}
            >
              <div
                title={url}
                class="w-full truncate rounded-md border border-gray-300 px-3 py-0.5 text-left text-sm italic focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
              >
                {url}
              </div>
              <div class="group absolute inset-y-0 right-1 my-1 flex items-center space-x-1 rounded bg-gray-600 px-2 text-xs text-white group-hover:bg-gray-400">
                <FontAwesomeIcon icon="upload" />
                <div>Browse</div>
              </div>
            </button>
          </div>
        </TextBlock>
      )}
    </>
  );
};
