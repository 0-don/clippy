import { IconTypes } from "solid-icons";
import { ComponentProps } from "@solidjs/web";
import { Component, omit } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";
import { cn } from "../../lib/utils";
import { useLanguage } from "../provider/language-provider";

type ButtonProps = ComponentProps<"button"> & {
  label: DictionaryKey;
  iconClassName?: string;
  Icon?: IconTypes;
};

export const Button: Component<ButtonProps> = (props) => {
  const buttonProps = omit(
    props,
    "label",
    "iconClassName",
    "Icon",
    "children",
    "class",
  );

  const { t } = useLanguage();

  return (
    <button
      type={buttonProps.type || "button"}
      {...buttonProps}
      class={cn(
        "bg-primary text-primary-foreground hover:bg-primary/90 disabled:bg-muted flex items-center justify-center rounded-sm px-1 py-1 text-xs font-bold",
        !buttonProps.disabled ? "cursor-pointer" : "cursor-not-allowed",
        props.class,
      )}
    >
      {props.Icon && (
        <props.Icon class={cn("mr-1 text-lg", props.iconClassName)} />
      )}
      {props.children}
      <span>{t(props.label) || props.label}</span>
    </button>
  );
};
