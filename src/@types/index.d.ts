export type Clipboards = {
  id: number;
  type: string;
  content: string | null;
  width: number | null;
  height: number | null;
  size: string | null;
  blob: Buffer | null;
  star: boolean;
  createdDate: Date;
};

export type Hotkey = {
  id: number;
  event: string;
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  key: string;
  status: boolean;
  name: string;
  icon: string;
};

export type Settings = {
  id: number;
  startup: boolean;
  notification: boolean;
  synchronize: boolean;
  synchronize_time: number;
  dar_kmode: boolean;
};
