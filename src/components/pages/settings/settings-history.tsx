import { BsDeviceHdd } from "solid-icons/bs";
import { FiTrash2 } from "solid-icons/fi";
import { SiSqlite } from "solid-icons/si";
import { Component, createResource } from "solid-js";
import { ClipboardType } from "../../../types/enums";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { formatBytes } from "../../../utils/helpers";
import { invokeCommand } from "../../../utils/tauri";
import { TextBlock } from "../../elements/text-block";

const CLIPBOARD_TYPES: { type: ClipboardType | null; label: string }[] = [
  { type: null, label: "Clear All" },
  { type: ClipboardType.Text, label: "Clear Text" },
  { type: ClipboardType.Html, label: "Clear Html" },
  { type: ClipboardType.Rtf, label: "Clear Rtf" },
  { type: ClipboardType.Image, label: "Clear Image" },
  { type: ClipboardType.File, label: "Clear File" },
];

export const SettingsHistory: Component = () => {
  const [databaseInfo, { refetch }] = createResource(() => invokeCommand(InvokeCommand.GetDbInfo));

  const handleClear = async (type: ClipboardType | null) => {
    await invokeCommand(InvokeCommand.ClearClipboards, { type });
    refetch();
  };

  return (
    <>
      <TextBlock Icon={SiSqlite} title="SQL Database Info">
        <ul class="mx-5 list-disc px-5 pb-5">
          <li>
            {`${databaseInfo()?.records} local items (${formatBytes(databaseInfo()?.size)}) are saved on this computer`}
          </li>
        </ul>
      </TextBlock>

      <TextBlock Icon={BsDeviceHdd} title="Storage Actions">
        <div class="flex w-full flex-wrap justify-center gap-2 px-5 pb-5">
          {CLIPBOARD_TYPES.map(({ type, label }) => (
            <button
              type="button"
              onClick={() => handleClear(type)}
              class="inline-flex items-center space-x-2 rounded bg-zinc-600 px-1 py-1 text-xs font-bold text-white hover:bg-zinc-700"
            >
              <FiTrash2 />
              <span>{label}</span>
            </button>
          ))}
        </div>
      </TextBlock>
    </>
  );
};
