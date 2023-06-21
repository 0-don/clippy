import { Switch } from "@kobalte/core";
import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Accessor, Component, Setter } from "solid-js";

type SwitchProps = {
  checked?: Accessor<boolean> | boolean;
  onChange: (val: boolean) => Promise<void> | Setter<boolean> | undefined;
};

const SwitchField: Component<SwitchProps> = ({ checked, onChange }) => {
  const getChecked = () =>
    typeof checked === "function" ? checked() : checked;

  return (
    <Switch.Root
      class="mx-1 inline-flex cursor-pointer items-center"
      checked={getChecked()}
      onChange={onChange}
    >
      <Switch.Input />
      <Switch.Control class="inline-flex h-4 w-11 items-center rounded-xl bg-red-600 bg-opacity-20 transition-colors kb-checked:bg-indigo-600">
        <Switch.Thumb class="inline-flex h-4 w-4 items-center justify-center rounded-lg bg-zinc-700 p-0.5 transition-all kb-checked:translate-x-[calc(172%)]">
          {getChecked() ? <FiCheck /> : <VsClose />}
        </Switch.Thumb>
      </Switch.Control>
    </Switch.Root>
  );
};

export default SwitchField;
