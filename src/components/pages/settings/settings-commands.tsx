import {
  TbOutlinePlayerPlay,
  TbOutlinePlus,
  TbOutlineTrash,
} from "solid-icons/tb";
import { VsTerminal } from "solid-icons/vs";
import { Accessor, Component, createSignal, For, Index, Show } from "solid-js";
import { DictionaryKey } from "../../../lib/i18n";
import { invokeCommand } from "../../../lib/tauri";
import { cn } from "../../../lib/utils";
import { AppStore } from "../../../store/app-store";
import { HotkeyStore } from "../../../store/hotkey-store";
import { SettingsStore } from "../../../store/settings-store";
import { CommandScript, CustomCommand } from "../../../types";
import { InvokeCommand } from "../../../types/tauri-invoke";
import {
  COMMAND_OS,
  COMMAND_OUTPUTS,
  CommandOs,
  GLOBAL_SHORTCUT_KEYS,
} from "../../../utils/constants";
import { Button } from "../../elements/button";
import { CheckBox } from "../../elements/checkbox";
import { Dropdown } from "../../elements/dropdown";
import { Input } from "../../elements/input";
import { TextBlock } from "../../elements/text-block";
import { Toggle } from "../../elements/toggle";
import { useLanguage } from "../../provider/language-provider";

const INTERPRETERS = [
  "sh",
  "bash",
  "zsh",
  "fish",
  "python3",
  "node",
  "powershell",
  "pwsh",
  "cmd",
];

const OS_LABELS: Record<CommandOs, DictionaryKey> = {
  linux: "SETTINGS.COMMANDS.LINUX",
  windows: "SETTINGS.COMMANDS.WINDOWS",
  macos: "SETTINGS.COMMANDS.MACOS",
};

const OUTPUT_LABELS: Record<CustomCommand["output"], DictionaryKey> = {
  ignore: "SETTINGS.COMMANDS.OUTPUT_IGNORE",
  copy: "SETTINGS.COMMANDS.OUTPUT_COPY",
  paste: "SETTINGS.COMMANDS.OUTPUT_PASTE",
};

const SUPER_LABELS: Record<CommandOs, DictionaryKey> = {
  linux: "MAIN.KEYS.SUPER_LINUX",
  windows: "MAIN.KEYS.SUPER_WIN",
  macos: "MAIN.KEYS.SUPER_MAC",
};

const emptyScript = (): CommandScript => ({ interpreter: "", script: "" });

const currentOs = (): CommandOs => {
  const os = AppStore.os();
  return COMMAND_OS.find((name) => name === os) ?? "linux";
};

const saveCommands = (commands: CustomCommand[]) =>
  invokeCommand(InvokeCommand.ChangeSettingsCommands, { commands });

const commands = () => SettingsStore.settings()?.commands || [];

const sameHotkey = (
  a: Pick<CustomCommand, "ctrl" | "alt" | "shift" | "super_key" | "key">,
  b: Pick<CustomCommand, "ctrl" | "alt" | "shift" | "super_key" | "key">,
) =>
  a.ctrl === b.ctrl &&
  a.alt === b.alt &&
  a.shift === b.shift &&
  a.super_key === b.super_key &&
  a.key.toUpperCase() === b.key.toUpperCase();

const hotkeyInUse = (command: CustomCommand) =>
  command.key !== "none" &&
  (HotkeyStore.hotkeys().some(
    (hotkey) => hotkey.status && sameHotkey(hotkey, command),
  ) ||
    commands().some(
      (other) =>
        other.id !== command.id && other.enabled && sameHotkey(other, command),
    ));

export const SettingsCommands: Component = () => {
  const { t } = useLanguage();

  const addCommand = () =>
    saveCommands([
      ...commands(),
      {
        id: crypto.randomUUID(),
        name: "",
        enabled: true,
        ctrl: true,
        alt: true,
        shift: false,
        super_key: false,
        key: "none",
        output: "ignore",
        scripts: {
          linux: emptyScript(),
          windows: emptyScript(),
          macos: emptyScript(),
        },
      },
    ]);

  return (
    <TextBlock
      Icon={VsTerminal}
      title={t("SETTINGS.COMMANDS.TITLE")}
      header={
        <Button
          label="SETTINGS.COMMANDS.ADD"
          Icon={TbOutlinePlus}
          class="px-2"
          onClick={addCommand}
        />
      }
    >
      <p class="text-muted-foreground px-5 pb-3 text-sm">
        {t("SETTINGS.COMMANDS.INFO")}
      </p>
      <datalist id="command-interpreters">
        <For each={INTERPRETERS}>{(name) => <option value={name} />}</For>
      </datalist>
      <div class="flex flex-col gap-4 px-5 pb-5">
        <Index each={commands()}>
          {(command, index) => (
            <CommandEditor command={command} index={index} />
          )}
        </Index>
      </div>
    </TextBlock>
  );
};

interface CommandEditorProps {
  command: Accessor<CustomCommand>;
  index: number;
}

const CommandEditor: Component<CommandEditorProps> = (props) => {
  const { t } = useLanguage();
  const [os, setOs] = createSignal<CommandOs>(currentOs());
  const [result, setResult] = createSignal<{ ok: boolean; text: string }>();
  const [running, setRunning] = createSignal(false);

  let pendingSave: Promise<CustomCommand[]> | undefined;

  const update = (patch: Partial<CustomCommand>) => {
    pendingSave = saveCommands(
      commands().map((command, i) =>
        i === props.index ? { ...command, ...patch } : command,
      ),
    );
    return pendingSave;
  };

  const updateScript = (patch: Partial<CommandScript>) =>
    update({
      scripts: {
        ...props.command().scripts,
        [os()]: { ...props.command().scripts[os()], ...patch },
      },
    });

  const remove = () =>
    saveCommands(commands().filter((_, i) => i !== props.index));

  // Blurring the script box saves it, so wait for that before running.
  const test = async () => {
    setRunning(true);
    setResult(undefined);
    const saved = await pendingSave;
    const command = saved?.[props.index] ?? props.command();
    try {
      const output = await invokeCommand(InvokeCommand.RunCustomCommand, {
        command,
      });
      setResult({
        ok: true,
        text: output.trim() || t("SETTINGS.COMMANDS.NO_OUTPUT"),
      });
    } catch (error) {
      setResult({ ok: false, text: String(error) });
    } finally {
      setRunning(false);
    }
  };

  const script = () => props.command().scripts[os()];

  return (
    <div class="border-border flex flex-col gap-2.5 rounded-md border p-3">
      <div class="flex items-center gap-2.5">
        <Input
          class="flex-1"
          placeholder={t("SETTINGS.COMMANDS.NAME")}
          value={props.command().name}
          onChange={(e) => update({ name: e.currentTarget.value })}
        />
        <Toggle
          checked={props.command().enabled}
          onChange={(enabled) => update({ enabled })}
        />
        <Button
          label="SETTINGS.COMMANDS.REMOVE"
          Icon={TbOutlineTrash}
          class="px-2"
          onClick={remove}
        />
      </div>

      <div class="flex flex-wrap items-center gap-2.5 text-sm">
        <CheckBox
          label={t("MAIN.KEYS.CTRL")}
          checked={props.command().ctrl}
          onChange={(ctrl) => update({ ctrl })}
        />
        <CheckBox
          label={t("MAIN.KEYS.ALT")}
          checked={props.command().alt}
          onChange={(alt) => update({ alt })}
        />
        <CheckBox
          label={t("MAIN.KEYS.SHIFT")}
          checked={props.command().shift}
          onChange={(shift) => update({ shift })}
        />
        <CheckBox
          label={t(SUPER_LABELS[currentOs()])}
          checked={props.command().super_key}
          onChange={(super_key) => update({ super_key })}
        />
        <Dropdown
          items={GLOBAL_SHORTCUT_KEYS.map((key) => ({
            value: key,
            label: key,
          }))}
          value={props.command().key}
          onChange={(key) => update({ key })}
        />
        <Dropdown
          className="ml-auto"
          items={COMMAND_OUTPUTS.map((output) => ({
            value: output,
            label: OUTPUT_LABELS[output],
          }))}
          value={props.command().output}
          onChange={(output) =>
            update({
              output:
                COMMAND_OUTPUTS.find((mode) => mode === output) ?? "ignore",
            })
          }
        />
      </div>
      <Show when={hotkeyInUse(props.command())}>
        <p class="text-sm text-red-500">
          {t("SETTINGS.COMMANDS.HOTKEY_IN_USE")}
        </p>
      </Show>

      <div class="flex gap-1">
        <For each={COMMAND_OS}>
          {(name) => (
            <button
              type="button"
              class={cn(
                "cursor-pointer rounded-sm px-2 py-0.5 text-xs",
                os() === name
                  ? "bg-primary text-primary-foreground"
                  : "bg-secondary text-muted-foreground",
              )}
              onClick={() => {
                setOs(name);
                setResult(undefined);
              }}
            >
              <span class="flex items-center gap-1">
                {t(OS_LABELS[name])}
                <Show when={props.command().scripts[name].script.trim()}>
                  <span class="h-1.5 w-1.5 rounded-full bg-current" />
                </Show>
              </span>
            </button>
          )}
        </For>
      </div>
      <Input
        placeholder={`${t("SETTINGS.COMMANDS.INTERPRETER")}: ${os() === "windows" ? "powershell" : "sh"}`}
        list="command-interpreters"
        value={script().interpreter}
        onChange={(e) => updateScript({ interpreter: e.currentTarget.value })}
      />
      <textarea
        class="border-border bg-popover text-foreground focus:border-primary min-h-24 w-full resize-y rounded-md border p-1.5 font-mono text-xs focus:outline-hidden"
        placeholder={t("SETTINGS.COMMANDS.SCRIPT")}
        spellcheck={false}
        value={script().script}
        onChange={(e) => updateScript({ script: e.currentTarget.value })}
      />

      <div class="flex items-center gap-2.5">
        <Show
          when={os() === currentOs()}
          fallback={
            <p class="text-muted-foreground text-xs">
              {t("SETTINGS.COMMANDS.TEST_OTHER_OS")}
            </p>
          }
        >
          <Button
            label="SETTINGS.COMMANDS.TEST"
            Icon={TbOutlinePlayerPlay}
            class="px-2"
            disabled={running() || !script().script.trim()}
            onClick={test}
          />
        </Show>
      </div>
      <Show when={result()}>
        {(result) => (
          <pre
            class={cn(
              "bg-secondary max-h-32 overflow-auto rounded-md p-2 text-xs whitespace-pre-wrap",
              result().ok ? "text-foreground" : "text-red-500",
            )}
          >
            {result().text}
          </pre>
        )}
      </Show>
    </div>
  );
};
