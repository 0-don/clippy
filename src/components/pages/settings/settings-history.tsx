import { BsDeviceHdd } from "solid-icons/bs";
import { FiTrash2 } from "solid-icons/fi";
import { SiSqlite } from "solid-icons/si";
import { Component, createResource } from "solid-js";
import { DictionaryKey } from "../../../lib/i18n";
import { invokeCommand } from "../../../lib/tauri";
import { ClipboardType } from "../../../types/enums";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { formatBytes } from "../../../utils";
import { TextBlock } from "../../elements/text-block";
import { useLanguage } from "../../provider/language-provider";

const CLIPBOARD_TYPES: { type: ClipboardType | null; label: DictionaryKey }[] = [
  { type: null, label: "SETTINGS.HISTORY.CLEAR_ALL" },
  { type: ClipboardType.Text, label: "SETTINGS.HISTORY.CLEAR_TEXT" },
  { type: ClipboardType.Html, label: "SETTINGS.HISTORY.CLEAR_HTML" },
  { type: ClipboardType.Rtf, label: "SETTINGS.HISTORY.CLEAR_RTF" },
  { type: ClipboardType.Image, label: "SETTINGS.HISTORY.CLEAR_IMAGE" },
  { type: ClipboardType.File, label: "SETTINGS.HISTORY.CLEAR_FILE" },
];

export const SettingsHistory: Component = () => {
  const { t } = useLanguage();
  const [databaseInfo, { refetch }] = createResource(() => invokeCommand(InvokeCommand.GetDbInfo));

  const handleClear = async (type: ClipboardType | null) => {
    await invokeCommand(InvokeCommand.ClearClipboards, { type });
    refetch();
  };

  return (
    <>
      <TextBlock Icon={SiSqlite} title={t("SETTINGS.HISTORY.SQL_DATABASE_INFO")}>
        <ul class="mx-5 list-disc px-5 pb-5">
          <li>
            {t("SETTINGS.HISTORY.DATABASE_INFO", {
              records: databaseInfo()?.records || 0,
              size: formatBytes(databaseInfo()?.size),
            })}
          </li>
        </ul>
      </TextBlock>

      <TextBlock Icon={BsDeviceHdd} title={t("SETTINGS.HISTORY.STORAGE_ACTIONS")}>
        <div class="flex w-full flex-wrap justify-center gap-2 px-5 pb-5">
          {CLIPBOARD_TYPES.map(({ type, label }) => (
            <button
              type="button"
              onClick={() => handleClear(type)}
              class="inline-flex items-center space-x-2 rounded bg-zinc-600 px-1 py-1 text-xs font-bold text-white hover:bg-zinc-700"
            >
              <FiTrash2 />
              <span>{t(label)}</span>
            </button>
          ))}
        </div>
      </TextBlock>
    </>
  );
};
