import { VsArrowSwap } from "solid-icons/vs";
import { Component, createSignal } from "solid-js";
import { GlobalShortcutKeys } from "../../utils/constants";

interface DropdownProps {
  items: string[];
  value: string;
  onChange: (value: GlobalShortcutKeys | (string & {})) => void;
}

export const Dropdown: Component<DropdownProps> = (props) => {
  const [ref, setRef] = createSignal<HTMLSelectElement>();

  return (
    <div
      onClick={() => ref()?.dispatchEvent(new MouseEvent("mousedown"))}
      class="group flex items-center rounded-md border border-gray-300 p-1 text-sm focus:outline-none focus:ring-0 dark:border-dark-light dark:bg-dark-light dark:text-white"
    >
      <select
        ref={setRef}
        value={props.value}
        onChange={(e) => props.onChange(e.target.value)}
        class="appearance-none bg-transparent text-sm focus:outline-none focus:ring-0"
      >
        {props.items.map((item) => (
          <option value={item} selected={item === props.value} class="!text-red-500 dark:!text-red-600">
            {item}
          </option>
        ))}
      </select>
      <VsArrowSwap class="rotate-90 group-hover:text-indigo-200" />
    </div>
  );
};
