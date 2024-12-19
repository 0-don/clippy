import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import utc from "dayjs/plugin/utc";
import { FiArrowUp } from "solid-icons/fi";
import { Component, For, Show, createSignal, onMount } from "solid-js";
import clippy from "../../../../assets/clippy.png";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { HotkeyStore } from "../../../../store/hotkey-store";
import { HotkeyEvent } from "../../../../types/enums";
import { ListenEvent } from "../../../../types/tauri-listen";
import { listenEvent } from "../../../../utils/tauri";
import { BaseClipboard } from "./base-clipboard";

dayjs.extend(utc);
dayjs.extend(relativeTime);

export const Clipboards: Component = () => {
  const { globalHotkeyEvent, hotkeys } = HotkeyStore;
  const [scrollToTop, setScrollToTop] = createSignal(false);

  const onScroll = async () => {
    if (!ClipboardStore.clipboardRef()) return;

    const bottom =
      ClipboardStore.clipboardRef() &&
      ClipboardStore.clipboardRef()!.scrollHeight - ClipboardStore.clipboardRef()!.scrollTop ===
        ClipboardStore.clipboardRef()!.clientHeight;

    ClipboardStore.clipboardRef()!.scrollTop !== 0 ? setScrollToTop(true) : setScrollToTop(false);

    if (bottom && ClipboardStore.hasMore()) {
      ClipboardStore.setWhere((prev) => ({ ...prev, cursor: ClipboardStore.clipboards().length }));
      const newClipboards = await ClipboardStore.getClipboards();
      ClipboardStore.setClipboards((prev) => [...prev, ...newClipboards]);
    }
  };

  onMount(() => listenEvent(ListenEvent.ScrollToTop, () => ClipboardStore.clipboardRef()!.scrollTo(0, 0)));

  onMount(() => {
    window.addEventListener("keydown", ClipboardStore.handleKeyDown);
    return () => window.removeEventListener("keydown", ClipboardStore.handleKeyDown);
  });

  return (
    <Show
      when={ClipboardStore.clipboards().length}
      fallback={
        <div class="flex h-screen w-full flex-col items-center justify-center space-y-3 opacity-30">
          <img src={clippy} width="50%" alt="no clipboards" />
          <h2 class="text-2xl font-medium opacity-50">No Clipboards yet...</h2>
        </div>
      }
    >
      <div ref={ClipboardStore.setClipboardRef} onScroll={onScroll} class="overflow-y-auto pb-5">
        <Show when={scrollToTop()}>
          <button
            type="button"
            class="absolute bottom-5 right-4 z-10 rounded-full bg-neutral-600 px-2 py-1 hover:bg-gray-500"
            onClick={() => ClipboardStore.clipboardRef()!.scrollTo(0, 0)}
          >
            <div class="relative flex items-center justify-center py-1">
              <FiArrowUp class="text-xl !text-white dark:!text-white" />
              <Show when={globalHotkeyEvent()}>
                <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
                  {hotkeys().find((key) => key.event === HotkeyEvent.ScrollToTop)?.key}
                </div>
              </Show>
            </div>
          </button>
        </Show>

        <For each={ClipboardStore.clipboards()}>
          {(clipboardData, index) => (
            <BaseClipboard
              data={clipboardData}
              index={index()}
              isSelected={index() === ClipboardStore.selectedIndex()}
            />
          )}
        </For>
      </div>
    </Show>
  );
};
