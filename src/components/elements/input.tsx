import { Component, JSX, createSignal } from "solid-js";

type InputProps = JSX.InputHTMLAttributes<HTMLInputElement> & {
  debounce?: number;
  className?: string;
};

export const Input: Component<InputProps> = ({
  className,
  debounce = 0,
  onInput,
  value: initialValue = "",
  ...props
}) => {
  let timeoutId: number;
  const [value, setValue] = createSignal(initialValue as string);

  const handleInput: JSX.EventHandler<HTMLInputElement, InputEvent> = (e) => {
    const target = e.currentTarget;
    setValue(target.value);

    if (debounce > 0) {
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => {
        // @ts-ignore
        onInput?.(e);
      }, debounce);
    } else {
      // @ts-ignore
      onInput?.(e);
    }
  };

  return (
    <div
      class={`${
        className ? className : ""
      } group flex items-center justify-between rounded-md border border-gray-300 p-1 px-1.5 text-sm focus-within:border-indigo-500 dark:border-dark-light dark:bg-dark-light`}
    >
      <input
        {...props}
        onInput={handleInput}
        value={value()}
        class={`w-full appearance-none bg-transparent text-sm focus:outline-none focus:ring-0 dark:text-white ${props.class}`}
      />
    </div>
  );
};
