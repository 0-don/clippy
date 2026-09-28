import type { DictionaryKey } from "../lib/i18n";
import { HotkeyEvent } from "../types/enums";

export const LANGUAGE_KEY = "lang";
export const MAX_SIZE = 104_857_600;
export const DEFAULT_SIZE = 10_485_760;

export const MIN_PASSWORD_LENGTH = 1;
export const MAX_PASSWORD_LENGTH = 128;
export const MIN_PATTERN_LENGTH = 1;
export const MAX_PATTERN_LENGTH = 128;
export const MIN_DESCRIPTION_LENGTH = 1;
export const MAX_DESCRIPTION_LENGTH = 128;

export const MAX_SYNC_LIMIT = 250;

export const SETTINGS_TAB = [
  "SETTINGS.TAB.GENERAL",
  "SETTINGS.TAB.BACKUP",
  "SETTINGS.TAB.ENCRYPTION",
  "SETTINGS.TAB.HISTORY",
  "SETTINGS.TAB.HOTKEYS",
  "SETTINGS.TAB.PATTERNS",
  "SETTINGS.TAB.COMMANDS",
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
export type PasteOnSelect = (typeof PASTE_ON_SELECT_MODES)[number];
export type CommandOutput = (typeof COMMAND_OUTPUTS)[number];
export type CommandOs = (typeof COMMAND_OS)[number];
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

export const PASTE_ON_SELECT_MODES = ["off", "paste", "type"] as const;

export const COMMAND_OUTPUTS = ["ignore", "copy", "paste"] as const;

// Values of std::env::consts::OS, which is what the backend matches scripts on.
export const COMMAND_OS = ["linux", "windows", "macos"] as const;

// Mirrors MAX_TEXT_PREVIEW in src-tauri/common/src/constants.rs: list payloads
// arrive truncated to this length.
export const MAX_TEXT_PREVIEW = 500;

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
  // "tray_left",
  // "tray_bottom_left",
  // "tray_right",
  // "tray_bottom_right",
  // "tray_center",
  // "tray_bottom_center",
] as const;

// Named color themes selectable in settings. Each maps to a [data-theme="..."]
// token block in styles.css. "neutral" is the default.
export const THEMES = [
  "neutral",
  "rose",
  "pink",
  "blue",
  "emerald",
  "violet",
  "amber",
] as const;
export type ThemeName = (typeof THEMES)[number];

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
  "F13",
  "F14",
  "F15",
  "F16",
  "F17",
  "F18",
  "F19",
  "F20",
  "F21",
  "F22",
  "F23",
  "F24",
  "ArrowUp",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
  "Home",
  "End",
  "PageUp",
  "PageDown",
  "Insert",
  "Delete",
  "Space",
  "Enter",
  "Tab",
  "Backspace",
  "Escape",
  "PrintScreen",
  "ScrollLock",
  "Pause",
  "Minus",
  "Equal",
  "BracketLeft",
  "BracketRight",
  "Backslash",
  "Semicolon",
  "Quote",
  "Comma",
  "Period",
  "Slash",
  "Backquote",
  "Numpad0",
  "Numpad1",
  "Numpad2",
  "Numpad3",
  "Numpad4",
  "Numpad5",
  "Numpad6",
  "Numpad7",
  "Numpad8",
  "Numpad9",
  "NumpadAdd",
  "NumpadSubtract",
  "NumpadMultiply",
  "NumpadDivide",
  "NumpadDecimal",
  "NumpadEnter",
  "MediaPlayPause",
  "MediaStop",
  "MediaTrackNext",
  "MediaTrackPrevious",
  "AudioVolumeUp",
  "AudioVolumeDown",
  "AudioVolumeMute",
] as const;
