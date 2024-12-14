import { ClipboardWhere, ClipboardWithRelations, DatabaseInfo, Hotkey, Settings } from ".";
import { ClipboardType, WebWindow } from "./enums";

export enum InvokeCommand {
  // Clipboard commands
  GetClipboards = "get_clipboards",
  DeleteClipboard = "delete_clipboard",
  StarClipboard = "star_clipboard",
  CopyClipboard = "copy_clipboard",
  ClearClipboards = "clear_clipboards",
  SaveClipboardImage = "save_clipboard_image",

  // Hotkey commands
  GetHotkeys = "get_hotkeys",
  UpdateHotkey = "update_hotkey",
  StopHotkeys = "stop_hotkeys",

  // Settings commands
  GetSettings = "get_settings",
  UpdateSettings = "update_settings",
  ToggleAutostart = "toggle_autostart",

  OpenNewWindow = "open_new_window",
  OpenBrowserUrl = "open_browser_url",
  ExitApp = "exit_app",
  GetAppVersion = "get_app_version",
  GetDbInfo = "get_db_info",
  GetDbPath = "get_db_path",
  SyncClipboardHistory = "sync_clipboard_history",
}

export interface TauriInvokeCommands {
  // Clipboard commands
  [InvokeCommand.GetClipboards]: {
    args: ClipboardWhere;
    return: ClipboardWithRelations[];
  };
  [InvokeCommand.DeleteClipboard]: {
    args: { id: number };
    return: boolean;
  };
  [InvokeCommand.StarClipboard]: {
    args: { id: number; star: boolean };
    return: boolean;
  };
  [InvokeCommand.CopyClipboard]: {
    args: { id: number; type: ClipboardType };
    return: boolean;
  };
  [InvokeCommand.ClearClipboards]: {
    args: undefined;
    return: void;
  };
  [InvokeCommand.SaveClipboardImage]: {
    args: { id: number };
    return: void;
  };

  // Hotkey commands
  [InvokeCommand.GetHotkeys]: {
    args: undefined;
    return: Hotkey[];
  };
  [InvokeCommand.UpdateHotkey]: {
    args: { hotkey: Hotkey };
    return: boolean;
  };
  [InvokeCommand.StopHotkeys]: {
    args: undefined;
    return: void;
  };

  // Settings commands
  [InvokeCommand.GetSettings]: {
    args: undefined;
    return: Settings;
  };
  [InvokeCommand.UpdateSettings]: {
    args: { settings: Settings };
    return: void;
  };
  [InvokeCommand.ToggleAutostart]: {
    args: undefined;
    return: void;
  };

  // Window commands
  [InvokeCommand.OpenNewWindow]: {
    args: { windowName: WebWindow };
    return: void;
  };
  [InvokeCommand.OpenBrowserUrl]: {
    args: { url: string };
    return: void;
  };
  [InvokeCommand.ExitApp]: {
    args: undefined;
    return: void;
  };

  // App info commands
  [InvokeCommand.GetAppVersion]: {
    args: undefined;
    return: string;
  };
  [InvokeCommand.GetDbInfo]: {
    args: undefined;
    return: DatabaseInfo;
  };
  [InvokeCommand.GetDbPath]: {
    args: undefined;
    return: string;
  };
  [InvokeCommand.SyncClipboardHistory]: {
    args: undefined;
    return: void;
  };
}