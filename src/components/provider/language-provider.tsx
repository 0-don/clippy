import {
  Translator,
  resolveTemplate,
  translator,
} from "@solid-primitives/i18n";
import {
  JSX,
  Resource,
  ResourceActions,
  createContext,
  createEffect,
  createResource,
  useContext,
} from "solid-js";
import { Dictionary, Locale, fetchDictionary } from "../../lib/i18n";
import { AppStore } from "../../store/app-store";
import { LANGUAGES, LANGUAGE_KEY, Language } from "../../utils/constants";

export interface LanguageContextType {
  locale: Resource<Locale>;
  setLocale: ResourceActions<Language | undefined, unknown>;
  t: Translator<Dictionary, string>;
}

export const LanguageContext = createContext<LanguageContextType>();

export function useLanguage() {
  return useContext(LanguageContext!)!;
}

export default function LanguageProvider(props: { children: JSX.Element }) {
  const [dict] = createResource(AppStore.locale, fetchDictionary);

  createEffect(() =>
    localStorage.setItem(
      LANGUAGE_KEY,
      AppStore.locale() || Object.values(LANGUAGES)[0],
    ),
  );

  const contextValue: LanguageContextType = {
    locale: AppStore.locale,
    setLocale: AppStore.setLocale,
    t: translator(dict, resolveTemplate) as Translator<Dictionary, string>,
  };

  return (
    <LanguageContext.Provider value={contextValue}>
      {props.children}
    </LanguageContext.Provider>
  );
}
