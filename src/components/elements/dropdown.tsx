import { VsArrowSwap } from "solid-icons/vs";
import { Component } from "solid-js";
import { DictionaryKey } from "../../lib/i18n";

import { useLanguage } from "../provider/language-provider";

interface DropdownProps {
  className?: string;
  items: { value: string; label: string }[];
  value: string;
  onChange: (value: string) => void;
}

export const Dropdown: Component<DropdownProps> = (props) => {
  const { t } = useLanguage();
  let select: HTMLSelectElement | undefined;

  return (
    <div
      onClick={() => select?.dispatchEvent(new MouseEvent("mousedown"))}
      class={`${props.className ? props.className : ""} group border-border bg-popover text-foreground flex items-center justify-between rounded-md border p-1 px-1.5 text-sm focus:ring-0 focus:outline-hidden`}
    >
      <select
        ref={select}
        value={props.value}
        onChange={(e) => props.onChange(e.target.value)}
        class="cursor-pointer appearance-none bg-transparent text-sm focus:ring-0 focus:outline-hidden"
      >
        {props.items.map((item) => (
          <option
            value={item.value}
            selected={item.value === props.value}
            class="bg-popover text-popover-foreground"
          >
            {t(item.label as DictionaryKey) || item.label}
          </option>
        ))}
      </select>
      <VsArrowSwap class="group-hover:text-primary rotate-90" />
    </div>
  );
};
