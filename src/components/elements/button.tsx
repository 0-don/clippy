import { IconTypes } from "solid-icons";
import { Component, ComponentProps, splitProps } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";
import { cn } from "../../lib/utils";
import { useLanguage } from "../provider/language-provider";

type ButtonProps = ComponentProps<"button"> & {
  label: DictionaryKey;
  iconClassName?: string;
  Icon?: IconTypes;
};

export const Button: Component<ButtonProps> = (props) => {
  const [local, buttonProps] = splitProps(props, ["label", "iconClassName", "Icon", "children", "class"]);

  const { t } = useLanguage();

  return (
    <button
      type={buttonProps.type || "button"}
      {...buttonProps}
      class={cn(
        "flex items-center justify-center rounded-sm bg-zinc-600 px-1 py-1 text-xs font-bold text-white hover:bg-zinc-700 disabled:bg-neutral-500",
        !buttonProps.disabled ? "cursor-pointer" : "cursor-not-allowed",
        local.class
      )}
    >
      {local.Icon && <local.Icon class={cn("mr-1 text-lg", local.iconClassName)} />}
      {local.children}
      <span>{t(local.label) || local.label}</span>
    </button>
  );
};
