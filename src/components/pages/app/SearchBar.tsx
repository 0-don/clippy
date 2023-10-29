import { invoke } from "@tauri-apps/api";
import { FaRegularImage } from "solid-icons/fa";
import { FiSearch } from "solid-icons/fi";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  onMount,
} from "solid-js";
import ClipboardStore, { initialWhere } from "../../../store/ClipboardStore";
import HotkeyStore from "../../../store/HotkeyStore";

interface SearchBarProps {}

export const SearchBar: Component<SearchBarProps> = ({}) => {
  let input: HTMLInputElement | undefined;
  const [search, setSearch] = createSignal("");
  const [showImages, setShowImages] = createSignal(false);
  const { setClipboards, setWhere, getClipboards } = ClipboardStore;
  const { setGlobalHotkeyEvent } = HotkeyStore;

  onMount(async () => {
    input?.focus();
    await invoke("stop_hotkeys");
    setGlobalHotkeyEvent(false);
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
      <div class="flex items-center dark:bg-zinc-800">
        <div class="relative w-full">
          <div class="absolute inset-y-0 left-2 flex items-center">
            <FiSearch class="opacity-50 dark:text-white" />
          </div>
          <input
            placeholder="Search Entries"
            class="w-full border border-gray-300 py-2 pl-8 pr-2 placeholder:text-sm focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            type="text"
            autofocus
            autocomplete="off"
            ref={input}
            value={search()}
            onInput={(e) => {
              setShowImages(false);
              setSearch(e.target.value);
            }}
            // onblur={(e) => e.target.focus()} // make search input always in focus (conflict with shortcuts, needs to be fixed)
          />
          <div class="absolute inset-y-0 right-2 flex items-center">
            <FaRegularImage
              class={` hover:text-indigo-600  ${
                showImages() ? "text-indigo-600" : ""
              }`}
              onClick={() => setShowImages(!showImages())}
            />
          </div>
        </div>
      </div>
    </>
  );
};
