import { FiFileText } from "solid-icons/fi";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../../types";
import { HotkeyNumber } from "../../../utils/hotkey-number";

interface RtfClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const RtfClipboard: Component<RtfClipboardProps> = (props) => {
  const { rtf } = props.data;

  return (
    <div class="flex min-w-0">
      <div class="flex items-center">
        <div class="relative" title={props.data.clipboard.id.toString()}>
          <FiFileText class="text-2xl text-zinc-700 dark:text-white" />
          <HotkeyNumber index={props.index} />
        </div>
      </div>
      <div class="mr-4 truncate px-4">
        <div class="flex">
          <p class="text-sm">{rtf?.data || " "}</p>
        </div>
      </div>
    </div>
  );
};
