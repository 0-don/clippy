import { AiFillUnlock } from "solid-icons/ai";
import { ImSpinner } from "solid-icons/im";
import { Component, createSignal } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";
import { invokeCommand } from "../../lib/tauri";
import { AppStore } from "../../store/app-store";
import { TauriError } from "../../types";
import { InvokeCommand } from "../../types/tauri-invoke";
import {
  MAX_PASSWORD_LENGTH,
  MIN_PASSWORD_LENGTH,
} from "../../utils/constants";
import { useLanguage } from "../provider/language-provider";
import { Button } from "./button";
import { Input } from "./input";

export const PasswordLock: Component = () => {
  const { t } = useLanguage();
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");
  const [password, setPassword] = createSignal("");

  const onSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      await invokeCommand(InvokeCommand.PasswordUnlock, {
        password: password(),
        action: AppStore.passwordLock()!,
      });
      AppStore.setPasswordLock(undefined);
    } catch (error) {
      const { Error } = error as TauriError;
      setError(Error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/95">
      <div class="dark:bg-dark w-80 rounded-lg bg-white p-5 shadow-lg">
        <form class="flex flex-col gap-2" onSubmit={onSubmit}>
          <Input
            type="password"
            required
            placeholder={t("SETTINGS.ENCRYPT.PASSWORD")}
            minLength={MIN_PASSWORD_LENGTH}
            maxLength={MAX_PASSWORD_LENGTH}
            value={password()}
            onInput={(e) => setPassword(e.target.value)}
            autofocus
          />

          {error() && (
            <p class="text-sm text-red-500">
              {t(error() as DictionaryKey) || error()}
            </p>
          )}

          <Button
            type="submit"
            label="MAIN.UNLOCK_CLIPPY"
            Icon={loading() ? ImSpinner : AiFillUnlock}
            iconClassName={loading() ? "animate-spin" : ""}
            disabled={loading()}
          />
        </form>
      </div>
    </div>
  );
};
