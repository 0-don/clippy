import { Switch } from "@kobalte/core";
import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Accessor, Component, Setter } from "solid-js";

type SwitchProps = {
  checked: Accessor<boolean>;
  onChange: Setter<boolean>;
};

const SwitchField: Component<SwitchProps> = ({ checked, onChange }) => {
  return (
    <Switch.Root
      class="mx-1 inline-flex cursor-pointer items-center"
      checked={checked()}
      onChange={onChange}
    >
      <Switch.Input />
      <Switch.Control class="inline-flex h-4 w-11 items-center rounded-xl bg-red-600 bg-opacity-20 transition-colors kb-checked:bg-green-600 kb-checked:bg-opacity-20">
        <Switch.Thumb class="inline-flex h-4 w-4 items-center justify-center rounded-lg bg-zinc-700 p-0.5 transition-all kb-checked:translate-x-[calc(172%)]">
          {checked() ? <FiCheck /> : <VsClose />}
        </Switch.Thumb>
      </Switch.Control>
    </Switch.Root>
  );
};

export default SwitchField;
