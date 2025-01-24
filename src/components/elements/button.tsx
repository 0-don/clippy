import { IconTypes } from "solid-icons";
import { Component, JSX } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";
import { useLanguage } from "../provider/language-provider";

interface ButtonProps {
  label: DictionaryKey;
  className?: string;
  iconClassName?: string;
  type?: HTMLButtonElement["type"];
  Icon?: IconTypes;
  onClick?: JSX.EventHandlerUnion<HTMLButtonElement, MouseEvent>;
  children?: JSX.Element;
  disabled?: boolean;
}

export const Button: Component<ButtonProps> = (props) => {
  const { t } = useLanguage();

  return (
    <button
      type={props.type || "button"}
      onClick={props.onClick}
      class={`flex ${!props.disabled ? "cursor-pointer" : "cursor-not-allowed"} items-center justify-center rounded-sm bg-zinc-600 px-1 py-1 text-xs font-bold text-white hover:bg-zinc-700 disabled:bg-neutral-500 ${props.className}`}
      disabled={props.disabled}
    >
      {props.Icon && <props.Icon class={`mr-1 text-lg ${props.iconClassName}`} />}
      {props.children}
      <span>{t(props.label)}</span>
    </button>
  );
};
