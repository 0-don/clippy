import { FiSave, FiX } from "solid-icons/fi";
import { ImSpinner } from "solid-icons/im";
import { Component, createSignal, onSettled, Show } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";
import { invokeCommand } from "../../lib/tauri";
import { ClipboardWithRelations, TauriError } from "../../types";
import { InvokeCommand } from "../../types/tauri-invoke";
import { useLanguage } from "../provider/language-provider";
import { Button } from "./button";
import { Input } from "./input";

interface ClipboardEditorProps {
  entry: ClipboardWithRelations;
  onClose: () => void;
  onSaved: (name: string | null, data: string) => void;
}

export const ClipboardEditor: Component<ClipboardEditorProps> = (props) => {
  const { t } = useLanguage();
  const [saving, setSaving] = createSignal(false);
  const [error, setError] = createSignal("");
  const [name, setName] = createSignal(() => props.entry.clipboard.name || "");
  const [data, setData] = createSignal("");
  const [loading, setLoading] = createSignal(true);

  // List payloads are truncated to a preview length, so the full text has to be
  // fetched before it can be edited.
  onSettled(() => {
    void invokeCommand(InvokeCommand.GetClipboard, {
      id: props.entry.clipboard.id,
    })
      .then((full) => setData(full.text?.data || ""))
      .finally(() => setLoading(false));
  });

  const onSave = async () => {
    setError("");
    setSaving(true);

    const trimmed = name().trim();
    const nextName = trimmed.length > 0 ? trimmed : null;

    try {
      await invokeCommand(InvokeCommand.UpdateClipboardContent, {
        id: props.entry.clipboard.id,
        name: nextName,
        data: data(),
      });
      props.onSaved(nextName, data());
      props.onClose();
    } catch (error) {
      const { Error } = error as TauriError;
      setError(Error);
    } finally {
      setSaving(false);
    }
  };

  const onKeyDown = (e: KeyboardEvent) => {
    e.stopPropagation();

    if (e.key === "Escape") {
      props.onClose();
    } else if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
      onSave();
    }
  };

  return (
    <div
      class="bg-background fixed inset-0 z-9999 flex flex-col p-3"
      onKeyDown={onKeyDown}
    >
      <div class="mb-2 flex items-center justify-between">
        <h6 class="text-sm font-bold">{t("CLIPBOARD.EDIT_ENTRY")}</h6>
        <FiX
          onClick={props.onClose}
          title={t("CLIPBOARD.CANCEL")}
          class="text-foreground cursor-pointer hover:text-red-600 dark:hover:text-red-500"
        />
      </div>

      <Input
        type="text"
        placeholder={t("CLIPBOARD.ENTER_NAME")}
        value={name()}
        onInput={(e) => setName(e.target.value)}
      />

      <Show
        when={!loading()}
        fallback={
          <div class="flex flex-1 items-center justify-center">
            <ImSpinner class="text-foreground animate-spin" />
          </div>
        }
      >
        <textarea
          ref={(el) => setTimeout(() => el.focus(), 0)}
          value={data()}
          onInput={(e) => setData(e.currentTarget.value)}
          placeholder={t("CLIPBOARD.ENTER_CONTENT")}
          class="border-border bg-background text-foreground focus:border-primary mt-2 min-h-0 flex-1 resize-none rounded border px-2 py-1 font-mono text-xs outline-none"
        />
      </Show>

      <Show when={error()}>
        <p class="mt-1 text-xs text-red-500">
          {t(error() as DictionaryKey) || error()}
        </p>
      </Show>

      <div class="mt-2 flex justify-end gap-2">
        <Button
          label="CLIPBOARD.CANCEL"
          Icon={FiX}
          onClick={props.onClose}
          class="bg-secondary text-secondary-foreground hover:bg-secondary/90"
        />
        <Button
          label="CLIPBOARD.SAVE"
          Icon={saving() ? ImSpinner : FiSave}
          iconClassName={saving() ? "animate-spin" : ""}
          onClick={onSave}
          disabled={saving() || loading()}
        />
      </div>
    </div>
  );
};
