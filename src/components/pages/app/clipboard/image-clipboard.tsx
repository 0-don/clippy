import { BsImages } from "solid-icons/bs";
import { Component, createEffect, createSignal } from "solid-js";
import { invokeCommand } from "../../../../lib/tauri";
import { SettingsStore } from "../../../../store/settings-store";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { formatBytes } from "../../../../utils";
import { LANGUAGES } from "../../../../utils/constants";
import dayjs from "../../../../utils/dayjs";
import { ClipboardHeader } from "./clipboard-header";

interface ImageClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const ImageClipboard: Component<ImageClipboardProps> = (props) => {
  let dbClickTimer: any;
  const [fromNowString, setFromNowString] = createSignal(dayjs.utc(props.data.clipboard.created_date).fromNow());

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    if (e.detail === 1) {
      dbClickTimer = setTimeout(async () => {
        await invokeCommand(InvokeCommand.CopyClipboard, {
          id: props.data.clipboard.id,
          type: ClipboardType.Image,
        });
      }, 200);
    }
  };

  const handleDoubleClick = async (e: MouseEvent) => {
    clearTimeout(dbClickTimer);
    e.stopPropagation();
    await invokeCommand(InvokeCommand.SaveClipboardImage, { id: props.data.clipboard.id });
  };

  createEffect(() => {
    dayjs.locale(SettingsStore.settings()?.language || LANGUAGES[0]);
    setFromNowString(dayjs.utc(props.data.clipboard.created_date).fromNow());
  });

  const imageInfo =
    props.data.image &&
    `${props.data.image.width}x${props.data.image.height} ${formatBytes(Number(props.data.image.size || "0"))}`;

  return (
    <button type="button" onClick={handleClick} onDblClick={handleDoubleClick} class="clipboard">
      <ClipboardHeader {...props} Icon={BsImages} />

      <div class="min-w-0 flex-1">
        {props.data.image?.thumbnail && (
          <img
            src={`data:image/*;base64,${props.data.image.thumbnail}`}
            class="max-h-20 w-[calc(100%-3rem)] rounded-md object-cover"
            alt={imageInfo}
            title={imageInfo}
          />
        )}
        <div
          class="text-left text-xs font-thin text-zinc-700 dark:text-zinc-300"
          title={dayjs.utc(props.data.clipboard.created_date).format()}
        >
          {fromNowString()}
        </div>
      </div>
    </button>
  );
};
