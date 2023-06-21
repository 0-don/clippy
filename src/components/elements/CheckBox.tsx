import { FiCheck } from "solid-icons/fi";
import { Component } from "solid-js";

interface CheckBoxProps {
  text: string;
  checked: boolean;
  onChange: () => void;
}

export const CheckBox: Component<CheckBoxProps> = (props) => {
  return (
    <button type="button" class="flex items-center" onClick={props.onChange}>
      <div class="relative flex h-[1.1rem] w-[1.1rem] flex-shrink-0 items-center justify-center rounded-sm border border-gray-400 bg-white dark:border-gray-700 dark:bg-dark">
        <input
          type="checkbox"
          class="checkbox absolute h-full w-full cursor-pointer opacity-0"
          checked={props.checked}
          readOnly
        />
        <div class="check-icon hidden rounded-sm bg-indigo-600 text-white">
          <FiCheck class="m-0.5 text-sm text-white" />
        </div>
      </div>
      <p class="ml-2 text-sm font-normal leading-4 text-gray-800 dark:text-gray-100">
        {props.text}
      </p>
    </button>
  );
};
