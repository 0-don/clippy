import { Combobox, createFilter } from "@kobalte/core";
import { FiCheck } from "solid-icons/fi";
import { VsArrowSwap } from "solid-icons/vs";
import { Component, createSignal } from "solid-js";
import { GlobalShortcutKeysType } from "../../utils/constants";

interface DropdownProps {
  items: string[];
  value: string;
  onChange: (char: GlobalShortcutKeysType | string) => void;
}

export const Dropdown: Component<DropdownProps> = ({
  items,
  onChange,
  value,
}) => {
  const filter = createFilter({ sensitivity: "base" });
  const [options, setOptions] = createSignal(items);

  const onOpenChange = (
    isOpen: boolean,
    triggerMode?: Combobox.ComboboxTriggerMode
  ) => isOpen && triggerMode === "manual" && setOptions(items);

  const onInputChange = (value: string) =>
    setOptions(items.filter((option) => filter.contains(option, value)));

  return (
    <Combobox.Root
      options={options()}
      onInputChange={onInputChange}
      onOpenChange={onOpenChange}
      onChange={onChange}
      defaultValue={value}
      itemComponent={(props) => (
        <Combobox.Item
          item={props.item}
          class={`${
            props.item.rawValue === value
              ? "bg-indigo-600 text-white"
              : "text-white"
          } flex cursor-pointer items-center justify-between px-2 hover:bg-indigo-600`}
        >
          <Combobox.ItemLabel>{props.item.rawValue}</Combobox.ItemLabel>
          <Combobox.ItemIndicator>
            <FiCheck />
          </Combobox.ItemIndicator>
        </Combobox.Item>
      )}
    >
      <Combobox.Control class="rounded-md border border-gray-300 p-1 focus:outline-none dark:border-dark-light dark:bg-dark-light dark:text-white dark:focus:bg-dark-dark">
        <Combobox.Trigger class="group flex items-center">
          <Combobox.Input class="w-8 cursor-pointer bg-transparent text-center outline-none" />
          <Combobox.Icon>
            <VsArrowSwap class="rotate-90 group-hover:text-indigo-200" />
          </Combobox.Icon>
        </Combobox.Trigger>
      </Combobox.Control>
      <Combobox.Portal>
        <Combobox.Content>
          <Combobox.Listbox class="absolute -top-1.5 z-10 max-h-24 w-full overflow-auto rounded-md bg-dark-light text-sm shadow-lg" />
        </Combobox.Content>
      </Combobox.Portal>
    </Combobox.Root>
  );
};
