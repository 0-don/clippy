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
      class={`${props.className ? props.className : ""} group flex items-center justify-between rounded-md border border-gray-300 p-1 px-1.5 text-sm focus:outline-none focus:ring-0 dark:border-dark-light dark:bg-dark-light dark:text-white`}
    >
      <select
        ref={setRef}
        value={props.value}
        onChange={(e) => props.onChange(e.target.value)}
        class="appearance-none bg-transparent text-sm focus:outline-none focus:ring-0"
      >
        {props.items.map((item) => (
          <option value={item.value} selected={item.value === props.value} class="!text-red-500 dark:!text-red-600">
            {t(item.label as DictionaryKey) || item.label}
          </option>
        ))}
      </select>
      <VsArrowSwap class="rotate-90 group-hover:text-indigo-200" />
    </div>
  );
};
