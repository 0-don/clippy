import { BsFileEarmarkLock2Fill } from "solid-icons/bs";
import { Component, createSignal } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { Button } from "../../elements/button";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

interface SettingsEncryptionProps {}

export const SettingsEncryption: Component<SettingsEncryptionProps> = ({}) => {
  const { t } = useLanguage();

  const [password, setPassword] = createSignal("");
  const [confirmPassword, setConfirmPassword] = createSignal("");

  return (
    <TextBlock
      Icon={BsFileEarmarkLock2Fill}
      header={<Toggle disabled={true} checked={SettingsStore.settings()?.encryption} />}
      title={t("SETTINGS.ENCRYPT.ENCRYPT_DECRYPT")}
    >
      <div class="flex flex-col gap-1 px-5 pb-5">
        <p class="text-sm text-zinc-700 dark:text-zinc-400">{t("SETTINGS.ENCRYPT.INFO")}</p>

        <div>
          <label>{t("SETTINGS.ENCRYPT.PASSWORD")}</label>
          <Input value={password()} debounce={0} onInput={(e) => setPassword(e.target.value)} />
        </div>
        <div>
          <label>{t("SETTINGS.ENCRYPT.CONFIRM_PASSWORD")}</label>
          <Input value={confirmPassword()} debounce={0} onInput={(e) => setConfirmPassword(e.target.value)} />
        </div>

        <Button label={"SETTINGS.ENCRYPT.ENCRYPT"} />
      </div>
    </TextBlock>
  );
};
