import { BsFileEarmarkLock2Fill } from "solid-icons/bs";
import { Component, createSignal, Show } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { useLanguage } from "../../provider/language-provider";

interface SettingsEncryptionProps {}

export const SettingsEncryption: Component<SettingsEncryptionProps> = ({}) => {
  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");
  const { t } = useLanguage();

  return (
    <Show when={SettingsStore.settings()}>
      <TextBlock Icon={BsFileEarmarkLock2Fill} title={t("SETTINGS.ENCRYPT.ENCRYPT_DECRYPT")}>
        <div class="px-5 pb-5">
          <p class="text-sm text-zinc-700 dark:text-zinc-400">{t("SETTINGS.ENCRYPT.INFO")}</p>

          <div>
            <label>Password</label>
            <Input value={password()} debounce={0} onInput={(e) => setPassword(e.target.value)} />
          </div>
          <div>
            <label>Confirm Password</label>
            <Input value={confirmPassword()} debounce={0} onInput={(e) => setConfirmPassword(e.target.value)} />
          </div>
        </div>
      </TextBlock>
    </Show>
  );
};
