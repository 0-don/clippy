import { invoke } from "@tauri-apps/api";
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
  const { setClipboards, setWhere, getClipboards } = ClipboardStore;
  const { setGlobalHotkeyEvent } = HotkeyStore;

  onMount(async () => {
    input?.focus();
    setSearch("");
    await invoke("stop_hotkeys");
    setGlobalHotkeyEvent(false);
  });

  createEffect(() => {
    const text = search();

    const delayDebounceFn = setTimeout(async () => {
      setWhere(() => ({
        ...initialWhere,
        search: text.length > 0 ? text : undefined
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
            <FiSearch class="dark:text-white opacity-50" />
          </div>
          <input
            placeholder="Search Entries"
            class="pl-8 pr-2 w-full border border-gray-300 py-4 focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark"
            type="text"
            autofocus
            autocomplete="off"
            ref={input}
            value={search()}
            onInput={(e) => {
              setSearch(e.target.value);
            }}
            // onblur={(e) => e.target.focus()} // make search input always in focus (conflict with shortcuts, needs to be fixed)
          />
        </div>
      </div>
    </>
  );
};
