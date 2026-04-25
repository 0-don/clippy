import Tooltip from "@corvu/tooltip";
import { BsFiletypeHtml, BsJournalRichtext } from "solid-icons/bs";
import { FiFileText, FiLink } from "solid-icons/fi";
import { Component, createEffect, createSignal } from "solid-js";
import { rgbCompatible } from "../../../../lib/colors";
import { invokeCommand } from "../../../../lib/tauri";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { SettingsStore } from "../../../../store/settings-store";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardTextType, ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { LANGUAGES } from "../../../../utils/constants";
import dayjs from "../../../../utils/dayjs";
import { ClipboardHeader } from "./clipboard-header";

interface TextClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const TextClipboard: Component<TextClipboardProps> = (props) => {
  const [fromNowString, setFromNowString] = createSignal(
    dayjs.utc(props.data.clipboard.created_at).fromNow(),
  );

  let type = ClipboardType.Text;

  let data = props.data.text?.data;
  let textType = props.data.text?.type as ClipboardTextType;

  if (props.data.text?.data && props.data.html?.data) {
    type = ClipboardType.Html;
    data = props.data.text?.data;
  }
  if (!props.data.text?.data && props.data.html?.data) {
    type = ClipboardType.Html;
    data = props.data.html.data;
  }
  if (props.data.text?.data && props.data.rtf?.data && props.data.html?.data) {
    type = ClipboardType.Rtf;
    data = props.data.text.data;
  }
  if (!props.data.text?.data && props.data.rtf?.data && props.data.html?.data) {
    type = ClipboardType.Rtf;
    data = props.data.rtf.data;
  }

  const getIcon = () => {
    if (type === ClipboardType.Html) {
      return BsFiletypeHtml;
    }
    if (type === ClipboardType.Rtf) {
      return BsJournalRichtext;
    }

    switch (textType) {
      case ClipboardTextType.Link:
        return FiLink;
      case ClipboardTextType.Hex:
        return () => (
          <div
            class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
            style={{
              "background-color": data?.includes("#") ? data : `#${data}`,
            }}
          />
        );
      case ClipboardTextType.Rgb:
        return () => (
          <div
            class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
            style={{ "background-color": rgbCompatible(data || "")! }}
          />
        );
      default:
        return FiFileText;
    }
  };

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: props.data.clipboard.id,
      type: ClipboardType.Text,
    });
  };

  createEffect(() => {
    dayjs.locale(SettingsStore.settings()?.language || LANGUAGES[0]);
    setFromNowString(dayjs.utc(props.data.clipboard.created_at).fromNow());
  });

  return (
    <Tooltip openDelay={1000}>
      <Tooltip.Trigger>
        <button type="button" onClick={handleClick} class="clipboard relative">
          <ClipboardHeader {...props} Icon={getIcon()} />

          <div class="min-w-0 flex-1">
            {props.data.clipboard.name ? (
              <>
                <p class="w-[calc(100vw-6.5rem)] truncate text-left text-sm font-medium">
                  {props.data.clipboard.name}
                </p>
                <p
                  class={`w-[calc(100vw-6.5rem)] text-left text-xs text-zinc-500 dark:text-zinc-400 ${
                    ClipboardStore.isExpanded(props.data.clipboard.id)
                      ? "max-h-96 overflow-y-auto wrap-break-word whitespace-pre-wrap"
                      : "truncate"
                  }`}
                >
                  {data}
                </p>
              </>
            ) : (
              <p
                class={`w-[calc(100vw-6.5rem)] text-left text-sm ${
                  ClipboardStore.isExpanded(props.data.clipboard.id)
                    ? "max-h-96 overflow-y-auto wrap-break-word whitespace-pre-wrap"
                    : "truncate"
                }`}
                title={
                  !props.data.html?.data && SettingsStore.settings()?.tooltip
                    ? data
                    : undefined
                }
              >
                {data}
              </p>
            )}
            <div
              class="text-left text-xs font-thin text-zinc-700 dark:text-zinc-300"
              title={new Date(props.data.clipboard.created_at).toLocaleString()}
            >
              {fromNowString()}
            </div>
          </div>
        </button>
      </Tooltip.Trigger>

      {SettingsStore.settings()?.tooltip && props.data.html?.data && (
        <Tooltip.Portal>
          <Tooltip.Content
            style={{ "max-width": "300px", "max-height": "300px" }}
            class="truncate rounded-md bg-white p-1 shadow-lg dark:bg-zinc-800"
          >
            <div innerHTML={props.data.html.data} />
          </Tooltip.Content>
        </Tooltip.Portal>
      )}
    </Tooltip>
  );
};
