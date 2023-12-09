import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Component, Setter } from "solid-js";

type SwitchProps = {
  checked?: boolean;
  onChange: (val: boolean) => Promise<void> | Setter<boolean> | undefined;
};

export const Toggle: Component<SwitchProps> = (props) => {
  return (
    <label class="relative inline-flex cursor-pointer items-center">
      <input
        type="checkbox"
        checked={props.checked}
        onChange={() => props.onChange(!props.checked)}
        class="peer sr-only"
      />
      <div
        class={`peer relative h-4 w-11 rounded-full bg-gray-200 after:absolute after:start-[2px] after:h-4 after:w-4 after:rounded-full after:border after:border-transparent after:bg-white after:transition-all after:content-[''] peer-checked:bg-indigo-600 peer-checked:after:translate-x-[150%] peer-checked:after:border-transparent peer-focus:outline-none rtl:peer-checked:after:-translate-x-full dark:border-gray-600 ${
          props.checked ? "dark:bg-gray-700" : "dark:bg-red-600"
        } dark:bg-opacity-20 after:dark:bg-zinc-700`}
      >
        {props.checked ? (
          <FiCheck class="absolute left-0 top-0 z-50 translate-x-[160%]" />
        ) : (
          <VsClose class="absolute left-0 top-0 z-50" />
        )}
      </div>
    </label>
  );
};
