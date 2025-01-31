import { ClipboardResponse, ClipboardWhere, DatabaseInfo, Hotkey, Settings } from ".";
import { ClipboardType, FolderLocation, PasswordAction, WebWindow } from "./enums";

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
  ChangeClipboardDbLocation = "change_clipboard_db_location",
  ResetClipboardDbLocation = "reset_clipboard_db_location",

  // Window commands
  OpenNewWindow = "open_new_window",
  OpenBrowserUrl = "open_browser_url",
  ExitApp = "exit_app",
  OpenFolder = "open_folder",

  // Sync commands
  SyncAuthenticateToggle = "sync_authenticate_toggle",
  SyncLimitChange = "sync_limit_change",

  // Cipher commands
  EnableEncryption = "enable_encryption",
  DisableEncryption = "disable_encryption",
  PasswordUnlock = "password_unlock",

  // App info commands
  GetAppVersion = "get_app_version",
  GetDbInfo = "get_db_info",
  GetDbPath = "get_db_path",
  GetConfigPath = "get_config_path",
}

export interface TauriInvokeCommands {
  // Clipboard commands
  [InvokeCommand.GetClipboards]: {
    args: ClipboardWhere;
    return: ClipboardResponse;
  };
  [InvokeCommand.DeleteClipboard]: {
    args: { id: number };
    return: void;
  };
  [InvokeCommand.StarClipboard]: {
    args: { id: number; star: boolean };
    return: boolean;
  };
  [InvokeCommand.CopyClipboard]: {
    args: { id: number; type?: ClipboardType | null };
    return: boolean;
  };
  [InvokeCommand.ClearClipboards]: {
    args: { type?: ClipboardType | null };
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
  [InvokeCommand.ChangeClipboardDbLocation]: {
    args: undefined;
    return: void;
  };
  [InvokeCommand.ResetClipboardDbLocation]: {
    args: undefined;
    return: void;
  };

  // Window commands
  [InvokeCommand.OpenNewWindow]: {
    args: { windowName: WebWindow; title: string };
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
  [InvokeCommand.OpenFolder]: {
    args: { location: FolderLocation };
    return: void;
  };

  // Sync commands
  [InvokeCommand.SyncAuthenticateToggle]: {
    args: undefined;
    return: boolean;
  };
  [InvokeCommand.SyncLimitChange]: {
    args: { syncLimit: number };
    return: Settings;
  };

  // Cipher commands
  [InvokeCommand.EnableEncryption]: {
    args: { password: string; confirmPassword: string };
    return: void;
  };
  [InvokeCommand.DisableEncryption]: {
    args: { password: string };
    return: void;
  };
  [InvokeCommand.PasswordUnlock]: {
    args: { password: string, action: PasswordAction };
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
  [InvokeCommand.GetConfigPath]: {
    args: undefined;
    return: string;
  };
}
