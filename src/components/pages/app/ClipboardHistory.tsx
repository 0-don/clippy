import { invoke } from "@tauri-apps/api";
import { BsCardImage } from "solid-icons/bs";
import { RiSystemSearchLine } from "solid-icons/ri";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  onMount,
} from "solid-js";
import ClipboardStore, { initialWhere } from "../../../store/ClipboardStore";
import SwitchField from "../../elements/SwitchField";
import { Clipboards } from "./Clipboards";

interface ClipboardHistoryProps {}

export const ClipboardHistory: Component<ClipboardHistoryProps> = ({}) => {
  let input: HTMLInputElement | undefined;
  const [search, setSearch] = createSignal("");
  const [showImages, setShowImages] = createSignal(false);
  const { setClipboards, setWhere, getClipboards } = ClipboardStore;

  onMount(() => {
    input?.focus();
    invoke("stop_hotkeys");
  });

  createEffect(() => {
    const text = search();
    const img = showImages();

    const delayDebounceFn = setTimeout(async () => {
      setWhere(() => ({
        ...initialWhere,
        search: text.length && !img ? text : undefined,
        img: img || undefined,
      }));
      const clipboards = await getClipboards();
      setClipboards(clipboards);
    }, 0);

    onCleanup(() => clearTimeout(delayDebounceFn));
  });

  return (
    <>
      <div class="flex items-center px-3 py-2 dark:bg-zinc-800">
        <div class="relative w-full">
          <input
            placeholder="search"
            class="w-full rounded-md border border-gray-300 px-3 py-0.5 focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            type="text"
            autofocus
            ref={input}
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
