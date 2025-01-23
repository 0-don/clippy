import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Component } from "solid-js";

type ToggleProps = {
  checked?: boolean;
  onChange?: (val: boolean) => Promise<void> | undefined;
  disabled?: boolean;
};

export const Toggle: Component<ToggleProps> = (props) => {
  return (
    <label class="relative inline-flex cursor-pointer items-center">
      <input
        type="checkbox"
        checked={props.checked}
        onChange={() => props.onChange?.(!props.checked)}
        class="peer sr-only"
        disabled={props.disabled}
      />
      <div
        class={`peer relative h-4 w-11 rounded-full bg-gray-200 peer-checked:bg-indigo-600 peer-focus:outline-hidden after:absolute after:start-[2px] after:top-0 after:h-4 after:w-4 after:rounded-full after:border after:border-transparent after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-[150%] peer-checked:after:border-transparent peer-checked:rtl:after:-translate-x-full dark:border-gray-600 ${
          props.checked ? "dark:bg-indigo-700" : "dark:bg-dark-light"
        } dark:bg-opacity-20 after:z-40 dark:after:bg-zinc-700`}
      >
        <div class="absolute inset-0 z-50 flex items-center justify-between px-1">
          {props.checked ? <FiCheck class="ml-auto text-sm text-white" /> : <VsClose class="text-white" />}
        </div>
      </div>
    </label>
  );
};
