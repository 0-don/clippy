import type { DictionaryKey } from "../lib/i18n";
import { HotkeyEvent } from "../types/enums";

export const LANGUAGE_KEY = "lang";
export const MAX_SIZE = 104_857_600;
export const DEFAULT_SIZE = 10_485_760;

export const SETTINGS_TAB = [
  "SETTINGS.TAB.GENERAL",
  "SETTINGS.TAB.BACKUP",
  "SETTINGS.TAB.HISTORY",
  "SETTINGS.TAB.HOTKEYS",
  "SETTINGS.TAB.LIMITS",
] as const satisfies readonly DictionaryKey[];

export const VIEW_MORE_NAMES = [
  "MAIN.HOTKEY.SYNC_CLIPBOARD_HISTORY",
  "MAIN.HOTKEY.SETTINGS",
  "MAIN.HOTKEY.ABOUT",
  "MAIN.HOTKEY.EXIT",
] as const satisfies readonly DictionaryKey[];
export const TAB_NAMES = [
  "MAIN.HOTKEY.RECENT_CLIPBOARDS",
  "MAIN.HOTKEY.STARRED_CLIPBOARDS",
  "MAIN.HOTKEY.HISTORY",
  "MAIN.HOTKEY.VIEW_MORE",
] as const satisfies readonly DictionaryKey[];

export const TABS = [
  HotkeyEvent.RecentClipboards,
  HotkeyEvent.StarredClipboards,
  HotkeyEvent.History,
  HotkeyEvent.ViewMore,
] as const;

export type SettingsTabName = (typeof SETTINGS_TAB)[number];

export type ViewMoreName = (typeof VIEW_MORE_NAMES)[number];
export type TabName = (typeof TAB_NAMES)[number];
export type Tab = (typeof TABS)[number];
export type Language = (typeof LANGUAGES)[number];
export type ClippyPosition = (typeof CLIPPY_POSITIONS)[number];
export type GlobalShortcutKeys = (typeof GLOBAL_SHORTCUT_KEYS)[number];

export const LANGUAGES = [
  "en", // English - ~1.1 billion speakers
  "zh", // Mandarin - ~1.1 billion speakers
  "hi", // Hindi - ~600 million speakers
  "es", // Spanish - ~550 million speakers
  "fr", // French - ~320 million speakers
  "ar", // Arabic - ~310 million speakers
  "bn", // Bengali - ~260 million speakers
  "pt", // Portuguese - ~240 million speakers
  "ru", // Russian - ~230 million speakers
  "ur", // Urdu - ~200 million speakers
  "ja", // Japanese - ~170 million speakers
  "de", // German - ~160 million speakers
  "ko", // Korean - ~130 million speakers
  "vi", // Vietnamese - ~100 million speakers
  "tr", // Turkish - ~95 million speakers
  "it", // Italian - ~85 million speakers
  "th", // Thai - ~80 million speakers
  "pl", // Polish - ~45 million speakers
  "nl", // Dutch - ~30 million speakers
] as const;

export const CLIPPY_POSITIONS = [
  "cursor",
  "top_left",
  "top_right",
  "bottom_left",
  "bottom_right",
  "top_center",
  "bottom_center",
  "left_center",
  "right_center",
  "center",
  "tray_left",
  "tray_bottom_left",
  "tray_right",
  "tray_bottom_right",
  "tray_center",
  "tray_bottom_center",
] as const;

export const GLOBAL_SHORTCUT_KEYS = [
  "none",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "0",
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
  "F1",
  "F2",
  "F3",
  "F4",
  "F5",
  "F6",
  "F7",
  "F8",
  "F9",
  "F10",
  "F11",
  "F12",
] as const;
