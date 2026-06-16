import { FiCheck } from "solid-icons/fi";
import { VsClose } from "solid-icons/vs";
import { Component } from "solid-js";
import { cn } from "../../lib/utils";

type ToggleProps = {
  checked?: boolean;
  onChange?: (val: boolean) => void;
  disabled?: boolean;
};

export const Toggle: Component<ToggleProps> = (props) => {
  return (
    <label class="relative inline-flex items-center">
      <input
        type="checkbox"
        checked={props.checked}
        onChange={() => props.onChange?.(!props.checked)}
        class="peer sr-only"
        disabled={props.disabled}
      />
      <div
        class={cn(
          "relative h-4 w-11 rounded-full peer-focus:outline-hidden after:absolute after:inset-s-0.5 after:top-0 after:h-4 after:w-4 after:rounded-full after:border after:border-transparent after:bg-secondary after:transition-all after:content-[''] peer-checked:after:translate-x-[150%] peer-checked:after:border-transparent peer-checked:rtl:after:-translate-x-full border-border",
          "dark:bg-opacity-20 peer-checked:bg-primary bg-secondary after:z-40 dark:after:bg-muted",
          props.checked
            ? "bg-primary"
            : "bg-secondary",
          props?.disabled ? "cursor-not-allowed" : "cursor-pointer",
        )}
      >
        <div class="absolute inset-0 z-50 flex items-center justify-between px-1">
          {props.checked ? (
            <FiCheck class="ml-auto text-sm text-foreground" />
          ) : (
            <VsClose class="text-foreground" />
          )}
        </div>
      </div>
    </label>
  );
};
