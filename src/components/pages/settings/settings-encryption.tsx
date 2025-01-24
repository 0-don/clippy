import { AiFillLock, AiFillUnlock } from "solid-icons/ai";
import { BsFileEarmarkLock2Fill } from "solid-icons/bs";
import { Component, createSignal, Show } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
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

  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");

  return (
    <form class="flex flex-col gap-2" onSubmit={(e) => e.preventDefault()}>
      <div>
        <label>{t("SETTINGS.ENCRYPT.PASSWORD")}</label>
        <Input value={password()} debounce={0} onInput={(e) => setPassword(e.target.value)} />
      </div>
      <div>
        <label>{t("SETTINGS.ENCRYPT.CONFIRM_PASSWORD")}</label>
        <Input value={confirmPassword()} debounce={0} onInput={(e) => setConfirmPassword(e.target.value)} />
      </div>

      <Button className="mt-1" disabled Icon={AiFillLock} label={"SETTINGS.ENCRYPT.ENCRYPT"} />
    </form>
  );
};

const Decrypt: Component = ({}) => {
  const { t } = useLanguage();

  const [password, setPassword] = createSignal("");

  return (
    <form class="flex flex-col gap-1">
      <div>
        <label>{t("SETTINGS.ENCRYPT.PASSWORD")}</label>
        <Input value={password()} debounce={0} onInput={(e) => setPassword(e.target.value)} />
      </div>

      <Button Icon={AiFillUnlock} label={"SETTINGS.ENCRYPT.DECRYPT"} />
    </form>
  );
};
