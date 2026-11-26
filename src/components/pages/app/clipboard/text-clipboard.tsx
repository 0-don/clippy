import { BsFiletypeHtml, BsJournalRichtext } from "solid-icons/bs";
import { FiFileText, FiLink } from "solid-icons/fi";
import { Component, createMemo } from "solid-js";
import { rgbCompatible } from "../../../../lib/colors";
import { ClipboardStore } from "../../../../store/clipboard-store";
import { SettingsStore } from "../../../../store/settings-store";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardTextType, ClipboardType } from "../../../../types/enums";
import { copyEntry } from "../../../../utils/clipboard-actions";
import { LANGUAGES } from "../../../../utils/constants";
import dayjs from "../../../../utils/dayjs";
import { HtmlTooltip } from "../../../elements/html-tooltip";
import { ClipboardHeader } from "./clipboard-header";

interface TextClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const TextClipboard: Component<TextClipboardProps> = (props) => {
  const fromNow = () =>
    dayjs
      .utc(props.data.clipboard.created_at)
      .locale(SettingsStore.settings()?.language || LANGUAGES[0])
      .fromNow();

  const content = createMemo(() => {
    const text = props.data.text?.data;
    const html = props.data.html?.data;
    const rtf = props.data.rtf?.data;

    if (rtf && html) return { type: ClipboardType.Rtf, data: text || rtf };
    if (html) return { type: ClipboardType.Html, data: text || html };
    return { type: ClipboardType.Text, data: text };
  });

  const data = () => content().data;

  const getIcon = () => {
    if (content().type === ClipboardType.Html) {
      return BsFiletypeHtml;
    }
    if (content().type === ClipboardType.Rtf) {
      return BsJournalRichtext;
    }

    switch (props.data.text?.type) {
      case ClipboardTextType.Link:
        return FiLink;
      case ClipboardTextType.Hex:
        return () => (
          <div
            class="border-border h-5 w-5 rounded-md border border-solid"
            style={{
              "background-color": data()?.includes("#") ? data() : `#${data()}`,
            }}
          />
        );
      case ClipboardTextType.Rgb:
        return () => (
          <div
            class="border-border h-5 w-5 rounded-md border border-solid"
            style={{ "background-color": rgbCompatible(data() || "")! }}
          />
        );
      default:
        return FiFileText;
    }
  };

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await copyEntry(props.data.clipboard.id, ClipboardType.Text);
  };

  const expandedClass = () =>
    ClipboardStore.isExpanded(props.data.clipboard.id)
      ? "max-h-96 overflow-y-auto wrap-break-word whitespace-pre-wrap"
      : "truncate";

  return (
    <HtmlTooltip
      delay={1000}
      html={
        SettingsStore.settings()?.tooltip ? props.data.html?.data : undefined
      }
    >
      <button type="button" onClick={handleClick} class="clipboard relative">
        <ClipboardHeader {...props} Icon={getIcon()} />

        <div class="min-w-0 flex-1">
          {props.data.clipboard.name ? (
            <>
              <p class="w-[calc(100vw-6.5rem)] truncate text-left text-sm font-medium">
                {props.data.clipboard.name}
              </p>
              <p
                class={[
                  "text-muted-foreground w-[calc(100vw-6.5rem)] text-left text-xs",
                  expandedClass(),
                ]}
              >
                {data()}
              </p>
            </>
          ) : (
            <p
              class={[
                "w-[calc(100vw-6.5rem)] text-left text-sm",
                expandedClass(),
              ]}
              title={
                !props.data.html?.data && SettingsStore.settings()?.tooltip
                  ? data()
                  : undefined
              }
            >
              {data()}
            </p>
          )}
          <div
            class="text-muted-foreground text-left text-xs font-thin"
            title={new Date(props.data.clipboard.created_at).toLocaleString()}
          >
            {fromNow()}
          </div>
        </div>
      </button>
    </HtmlTooltip>
  );
};
