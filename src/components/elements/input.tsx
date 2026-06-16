import { Component, ComponentProps, JSX, splitProps } from "solid-js";
import { cn } from "../../lib/utils";

type InputProps = ComponentProps<"input"> & {
  debounce?: number;
};

export const Input: Component<InputProps> = (props) => {
  const [local, inputProps] = splitProps(props, [
    "debounce",
    "onInput",
    "class",
  ]);
  let timeoutId: number;

  const onInput: JSX.EventHandler<HTMLInputElement, InputEvent> = (e) => {
    if (!local.onInput) return;

    const handler =
      typeof local.onInput === "function" ? local.onInput : local.onInput[0];

    if (local.debounce) {
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
      }, local.debounce);
    } else {
      handler(e);
    }
  };

  return (
    <div
      class={`group flex items-center justify-between rounded-md border border-border bg-popover p-1 px-1.5 text-sm focus-within:border-primary ${local.class || ""}`}
    >
      <input
        {...inputProps}
        onInput={onInput}
        class={cn(
          "w-full appearance-none bg-transparent text-sm focus:ring-0 focus:outline-hidden text-foreground",
          local.class,
        )}
      />
    </div>
  );
};
