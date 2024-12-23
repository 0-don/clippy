import { BsFiletypeHtml, BsImages, BsJournalRichtext } from "solid-icons/bs";
import { FiFileText } from "solid-icons/fi";
import { HiSolidCog8Tooth } from "solid-icons/hi";
import { VsFileBinary } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { formatBytes } from "../../../utils/helpers";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";

interface SettingsLimitsProps {}

const MAX_SIZE = 104_857_600;
const DEFAULT_SIZE = 10_485_760;

export const SettingsLimits: Component<SettingsLimitsProps> = ({}) => {
  return (
    <Show when={SettingsStore.settings()}>
      <TextBlock Icon={HiSolidCog8Tooth} title="Clipboard Limits">
        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <p>If set to 0 the clipboard type will be skipped entirely</p>
        </div>
        <div class="flex items-center justify-between gap-2 px-5 pb-5">
          <div class="flex items-center gap-2 truncate">
            <FiFileText />
            <h6 class="text-sm">Max text size ({formatBytes(SettingsStore.settings()?.max_text_size)})</h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            className="w-36"
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
            <h6 class="text-sm">Max html size ({formatBytes(SettingsStore.settings()?.max_html_size)})</h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            className="w-36"
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
            <h6 class="text-sm">Max rtf size ({formatBytes(SettingsStore.settings()?.max_rtf_size)})</h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            className="w-36"
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
            <h6 class="text-sm">Max file size ({formatBytes(SettingsStore.settings()?.max_file_size)})</h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            className="w-36"
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
            <h6 class="text-sm">Max image size ({formatBytes(SettingsStore.settings()?.max_image_size)})</h6>
          </div>

          <Input
            type="number"
            step="1"
            min={0}
            max={MAX_SIZE}
            className="w-36"
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
