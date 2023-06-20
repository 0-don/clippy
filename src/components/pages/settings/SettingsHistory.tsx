import { BsDeviceHdd } from "solid-icons/bs";
import { FiTrash2 } from "solid-icons/fi";
import { Component, createEffect, createSignal } from "solid-js";

interface SettingsHistoryProps {}

export const SettingsHistory: Component<SettingsHistoryProps> = ({}) => {
  const [text, setText] = createSignal<string>();

  createEffect(() => {
    // const getDatabaseInfo = async () =>
    //   setText(await window.electron.getDatabaseInfo());
    // getDatabaseInfo();
  });

  return (
    <>
      <div class="rounded-md border border-solid border-zinc-700 shadow-2xl">
        <div class="mb-2 flex items-center space-x-2 bg-zinc-800 px-5 pb-2.5 pt-5">
          <BsDeviceHdd
            onClick={async () => {
              // setText(await window.electron.getDatabaseInfo());
            }}
          />
          <h2 class="font-semibold">Local Storage</h2>
        </div>

        <ul class="mx-5 list-disc px-5 pb-5">
          <li>{text()}</li>
        </ul>
      </div>

      <div class="flex w-full justify-end pt-5">
        <button
          type="button"
          // onClick={async () => setText(await window.electron.clearDatabase())}
          class="inline-flex items-center space-x-2 rounded bg-zinc-600 px-4 py-2 text-sm font-bold text-white hover:bg-zinc-700"
        >
          <FiTrash2 />
          <span>Clear...</span>
        </button>
      </div>
    </>
  );
};
