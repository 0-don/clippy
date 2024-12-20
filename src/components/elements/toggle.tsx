import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Component, Setter, createEffect, createSignal } from "solid-js";

type SwitchProps = {
  checked?: boolean;
  onChange: (val: boolean) => Promise<void> | Setter<boolean> | undefined;
};

export const Toggle: Component<SwitchProps> = (props) => {
  let inputRef: HTMLInputElement | undefined;
  const [internalChecked, setInternalChecked] = createSignal(props.checked || false);

  createEffect(() => {
    if (props.checked !== undefined) {
      setInternalChecked(props.checked);
      if (inputRef) {
        inputRef.checked = props.checked;
      }
    }
  });

  const handleChange = () => {
    const newValue = !internalChecked();
    setInternalChecked(newValue);
    props.onChange(newValue);
  };

  return (
    <label class="relative inline-flex cursor-pointer items-center">
      <input ref={inputRef} type="checkbox" checked={internalChecked()} onChange={handleChange} class="peer sr-only" />
      <div
        class={`peer relative h-4 w-11 rounded-full bg-gray-200 after:absolute after:start-[2px] after:top-0 after:h-4 after:w-4 after:rounded-full after:border after:border-transparent after:bg-white after:transition-all after:content-[''] peer-checked:bg-indigo-600 peer-checked:after:translate-x-[150%] peer-checked:after:border-transparent peer-focus:outline-none dark:border-gray-600 rtl:peer-checked:after:-translate-x-full ${
          internalChecked() ? "dark:bg-gray-700" : "dark:bg-red-600"
        } after:z-[40] dark:bg-opacity-20 after:dark:bg-zinc-700`}
      >
        <div class="absolute inset-0 z-[50] flex items-center justify-between px-1">
          {internalChecked() ? <FiCheck class="ml-auto text-sm text-white" /> : <VsClose class="text-white" />}
        </div>
      </div>
    </label>
  );
};
