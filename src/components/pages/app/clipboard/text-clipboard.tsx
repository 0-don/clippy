import { FiFileText, FiLink } from "solid-icons/fi";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardTextType } from "../../../../types/enums";
import { rgbCompatible } from "../../../../utils/colors";
import { HotkeyNumber } from "../../../utils/hotkey-number";

interface TextClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const TextClipboard: Component<TextClipboardProps> = (props) => {
  const { text } = props.data;

  const getIcon = () => {
    if (text?.type === ClipboardTextType.Link) {
      return <FiLink class="text-2xl text-zinc-700 dark:text-white" />;
    }
    if (text?.type === ClipboardTextType.Hex) {
      return (
        <div
          class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
          style={{
            "background-color": `${text.data.includes("#") ? text.data : `#${text.data}`}`,
          }}
        />
      );
    }
    if (text?.type === ClipboardTextType.Rgb) {
      return (
        <div
          class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
          style={{ "background-color": rgbCompatible(text.data)! }}
        />
      );
    }
    return <FiFileText class="text-2xl text-zinc-700 dark:text-white" />;
  };

  return (
    <div class="flex min-w-0">
      <div class="flex items-center">
        <div class="relative" title={props.data.clipboard.id.toString()}>
          {getIcon()}
          <HotkeyNumber index={props.index} />
        </div>
      </div>
      <div class="mr-4 truncate px-4">
        <div class="flex" title={props.data.text?.data || ""}>
          <p class="text-sm">{props.data.text?.data || " "}</p>
        </div>
      </div>
    </div>
  );
};
