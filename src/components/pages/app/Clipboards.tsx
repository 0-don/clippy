import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import utc from "dayjs/plugin/utc";
import { BsImages } from "solid-icons/bs";
import { FiArrowUp, FiFileText, FiLink } from "solid-icons/fi";
import { IoTrashOutline } from "solid-icons/io";
import { VsStarFull } from "solid-icons/vs";
import { Component, For, Show, createSignal, onMount } from "solid-js";
import { ClipboardModel } from "../../../@types";
import clippy from "../../../assets/clippy.png";
import ClipboardStore from "../../../store/ClipboardStore";
import HotkeyStore from "../../../store/HotkeyStore";
import { rgbCompatible } from "../../../utils/colors";
import { formatBytes } from "../../../utils/helpers";

dayjs.extend(utc);
dayjs.extend(relativeTime);

interface ClipboardsProps {}

export const Clipboards: Component<ClipboardsProps> = ({}) => {
  let dbClickTimer: any;

  const [scrollToTop, setScrollToTop] = createSignal(false);

  const { clipboards, setClipboards, getClipboards, setWhere, clipboardRef, setClipboardRef } = ClipboardStore;
  const { globalHotkeyEvent, hotkeys } = HotkeyStore;

  const onScroll = async () => {
    if (!clipboardRef()) return;

    const bottom =
      clipboardRef() && clipboardRef()!.scrollHeight - clipboardRef()!.scrollTop === clipboardRef()!.clientHeight;

    clipboardRef()!.scrollTop !== 0 ? setScrollToTop(true) : setScrollToTop(false);

    if (bottom) {
      setWhere((prev) => ({ ...prev, cursor: clipboards().length }));
      const newClipboards = await getClipboards();
      setClipboards((prev) => [...prev, ...newClipboards]);
    }
  };

  onMount(() => listen("scroll_to_top", () => clipboardRef()!.scrollTo(0, 0)));

  const IconFunctions = (clipboard: ClipboardModel) => (
    <>
      <VsStarFull
        onClick={async (e) => {
          e.stopPropagation();
          await invoke<boolean>("star_clipboard", {
            id: clipboard.id,
            star: !clipboard.star,
          });
          setClipboards((prev) =>
            prev.map((o) => (o.clipboard.id === clipboard.id ? { ...o, star: !o.clipboard.star } : o))
          );
        }}
        class={`${
          clipboard.star ? "text-yellow-400 dark:text-yellow-300" : "hidden text-zinc-700"
        } z-10 hover:text-yellow-400 group-hover:block dark:text-white dark:hover:text-yellow-300`}
      />
      <IoTrashOutline
        onClick={async (e) => {
          e.stopPropagation();
          if (await invoke<boolean>("delete_clipboard", { id: clipboard.id })) {
            setClipboards((prev) => prev.filter((o) => o.clipboard.id !== clipboard.id));
          }
        }}
        class="hidden text-zinc-700 hover:text-red-600 group-hover:block dark:text-white dark:hover:text-red-600"
      />
    </>
  );

  return (
    <Show
      when={clipboards().length}
      fallback={
        <div class="flex h-screen w-full flex-col items-center justify-center space-y-3 opacity-30">
          <img src={clippy} width="50%" alt="no clipboards" />
          <h2 class="text-2xl font-medium opacity-50">No Clipboards yet...</h2>
        </div>
      }
    >
      <div ref={(ref) => setClipboardRef(ref)} onScroll={onScroll} class="overflow-y-auto pb-5">
        <Show when={scrollToTop()}>
          <button
            type="button"
            class="absolute bottom-5 right-4 z-10 rounded-full bg-neutral-600 px-2 py-1 hover:bg-gray-500"
            onClick={() => clipboardRef()!.scrollTo(0, 0)}
          >
            <div class="relative flex items-center justify-center py-1">
              <FiArrowUp class="text-xl !text-white dark:!text-white" />
              <Show when={globalHotkeyEvent()}>
                <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-600 px-1 text-[12px] font-semibold">
                  {hotkeys().find((key) => key.event === "scroll_to_top")?.key}
                </div>
              </Show>
            </div>
          </button>
        </Show>

        <For each={clipboards()}>
          {({ clipboard, file, html, image, rtf, text }, index) => {
            return (
              <button
                type="button"
                class="group relative w-full cursor-pointer select-none px-3 hover:bg-zinc-200 dark:hover:bg-neutral-700"
                onClick={(e) => {
                  e.stopPropagation();
                  if (e.detail === 1) {
                    dbClickTimer = setTimeout(
                      async () => await invoke("copy_clipboard", { id: clipboard.id }),
                      clipboard.types.includes("image") ? 200 : 0
                    );
                  }
                }}
                onDblClick={async (e) => {
                  clearTimeout(dbClickTimer);
                  e.stopPropagation();
                  if (!clipboard.types.includes("image")) return;
                  await invoke("save_clipboard_image", { id: clipboard.id });
                }}
              >
                <div class="flex justify-between py-3">
                  <div class="flex min-w-0">
                    <div class="flex items-center">
                      <div class="relative" title={clipboard.id.toString()}>
                        {text?.type === "link" && <FiLink class="text-2xl text-zinc-700 dark:text-white" />}
                        {text?.type === "text" && <FiFileText class="text-2xl text-zinc-700 dark:text-white" />}
                        {clipboard.types.includes("image") && (
                          <BsImages class="text-2xl text-zinc-700 dark:text-white" />
                        )}
                        {text?.type === "hex" && (
                          <div
                            class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
                            style={{
                              "background-color": `${text.data.includes("#") ? `${text.data}` : `#${text.data}`}`,
                            }}
                          />
                        )}
                        {text?.type === "rgb" && (
                          <div
                            class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
                            style={{ "background-color": `${rgbCompatible(text.data)}` }}
                          />
                        )}
                        <Show when={globalHotkeyEvent()}>
                          <div class="absolute left-0 top-0 -ml-3 -mt-3 rounded-sm bg-zinc-800 px-1 text-[12px] font-semibold text-white dark:bg-zinc-600">
                            {index() + 1 < 10 && index() + 1}
                          </div>
                        </Show>
                      </div>
                    </div>
                    <div class="mr-4 truncate px-4">
                      {image?.thumbnail ? (
                        <img
                          src={`data:image/*;base64,${image.thumbnail}`}
                          width={image.width || 0}
                          height={image.height || 0}
                          class="max-h-52"
                          alt={`${image.width}x${image.height} ${image.size}`}
                          title={`${image.width}x${image.height} ${formatBytes(Number(image.size || "0"))}`}
                        />
                      ) : (
                        <div class="flex" title={text?.data || ""}>
                          <p class="text-sm">{text?.data || " "}</p>
                        </div>
                      )}
                      <div class="text-left text-xs text-zinc-400">{dayjs.utc(clipboard.created_date).fromNow()}</div>
                    </div>
                  </div>
                  <div class="absolute bottom-0 right-0 top-0 m-2 flex w-4">
                    <div class="flex w-full flex-col items-end justify-between">{IconFunctions(clipboard)}</div>
                  </div>
                </div>
                <hr class="border-zinc-400 dark:border-zinc-700" />
              </button>
            );
          }}
        </For>
      </div>
    </Show>
  );
};
