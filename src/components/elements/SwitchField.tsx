import { Switch } from "@kobalte/core";
import { Accessor, Component, Setter } from "solid-js";

type SwitchProps = {
  checked?: Accessor<boolean | undefined> | boolean;
  onChange?: ((check: boolean) => void) | Setter<boolean>;
};

function className(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

const SwitchField: Component<SwitchProps> = ({ checked, onChange }) => {
  const res = typeof checked === "function" ? checked() : checked;
  // const [checked, setChecked] = createSignal(false);
  return (
    <Switch.Root
      checked={res}
      onChange={(e) => onChange && onChange(e)}
      class="switch"
    >
      <Switch.Label class="switch__label">Airplane mode</Switch.Label>
      <Switch.Input class="switch__input" />
      <Switch.Control class="switch__control">
        <Switch.Thumb class="switch__thumb" />
      </Switch.Control>
    </Switch.Root>
    // <Toggle>
    //   {checked() && <AiOutlineCheck class="mr-3 text-white" />}
    //   <span
    //     aria-hidden="true"
    //     class="pointer-events-none absolute h-full w-full rounded-md"
    //   />

    //   <span
    //     aria-hidden="true"
    //     class={className(
    //       checked() ? "border-white" : "border-gray-500",
    //       "pointer-events-none absolute mx-auto h-4 w-9 rounded-full border-2 transition-colors duration-200 ease-in-out"
    //     )}
    //   />
    //   <span
    //     aria-hidden="true"
    //     class={className(
    //       checked() ? "translate-x-6 bg-white" : "translate-x-1 bg-gray-500 ",
    //       "pointer-events-none absolute left-0 flex h-2 w-2 transform items-center justify-center rounded-full text-xs shadow ring-0 transition-transform duration-200 ease-in-out"
    //     )}
    //   />
    // </Toggle>
  );
};

export default SwitchField;
