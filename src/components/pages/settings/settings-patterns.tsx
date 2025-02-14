import { AiFillEye, AiFillEyeInvisible } from "solid-icons/ai";
import { TbGridPattern, TbTrash } from "solid-icons/tb";
import { Component, createSignal, For, Show } from "solid-js";
import { invokeCommand } from "../../../lib/tauri";
import { SettingsStore } from "../../../store/settings-store";
import { InvokeCommand } from "../../../types/tauri-invoke";
import { MAX_DESCRIPTION_LENGTH, MIN_PATTERN_LENGTH } from "../../../utils/constants";
import { Button } from "../../elements/button";
import { CheckBox } from "../../elements/checkbox";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

interface SettingsPatternsProps {}

export const SettingsPatterns: Component<SettingsPatternsProps> = ({}) => {
  const { t } = useLanguage();
  const [passwordType, setPasswordType] = createSignal(!import.meta.env.DEV);
  const [matchExpression, setMatchExpression] = createSignal("");
  const [substitution, setSubstitution] = createSignal("");
  const [enabled, setEnabled] = createSignal(true);

  const onSubmit = async (e: Event) => {
    e.preventDefault();
    await invokeCommand(InvokeCommand.ChangeSettingsTextMatchers, {
      textMatchers: [
        ...(SettingsStore.settings()?.text_matchers || []),
        {
          match_expression: matchExpression(),
          substitution: substitution(),
          enabled: enabled(),
        },
      ],
    });
    setMatchExpression("");
    setSubstitution("");
    setEnabled(true);
  };

  return (
    <TextBlock Icon={TbGridPattern} title={t("SETTINGS.PATTERNS.CHANGE_YOUR_TEXT_MATCHES")}>
      <div class="flex items-center justify-between gap-2 px-5 pb-5">
        <p class="text-sm text-zinc-700 dark:text-zinc-400">{t("SETTINGS.PATTERNS.INFO")}</p>
      </div>
      {/* Table Headers */}
      <div class="mb-2 grid grid-cols-[1fr_1fr_auto_auto] gap-2.5 px-5">
        <div class="text-sm font-bold text-zinc-700 dark:text-zinc-400">{t("SETTINGS.PATTERNS.MATCH_EXPRESSION")}</div>
        <div class="text-sm font-bold text-zinc-700 dark:text-zinc-400">{t("SETTINGS.PATTERNS.SUBSTITUTION")}</div>
        <div class="text-sm font-bold text-zinc-700 dark:text-zinc-400">{t("SETTINGS.PATTERNS.ACTIONS")}</div>
        <div></div> {/* Empty header for action button */}
      </div>
      {/* Add new pattern form */}
      <form onSubmit={onSubmit} class="grid grid-cols-[1fr_1fr_auto_auto] gap-2.5 px-5 pb-5">
        <Input
          placeholder={t("SETTINGS.PATTERNS.MATCH_EXPRESSION")}
          value={matchExpression()}
          onInput={(e) => setMatchExpression(e.currentTarget.value)}
          minLength={MIN_PATTERN_LENGTH}
          maxlength={MAX_DESCRIPTION_LENGTH}
          required
        />
        <Input
          placeholder={t("SETTINGS.PATTERNS.SUBSTITUTION")}
          value={substitution()}
          onInput={(e) => setSubstitution(e.currentTarget.value)}
          minLength={0}
          maxlength={MAX_DESCRIPTION_LENGTH}
        />
        <CheckBox label={t("SETTINGS.PATTERNS.ENABLED")} checked={enabled()} onChange={setEnabled} />
        <Button label={"SETTINGS.PATTERNS.ADD"} type="submit" class="w-24" Icon={TbGridPattern} />
      </form>
      <div class="flex items-center gap-2 px-5">
        <Toggle checked={passwordType()} onChange={(val) => setPasswordType(val)} />
        <Show when={passwordType()} fallback={<AiFillEye />}>
          <AiFillEyeInvisible />
        </Show>
      </div>
      {/* Existing patterns list */}
      <div class="h-72 overflow-auto px-5">
        <For each={SettingsStore.settings()?.text_matchers || []}>
          {(pattern, index) => (
            <div class="flex flex-col gap-2.5">
              <div class="mt-2.5 grid grid-cols-[1fr_1fr_auto_auto] gap-2.5">
                <Input
                  placeholder={t("SETTINGS.PATTERNS.MATCH_EXPRESSION")}
                  type={passwordType() ? "password" : "text"}
                  minLength={MIN_PATTERN_LENGTH}
                  maxlength={MAX_DESCRIPTION_LENGTH}
                  value={pattern.match_expression}
                  debounce={1000}
                  onInput={(e) => {
                    invokeCommand(InvokeCommand.ChangeSettingsTextMatchers, {
                      textMatchers:
                        SettingsStore.settings()?.text_matchers.map((p, i) =>
                          i === index() ? { ...p, match_expression: e.currentTarget.value || "" } : p
                        ) || [],
                    });
                  }}
                  required
                />

                <Input
                  placeholder={t("SETTINGS.PATTERNS.SUBSTITUTION")}
                  type={passwordType() ? "password" : "text"}
                  value={pattern.substitution}
                  minLength={0}
                  maxlength={MAX_DESCRIPTION_LENGTH}
                  debounce={1000}
                  onInput={(e) => {
                    invokeCommand(InvokeCommand.ChangeSettingsTextMatchers, {
                      textMatchers:
                        SettingsStore.settings()?.text_matchers.map((p, i) =>
                          i === index() ? { ...p, substitution: e.currentTarget.value || "" } : p
                        ) || [],
                    });
                  }}
                />
                <CheckBox
                  label={t("SETTINGS.PATTERNS.ENABLED")}
                  checked={pattern.enabled}
                  onChange={(enabled) =>
                    invokeCommand(InvokeCommand.ChangeSettingsTextMatchers, {
                      textMatchers:
                        SettingsStore.settings()?.text_matchers.map((p, i) =>
                          i === index() ? { ...p, enabled: !!enabled } : p
                        ) || [],
                    })
                  }
                />
                <Button
                  label={"SETTINGS.PATTERNS.REMOVE"}
                  Icon={TbTrash}
                  class="w-24"
                  onClick={() =>
                    invokeCommand(InvokeCommand.ChangeSettingsTextMatchers, {
                      textMatchers: SettingsStore.settings()?.text_matchers.filter((_, i) => i !== index()) || [],
                    })
                  }
                />
              </div>
              <hr class="border-zinc-700" />
            </div>
          )}
        </For>
      </div>
    </TextBlock>
  );
};
