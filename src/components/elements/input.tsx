import { ComponentProps, JSX } from "@solidjs/web";
import { Component, omit } from "solid-js";
import { cn } from "../../lib/utils";

type InputProps = ComponentProps<"input"> & {
  debounce?: number;
};

export const Input: Component<InputProps> = (props) => {
  const inputProps = omit(props, "debounce", "onInput", "class");
  let timeoutId: number;

  const onInput: JSX.EventHandler<HTMLInputElement, InputEvent> = (e) => {
    if (!props.onInput) return;

    const handler =
      typeof props.onInput === "function" ? props.onInput : props.onInput[0];

    if (props.debounce) {
      clearTimeout(timeoutId);
      const value = e.currentTarget.value;
      const input = e.currentTarget;
      timeoutId = setTimeout(() => {
        input.value = value;
        handler({
          ...e,
          currentTarget: input,
          target: input,
        });
      }, props.debounce);
    } else {
      handler(e);
    }
  };

  return (
    <div
      class={`group border-border bg-popover focus-within:border-primary flex items-center justify-between rounded-md border p-1 px-1.5 text-sm ${props.class || ""}`}
    >
      <input
        {...inputProps}
        onInput={onInput}
        class={cn(
          "text-foreground w-full appearance-none bg-transparent text-sm focus:ring-0 focus:outline-hidden",
          props.class,
        )}
      />
    </div>
  );
};
