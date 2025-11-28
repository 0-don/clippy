import { FaRegularImage } from "solid-icons/fa";
import { FiSearch } from "solid-icons/fi";
import { ImSpinner2 } from "solid-icons/im";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  onMount,
  Show,
} from "solid-js";
import { invokeCommand } from "../../../lib/tauri";
import { AppStore } from "../../../store/app-store";
import { ClipboardStore, initialWhere } from "../../../store/clipboard-store";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { useLanguage } from "../../provider/language-provider";

interface SearchBarProps {}

export const SearchBar: Component<SearchBarProps> = ({}) => {
  let input: HTMLInputElement | undefined;
  const { t } = useLanguage();
  const [search, setSearch] = createSignal("");
  const [showImages, setShowImages] = createSignal(false);

  onMount(async () => {
    input?.focus();
    await invokeCommand(InvokeCommand.StopHotkeys);
    HotkeyStore.enableGlobalHotkeyEvent(false);
  });

  createEffect(() => {
    const text = search();
    const img = showImages();

    // Use conditional debounce: 300ms for encrypted search, 0ms for non-encrypted
    const debounceDelay = SettingsStore.settings()?.encryption ? 300 : 0;

    const delayDebounceFn = setTimeout(async () => {
      ClipboardStore.setWhere(() => ({
        ...initialWhere,
        search: text.length && !img ? text : undefined,
        img: img || undefined,
        star:
          AppStore.getCurrentTab()?.name === "MAIN.HOTKEY.STARRED_CLIPBOARDS"
            ? true
            : undefined,
      }));
      const clipboards = await ClipboardStore.getClipboards();
      ClipboardStore.setClipboards(clipboards);
    }, debounceDelay);

    onCleanup(() => clearTimeout(delayDebounceFn));
  });

  return (
    <>
      <div class="flex items-center dark:bg-zinc-800">
        <div class="relative w-full">
          <div class="absolute inset-y-0 left-2 flex items-center">
            <Show
              when={!ClipboardStore.isSearching()}
              fallback={<ImSpinner2 class="animate-spin opacity-50 dark:text-white" />}
            >
              <FiSearch class="opacity-50 dark:text-white" />
            </Show>
          </div>
          <input
            placeholder={t("CLIPBOARD.SEARCH_ENTRIES")}
            class="dark:border-dark-light dark:bg-dark-light dark:focus:bg-dark-dark w-full border border-gray-300 py-2 pr-2 pl-8 placeholder:text-sm focus:outline-hidden dark:text-white"
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
              class={`hover:text-indigo-600 ${showImages() ? "text-indigo-600" : ""}`}
              onClick={() => setShowImages(!showImages())}
            />
          </div>
        </div>
      </div>
    </>
  );
};
