import { type Flatten, flatten } from "@solid-primitives/i18n";
import { Language } from "../../utils/constants";
import type * as en from "./en.json";

export type RawDictionary = typeof en;
export type Locale = `${Language}`;
export type Dictionary = Flatten<RawDictionary>;
export type DictionaryKey = {
  [K in keyof Dictionary]: Dictionary[K] extends string ? K : never;
}[keyof Dictionary];

export async function fetchDictionary(locale: Locale): Promise<Dictionary> {
  const dict: RawDictionary = await import(`./${locale}.json?import`);
  return flatten(dict);
}

export const msg = (key: DictionaryKey) => key;
