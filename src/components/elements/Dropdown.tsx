import { Combobox, createFilter } from "@kobalte/core";
import { FaSolidSort } from "solid-icons/fa";
import { FiCheck } from "solid-icons/fi";
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
      multiple={false}
      itemComponent={(props) => (
        <Combobox.Item
          item={props.item}
          class={`${
            props.item.rawValue === value
              ? "bg-indigo-600 text-white"
              : "text-white"
          } flex justify-between`}
        >
          <Combobox.ItemLabel>{props.item.rawValue}</Combobox.ItemLabel>
          <Combobox.ItemIndicator>
            <FiCheck />
          </Combobox.ItemIndicator>
        </Combobox.Item>
      )}
    >
      <Combobox.Control class="inline-flex w-[200px] justify-between rounded-md border border-solid border-zinc-200 bg-[white] text-base leading-none text-zinc-800">
        <Combobox.Input class="inline-flex min-h-[40px] min-w-0 appearance-none rounded-bl-md rounded-tl-md pl-4 text-base" />
        <Combobox.Trigger class="inline-flex w-auto appearance-none items-center justify-center rounded-br-md rounded-tr-md border-l border-solid border-l-zinc-200 bg-zinc-100 px-2.5 py-0 text-base leading-[0] text-zinc-800 transition-[250ms] duration-[background-color]">
          <Combobox.Icon class="h-5 w-5 flex-[0_0_20px]">
            <FaSolidSort />
          </Combobox.Icon>
        </Combobox.Trigger>
      </Combobox.Control>
      <Combobox.Portal>
        <Combobox.Content class="">
          <Combobox.Listbox class="absolute z-10 max-h-24 w-full overflow-auto rounded-md bg-dark-light py-1 text-sm shadow-lg" />
        </Combobox.Content>
      </Combobox.Portal>
    </Combobox.Root>
  );
};
