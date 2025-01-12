import { ClipboardResponse, ClipboardWhere, DatabaseInfo, Hotkey, Settings } from ".";
import { ClipboardType, FolderLocation, WebWindow } from "./enums";

export enum InvokeCommand {
  //Auth
  AuthGoogleDrive = "auth_google_drive",

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

  // Window commands
  OpenNewWindow = "open_new_window",
  OpenBrowserUrl = "open_browser_url",
  ExitApp = "exit_app",
  OpenFolder = "open_folder",

  // Sync commands
  SyncIsAuthenticated = "sync_is_authenticated",
  SyncAuthenticateToggle = "sync_authenticate_toggle",

  // App info commands
  GetAppVersion = "get_app_version",
  GetDbInfo = "get_db_info",
  GetDbPath = "get_db_path",
  GetConfigPath = "get_config_path",
}

export interface TauriInvokeCommands {
  //Auth
  [InvokeCommand.AuthGoogleDrive]: {
    args: undefined;
    return: string;
  };

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
