import { invoke } from "@tauri-apps/api";
import { BsCardImage } from "solid-icons/bs";
import { RiSystemSearchLine } from "solid-icons/ri";
import { Component, createEffect, createSignal, onCleanup } from "solid-js";
import { Clips } from "../../@types";
import ClipboardStore from "../../store/ClipboardStore";
import SwitchField from "../elements/SwitchField";
import { Clipboards } from "./Clipboards";

interface HistoryProps {}

export const History: Component<HistoryProps> = ({}) => {
  const [search, setSearch] = createSignal<undefined | string>();
  const [showImages, setShowImages] = createSignal(false);
  const { setClipboards } = ClipboardStore;

  createEffect(() => {
    const text = search();
    const img = showImages();
    const delayDebounceFn = setTimeout(async () => {
      const clipboards = await invoke<Clips[]>("infinite_scroll_clipboards", {
        search: img ? undefined : text,
        show_images: img,
      });

      console.log(clipboards);

      setClipboards(clipboards);
    }, 0);

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
            value={search()}
            onInput={(e) => {
              setShowImages(false);
              setSearch(e.target.value);
            }}
          />
          <div class="absolute inset-y-0 right-1 flex items-center">
            <RiSystemSearchLine class="dark:text-white" />
          </div>
        </div>
        <div class="flex items-center pl-2">
          <BsCardImage class="text-2xl dark:text-white" />
          <SwitchField checked={showImages} onChange={setShowImages} />
        </div>
      </div>

      <Clipboards />
    </>
  );
};
