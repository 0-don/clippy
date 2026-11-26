import {
  Translator,
  resolveTemplate,
  translator,
} from "@solid-primitives/i18n";
import { JSX } from "@solidjs/web";
import {
  createContext,
  createEffect,
  createSignal,
  useContext,
} from "solid-js";
import { Dictionary, fetchDictionary } from "../../lib/i18n";
import { AppStore } from "../../store/app-store";
import { LANGUAGE_KEY } from "../../utils/constants";

export interface LanguageContextType {
  t: Translator<Dictionary, string>;
}

export const LanguageContext = createContext<LanguageContextType>();

export function useLanguage() {
  return useContext(LanguageContext);
}

export default function LanguageProvider(props: { children: JSX.Element }) {
  const [dict, setDict] = createSignal<Dictionary>();

  createEffect(AppStore.locale, (locale) => {
    localStorage.setItem(LANGUAGE_KEY, locale);
    void fetchDictionary(locale).then(setDict);
  });

  const contextValue: LanguageContextType = {
    t: translator(dict, resolveTemplate) as Translator<Dictionary, string>,
  };

  return (
    <LanguageContext value={contextValue}>{props.children}</LanguageContext>
  );
}
