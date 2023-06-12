import { invoke } from "@tauri-apps/api";
import { AiOutlineSearch } from "solid-icons/ai";
import { FaRegularImages } from "solid-icons/fa";
import { Component, createEffect, createSignal, onCleanup } from "solid-js";
import { Clips } from "../../@types";
import AppStore from "../../store/AppStore";
import SettingsStore from "../../store/SettingsStore";
import SwitchField from "../elements/SwitchField";
import { Clipboards } from "./Clipboards";

interface HistoryProps {}

export const History: Component<HistoryProps> = ({}) => {
  const [input, setInput] = createSignal<HTMLInputElement>();
  const [search, setSearch] = createSignal("");
  const [showImages, setShowImages] = createSignal(false);
  const { setGlobalHotkeyEvent } = SettingsStore;
  const { setClipboards } = AppStore;

  createEffect(() => {
    const delayDebounceFn = setTimeout(async () => {
      const clipboards = await invoke<Clips[]>("infinite_scroll_clipboards", {
        search: showImages() ? undefined : search(),
        showImages: showImages(),
      });
      setClipboards(clipboards);
    });

    onCleanup(() => clearTimeout(delayDebounceFn));
  });

  return (
    <>
      <div class="flex items-center bg-zinc-800 px-3 py-2">
        <div class="relative w-full">
          <input
            placeholder="search"
            class="w-full rounded-md border border-gray-300 px-3 py-0.5 focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            type="text"
            onFocus={async () => {
              setGlobalHotkeyEvent(false);
              await invoke("disableHotkeys");
            }}
            value={search()}
            ref={(e) => setInput(e)}
            onChange={(e) => {
              setShowImages(false);
              setSearch(e.target.value);
            }}
          />
          <div class="absolute inset-y-0 right-1 flex items-center">
            <AiOutlineSearch />
          </div>
        </div>
        <div class="flex items-center pl-2">
          <FaRegularImages class="text-2xl" />
          <SwitchField checked={showImages} onChange={setShowImages} />
        </div>
      </div>

      <Clipboards search={search} />
    </>
  );
};
