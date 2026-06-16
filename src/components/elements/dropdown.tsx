import { VsArrowSwap } from "solid-icons/vs";
import { Component, createSignal } from "solid-js";
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
  const [ref, setRef] = createSignal<HTMLSelectElement>();

  return (
    <div
      onClick={() => ref()?.dispatchEvent(new MouseEvent("mousedown"))}
      class={`${props.className ? props.className : ""} group flex items-center justify-between rounded-md border border-border bg-popover p-1 px-1.5 text-sm text-foreground focus:ring-0 focus:outline-hidden`}
    >
      <select
        ref={setRef}
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
      <VsArrowSwap class="rotate-90 group-hover:text-primary" />
    </div>
  );
};
