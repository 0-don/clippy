import { BsFiletypeHtml, BsImages, BsJournalRichtext } from "solid-icons/bs";
import { FiFileText } from "solid-icons/fi";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { VsFileBinary } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { formatBytes } from "../../../utils";
import { DEFAULT_SIZE, MAX_SIZE } from "../../../utils/constants";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { useLanguage } from "../../provider/language-provider";

interface SettingsLimitsProps {}

export const SettingsLimits: Component<SettingsLimitsProps> = ({}) => {
  const { t } = useLanguage();

  return (
    <Show when={SettingsStore.settings()}>
      <TextBlock Icon={HiSolidCog8Tooth} title={t("SETTINGS.LIMITS.CLIPBOARD_LIMITS")}>
        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <p class="text-sm text-zinc-700 dark:text-zinc-400">{t("SETTINGS.LIMITS.IF_SET_TO_ZERO")}</p>
        </div>
        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <div class="flex items-center gap-2 truncate">
            <FiFileText />
            <h6 class="text-sm">
              {t("SETTINGS.LIMITS.MAX_TEXT_SIZE")} ({formatBytes(SettingsStore.settings()?.max_text_size)})
            </h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            class="w-36"
            value={SettingsStore.settings()?.max_text_size || DEFAULT_SIZE}
            debounce={1000}
            onInput={async (e) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                max_text_size: Number(e.target.value),
              });
            }}
          />
        </div>
        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <div class="flex items-center gap-2 truncate">
            <BsFiletypeHtml />
            <h6 class="text-sm">
              {t("SETTINGS.LIMITS.MAX_HTML_SIZE")} ({formatBytes(SettingsStore.settings()?.max_html_size)})
            </h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            class="w-36"
            value={SettingsStore.settings()?.max_html_size || DEFAULT_SIZE}
            debounce={1000}
            onInput={async (e) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                max_html_size: Number(e.target.value),
              });
            }}
          />
        </div>
        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <div class="flex items-center gap-2 truncate">
            <BsJournalRichtext />
            <h6 class="text-sm">
              {t("SETTINGS.LIMITS.MAX_RTF_SIZE")} ({formatBytes(SettingsStore.settings()?.max_rtf_size)})
            </h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            class="w-36"
            value={SettingsStore.settings()?.max_rtf_size || DEFAULT_SIZE}
            debounce={1000}
            onInput={async (e) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                max_rtf_size: Number(e.target.value),
              });
            }}
          />
        </div>

        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <div class="flex items-center gap-2 truncate">
            <VsFileBinary />
            <h6 class="text-sm">
              {t("SETTINGS.LIMITS.MAX_FILE_SIZE")} ({formatBytes(SettingsStore.settings()?.max_file_size)})
            </h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            class="w-36"
            value={SettingsStore.settings()?.max_file_size || DEFAULT_SIZE}
            debounce={1000}
            onInput={async (e) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                max_file_size: Number(e.target.value),
              });
            }}
          />
        </div>

        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <div class="flex items-center gap-2 truncate">
            <BsImages />
            <h6 class="text-sm">
              {t("SETTINGS.LIMITS.MAX_IMAGE_SIZE")}({formatBytes(SettingsStore.settings()?.max_image_size)})
            </h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            class="w-36"
            value={SettingsStore.settings()?.max_image_size || DEFAULT_SIZE}
            debounce={1000}
            onInput={async (e) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                max_image_size: Number(e.target.value),
              });
            }}
          />
        </div>
      </TextBlock>
    </Show>
  );
};
