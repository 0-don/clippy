import { FiArrowUp } from "solid-icons/fi";
import { ImSpinner2 } from "solid-icons/im";
import { Component, For, Show, createSignal, onSettled } from "solid-js";
import clippy from "../../../../assets/clippy.png";
import { listenEvent } from "../../../../lib/tauri";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { HotkeyStore } from "../../../../store/hotkey-store";
import { HotkeyEvent } from "../../../../types/enums";
import { ListenEvent } from "../../../../types/tauri-listen";
import { useLanguage } from "../../../provider/language-provider";
import { BaseClipboard } from "./base-clipboard";

export const Clipboards: Component = () => {
  const { t } = useLanguage();
  const [scrollToTop, setScrollToTop] = createSignal(false);

  const onScroll = async () => {
    const ref = ClipboardStore.clipboardRef();
    if (!ref) return;

    const bottom = ref.scrollHeight - ref.scrollTop - ref.clientHeight < 50;

    ref.scrollTop !== 0 ? setScrollToTop(true) : setScrollToTop(false);

    if (bottom && ClipboardStore.hasMore() && !ClipboardStore.isSearching()) {
      await ClipboardStore.loadMore();
    }
  };

  onSettled(() => {
    const unlisten = listenEvent(ListenEvent.ScrollToTop, () =>
      ClipboardStore.clipboardRef()?.scrollTo(0, 0),
    );
    window.addEventListener("keydown", ClipboardStore.handleKeyDown);
    return () => {
      void unlisten.then((stop) => stop());
      window.removeEventListener("keydown", ClipboardStore.handleKeyDown);
    };
  });

  return (
    <Show
      when={ClipboardStore.clipboards().length}
      fallback={
        <Show
          when={!ClipboardStore.isSearching()}
          fallback={
            <div class="flex h-screen w-full items-center justify-center">
              <ImSpinner2 class="text-foreground animate-spin text-4xl opacity-30" />
            </div>
          }
        >
          <div class="flex h-screen w-full flex-col items-center justify-center space-y-3 opacity-30">
            <img src={clippy} width="50%" alt="no clipboards" />
            <h2 class="text-center text-2xl font-medium opacity-50">
              {t("CLIPBOARD.NO_CLIPBOARDS_YET")}
            </h2>
          </div>
        </Show>
      }
    >
      <div
        ref={ClipboardStore.setClipboardRef}
        onScroll={onScroll}
        class="overflow-x-hidden overflow-y-auto pb-5"
      >
        <Show when={scrollToTop()}>
          <button
            type="button"
            class="bg-secondary hover:bg-muted absolute right-4 bottom-5 z-10 rounded-full px-2 py-1"
            onClick={ClipboardStore.init}
          >
            <div class="relative flex items-center justify-center py-1">
              <FiArrowUp class="text-foreground! text-xl" />
              <Show when={HotkeyStore.globalHotkeyEvent()}>
                <div class="bg-card absolute top-0 left-0 -mt-3 -ml-3 rounded-xs px-1 text-[12px] font-semibold">
                  {
                    HotkeyStore.hotkeys().find(
                      (key) => key.event === HotkeyEvent.ScrollToTop,
                    )?.key
                  }
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
