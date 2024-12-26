import { CgDisplayFlex } from "solid-icons/cg";
import { FiMoon } from "solid-icons/fi";
import { HiOutlineWindow, HiSolidCog8Tooth } from "solid-icons/hi";
import { IoLanguageOutline } from "solid-icons/io";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { TbTooltip } from "solid-icons/tb";
import { VsRocket } from "solid-icons/vs";
import { Component, Show } from "solid-js";
import { msg } from "../../../lib/i18n";
import { invokeCommand } from "../../../lib/tauri";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { HotkeyEvent, WebWindow } from "../../../types/enums";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { CLIPPY_POSITIONS, ClippyPosition, Language, LANGUAGES } from "../../../utils/constants";
import { Dropdown } from "../../elements/dropdown";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";
import { DarkMode } from "../../utils/dark-mode";
import { Shortcut } from "../../utils/shortcut";

interface SettingsGeneralProps {}

export const SettingsGeneral: Component<SettingsGeneralProps> = ({}) => {
  const { t } = useLanguage();

  return (
    <Show when={SettingsStore.settings()}>
      <TextBlock Icon={RiDeviceKeyboardFill} title={t("SETTINGS.GENERAL.KEYBOARD_SHORTCUT")}>
        <div class="mb-2 flex items-center space-x-2 px-5 pb-2.5">
          <Show when={HotkeyStore.getHotkey(HotkeyEvent.WindowDisplayToggle)}>
            {(hotkey) => <Shortcut hotkey={hotkey()} />}
          </Show>
        </div>
      </TextBlock>

      <TextBlock Icon={HiSolidCog8Tooth} title={t("SETTINGS.GENERAL.SYSTEM")}>
        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <VsRocket />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.START_CLIPPY_ON_STARTUP")}</h6>
          </div>
          <div>
            <Toggle
              checked={SettingsStore.settings()?.startup}
              onChange={async (startup) => SettingsStore.updateSettings({ ...SettingsStore.settings()!, startup })}
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <FiMoon class="dark:text-white" />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.SWITCH_THEME")}</h6>
          </div>
          <div>
            <DarkMode />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <TbTooltip />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.HTML_CLIPBOARD_TOOLTIP")}</h6>
          </div>
          <div>
            <Toggle
              checked={SettingsStore.settings()?.tooltip}
              onChange={async (tooltip: boolean) =>
                SettingsStore.updateSettings({ ...SettingsStore.settings()!, tooltip })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <HiOutlineWindow />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.CHANGE_WINDOW_POSITION")}</h6>
          </div>

          <Dropdown
            items={CLIPPY_POSITIONS.map((value) => ({
              value,
              label: msg(`MAIN.POSITION.${value.toUpperCase() as Uppercase<ClippyPosition>}`),
            }))}
            value={SettingsStore.settings()!.position}
            onChange={(position) => {
              SettingsStore.updateSettings({ ...SettingsStore.settings()!, position: position as ClippyPosition });
            }}
          />
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <IoLanguageOutline />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.CHANGE_LANGUAGE")}</h6>
          </div>

          <Dropdown
            items={LANGUAGES.map((value) => ({
              value,
              label: msg(`MAIN.LANGUAGE.${value.toUpperCase() as Uppercase<Language>}`),
            }))}
            value={SettingsStore.settings()!.language}
            onChange={(language) => {
              SettingsStore.updateSettings({ ...SettingsStore.settings()!, language: language as Language });
            }}
          />
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <CgDisplayFlex />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.WINDOW_SCALE")}</h6>
          </div>

          <Input
            type="number"
            step="0.01"
            min={0.5}
            max={2}
            value={SettingsStore.settings()!.display_scale}
            debounce={1000}
            onInput={async (e) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                display_scale: Number(parseFloat(e.target.value).toFixed(2)),
              });
              await invokeCommand(InvokeCommand.OpenNewWindow, { windowName: WebWindow.Settings, title: "Settings" });
            }}
          />
        </div>
      </TextBlock>
    </Show>
  );
};
