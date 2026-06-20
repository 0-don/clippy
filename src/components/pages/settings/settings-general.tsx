import { CgDisplayFlex } from "solid-icons/cg";
import { FiDroplet, FiMoon, FiSliders } from "solid-icons/fi";
import { HiOutlineWindow, HiSolidCog8Tooth } from "solid-icons/hi";
import { IoColorPaletteOutline, IoLanguageOutline } from "solid-icons/io";
import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { TbOutlineMaximize, TbOutlineTooltip } from "solid-icons/tb";
import { VsRocket } from "solid-icons/vs";
import { Component, onCleanup, Show } from "solid-js";
import { msg } from "../../../lib/i18n";
import { invokeCommand } from "../../../lib/tauri";
import { AppStore } from "../../../store/app-store";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { Settings } from "../../../types";
import { HotkeyEvent, WebWindow } from "../../../types/enums";
import { InvokeCommand } from "../../../types/tauri-invoke";
import {
  CLIPPY_POSITIONS,
  ClippyPosition,
  Language,
  LANGUAGES,
  THEMES,
  ThemeName,
} from "../../../utils/constants";
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

  // Glass sliders: preview instantly (setSettings, no IO), persist once on pause.
  let glassPersistTimer: ReturnType<typeof setTimeout> | undefined;
  const onGlassSlide = (patch: Partial<Settings>) => {
    const next = { ...SettingsStore.settings()!, ...patch };
    SettingsStore.setSettings(next); // live CSS-var preview via the dark-mode effect
    clearTimeout(glassPersistTimer);
    glassPersistTimer = setTimeout(
      () => SettingsStore.updateSettings(next),
      400,
    );
  };

  onCleanup(() => clearTimeout(glassPersistTimer));

  return (
    <Show when={SettingsStore.settings()}>
      <TextBlock
        Icon={RiDeviceKeyboardFill}
        title={t("SETTINGS.GENERAL.KEYBOARD_SHORTCUT")}
      >
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
            <h6 class="text-sm">
              {t("SETTINGS.GENERAL.START_CLIPPY_ON_STARTUP")}
            </h6>
          </div>
          <div>
            <Toggle
              checked={SettingsStore.settings()?.startup}
              onChange={async (startup) =>
                SettingsStore.updateSettings({
                  ...SettingsStore.settings()!,
                  startup,
                })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <FiMoon class="text-foreground" />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.SWITCH_THEME")}</h6>
          </div>
          <div>
            <DarkMode />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <IoColorPaletteOutline />
            <h6 class="text-sm">{t("SETTINGS.GENERAL.COLOR_THEME")}</h6>
          </div>

          <Dropdown
            items={THEMES.map((value) => ({
              value,
              label: msg(
                `MAIN.THEME.${value.toUpperCase() as Uppercase<ThemeName>}`,
              ),
            }))}
            value={SettingsStore.settings()!.theme || "neutral"}
            onChange={(theme) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                theme: theme as ThemeName,
              });
            }}
          />
        </div>

        {/* Native window glass: Windows + macOS only (Linux compositor controls blur). */}
        <Show when={AppStore.os() !== "linux"}>
          <div class="flex items-center justify-between space-x-2 px-5 pb-5">
            <div class="flex items-center space-x-2 truncate">
              <FiDroplet />
              <h6 class="text-sm">{t("SETTINGS.GENERAL.GLASS_EFFECT")}</h6>
            </div>
            <div>
              <Toggle
                checked={SettingsStore.settings()?.glass}
                onChange={async (glass: boolean) =>
                  SettingsStore.updateSettings({
                    ...SettingsStore.settings()!,
                    glass,
                  })
                }
              />
            </div>
          </div>

          {/* Glass tint: only relevant while glass is on. 0..1, drives the CSS
              surface alpha live (applyAppearance reacts to the settings signal). On
              drag we setSettings() for an instant preview with NO backend call, then
              persist once (debounced) on pause — otherwise every pixel of the drag
              would hammer the DB + autostart toggle. */}
          <Show when={SettingsStore.settings()?.glass}>
            <div class="flex items-center justify-between space-x-2 px-5 pb-5">
              <div class="flex items-center space-x-2 truncate">
                <FiSliders />
                <h6 class="text-sm">{t("SETTINGS.GENERAL.GLASS_OPACITY")}</h6>
              </div>
              <div class="flex items-center space-x-2">
                <input
                  type="range"
                  min="0"
                  max="1"
                  step="0.01"
                  class="accent-primary"
                  value={SettingsStore.settings()!.glass_opacity}
                  onInput={(e) =>
                    onGlassSlide({ glass_opacity: Number(e.currentTarget.value) })
                  }
                />
                <span class="w-9 text-right text-xs text-muted-foreground tabular-nums">
                  {Math.round(SettingsStore.settings()!.glass_opacity * 100)}%
                </span>
              </div>
            </div>
          </Show>
        </Show>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <TbOutlineTooltip />
            <h6 class="text-sm">
              {t("SETTINGS.GENERAL.HTML_CLIPBOARD_TOOLTIP")}
            </h6>
          </div>
          <div>
            <Toggle
              checked={SettingsStore.settings()?.tooltip}
              onChange={async (tooltip: boolean) =>
                SettingsStore.updateSettings({
                  ...SettingsStore.settings()!,
                  tooltip,
                })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex flex-col truncate">
            <div class="flex items-center space-x-2">
              <TbOutlineMaximize />
              <h6 class="text-sm">
                {t("SETTINGS.GENERAL.SUPPRESS_HOTKEY_ON_FULLSCREEN")}
              </h6>
            </div>
            <p class="ml-6 text-xs text-muted-foreground">
              {t("SETTINGS.GENERAL.SUPPRESS_HOTKEY_ON_FULLSCREEN_INFO")}
            </p>
          </div>
          <div>
            <Toggle
              checked={SettingsStore.settings()?.suppress_hotkey_on_fullscreen}
              onChange={async (suppress_hotkey_on_fullscreen: boolean) =>
                SettingsStore.updateSettings({
                  ...SettingsStore.settings()!,
                  suppress_hotkey_on_fullscreen,
                })
              }
            />
          </div>
        </div>

        <div class="flex items-center justify-between space-x-2 px-5 pb-5">
          <div class="flex items-center space-x-2 truncate">
            <HiOutlineWindow />
            <h6 class="text-sm">
              {t("SETTINGS.GENERAL.CHANGE_WINDOW_POSITION")}
            </h6>
          </div>

          <Dropdown
            items={CLIPPY_POSITIONS.map((value) => ({
              value,
              label: msg(
                `MAIN.POSITION.${value.toUpperCase() as Uppercase<ClippyPosition>}`,
              ),
            }))}
            value={SettingsStore.settings()!.position}
            onChange={(position) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                position: position as ClippyPosition,
              });
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
              label: msg(
                `MAIN.LANGUAGE.${value.toUpperCase() as Uppercase<Language>}`,
              ),
            }))}
            value={SettingsStore.settings()!.language}
            onChange={(language) => {
              SettingsStore.updateSettings({
                ...SettingsStore.settings()!,
                language: language as Language,
              });
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
              await invokeCommand(InvokeCommand.OpenNewWindow, {
                windowName: WebWindow.Settings,
              });
            }}
          />
        </div>
      </TextBlock>
    </Show>
  );
};
