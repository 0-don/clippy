import { AiFillLock, AiFillUnlock } from "solid-icons/ai";
import { BsFileEarmarkLock2Fill } from "solid-icons/bs";
import { ImSpinner } from "solid-icons/im";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { DictionaryKey } from "../../../lib/i18n";
import { invokeCommand, listenEvent } from "../../../lib/tauri";
import { cn } from "../../../lib/utils";
import { SettingsStore } from "../../../store/settings-store";
import { Progress, TauriError } from "../../../types";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { ListenEvent } from "../../../types/tauri-listen";
import { MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH } from "../../../utils/constants";
import { Button } from "../../elements/button";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

interface SettingsEncryptionProps {}

export const SettingsEncryption: Component<SettingsEncryptionProps> = ({}) => {
  const { t } = useLanguage();

  return (
    <TextBlock
      Icon={BsFileEarmarkLock2Fill}
      header={<Toggle disabled checked={SettingsStore.settings()?.encryption} />}
      title={t("SETTINGS.ENCRYPT.ENCRYPT_DECRYPT")}
    >
      <div class="px-5 pb-5">
        <p class="text-sm text-zinc-700 dark:text-zinc-400">{t("SETTINGS.ENCRYPT.INFO")}</p>
        <Show when={SettingsStore.settings()?.encryption} fallback={<Encrypt />}>
          <Decrypt />
        </Show>
      </div>
    </TextBlock>
  );
};

const Encrypt: Component = ({}) => {
  const { t } = useLanguage();

  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");
  const [encryptionProgress, setEncryptionProgress] = createSignal<Progress>();
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");

  const onSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    if (password() !== confirmPassword()) {
      setError(t("MAIN.ERROR.PASSWORD_NOT_MATCH"));
      setLoading(false);
      return;
    }

    try {
      await invokeCommand(InvokeCommand.EnableEncryption, { password: password(), confirmPassword: confirmPassword() });
      await SettingsStore.init();
    } catch (error) {
      const { Error } = error as TauriError;
      setError(Error);
    } finally {
      setLoading(false);
    }
  };

  listenEvent(ListenEvent.Progress, setEncryptionProgress);

  createEffect(() => {
    console.log(encryptionProgress());
  });
  return (
    <form class="flex flex-col gap-2" onSubmit={onSubmit}>
      <div>
        <label>{t("SETTINGS.ENCRYPT.PASSWORD")}</label>
        <Input required minLength={1} maxLength={128} value={password()} onInput={(e) => setPassword(e.target.value)} />
      </div>
      <div>
        <label>{t("SETTINGS.ENCRYPT.CONFIRM_PASSWORD")}</label>
        <Input
          required
          minLength={MIN_PASSWORD_LENGTH}
          maxLength={MAX_PASSWORD_LENGTH}
          value={confirmPassword()}
          onInput={(e) => setConfirmPassword(e.target.value)}
        />
      </div>

      <Show when={error()}>
        <p class="text-red-500">{t(error() as DictionaryKey) || error()}</p>
      </Show>

      <Button
        type="submit"
        className="mt-1"
        Icon={loading() ? ImSpinner : AiFillLock}
        iconClassName={cn(loading() && "animate-spin")}
        label={
          !loading()
            ? "SETTINGS.ENCRYPT.ENCRYPT"
            : encryptionProgress()
              ? (t(encryptionProgress()!.label, {
                  current: encryptionProgress()?.current || 0,
                  total: encryptionProgress()?.total || 0,
                }) as DictionaryKey)
              : "SETTINGS.ENCRYPT.ENCRYPT"
        }
      />
    </form>
  );
};

const Decrypt: Component = ({}) => {
  const { t } = useLanguage();

  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal("");
  const [encryptionProgress, setEncryptionProgress] = createSignal<Progress>();
  const [password, setPassword] = createSignal("");

  const onSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      await invokeCommand(InvokeCommand.DisableEncryption, { password: password() });
      await SettingsStore.init();
    } catch (error) {
      const { Error } = error as TauriError;
      setError(Error);
    } finally {
      setLoading(false);
    }
  };

  listenEvent(ListenEvent.Progress, setEncryptionProgress);

  createEffect(() => {
    console.log(encryptionProgress());
  });

  return (
    <form class="flex flex-col gap-1" onSubmit={onSubmit}>
      <div>
        <label>{t("SETTINGS.ENCRYPT.PASSWORD")}</label>
        <Input
          minLength={MIN_PASSWORD_LENGTH}
          maxLength={MAX_PASSWORD_LENGTH}
          value={password()}
          onInput={(e) => setPassword(e.target.value)}
        />
      </div>

      <Show when={error()}>
        <p class="text-red-500">{t(error() as DictionaryKey) || error()}</p>
      </Show>

      <Button
        type="submit"
        className="mt-1"
        Icon={loading() ? ImSpinner : AiFillUnlock}
        iconClassName={cn(loading() && "animate-spin")}
        label={
          !loading()
            ? "SETTINGS.ENCRYPT.DECRYPT"
            : encryptionProgress()
              ? (t(encryptionProgress()!.label, {
                  current: encryptionProgress()?.current || 0,
                  total: encryptionProgress()?.total || 0,
                }) as DictionaryKey)
              : "SETTINGS.ENCRYPT.DECRYPT"
        }
      />
    </form>
  );
};
