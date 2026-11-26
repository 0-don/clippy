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
      <div class="border-border bg-background relative flex h-[1.1rem] w-[1.1rem] shrink-0 items-center justify-center rounded-xs border">
        <input
          type="checkbox"
          class="checkbox absolute h-full w-full cursor-pointer opacity-0"
          checked={props.checked}
          readonly
        />
        <div class="check-icon bg-primary text-primary-foreground hidden rounded-xs">
          <FiCheck class="text-primary-foreground m-0.5 text-sm" />
        </div>
      </div>
      <p class="text-muted-foreground ml-2 text-sm leading-4 font-normal">
        {props.label}
      </p>
    </button>
  );
};
