import { FiCheck } from "solid-icons/fi";
import { Component } from "solid-js";

interface CheckBoxProps {
  label?: string;
  checked: boolean;
  onChange: (check: boolean) => void;
}

export const CheckBox: Component<CheckBoxProps> = (props) => {
  return (
    <button
      type="button"
      class="flex items-center"
      onClick={() => props.onChange(!props.checked)}
    >
      <div class="dark:bg-dark relative flex h-[1.1rem] w-[1.1rem] shrink-0 items-center justify-center rounded-xs border border-gray-400 bg-white dark:border-gray-700">
        <input
          type="checkbox"
          class="checkbox absolute h-full w-full cursor-pointer opacity-0"
          checked={props.checked}
          readOnly
        />
        <div class="check-icon hidden rounded-xs bg-indigo-600 text-white">
          <FiCheck class="m-0.5 text-sm text-white" />
        </div>
      </div>
      <p class="ml-2 text-sm leading-4 font-normal text-gray-800 dark:text-gray-100">
        {props.label}
      </p>
    </button>
  );
};
