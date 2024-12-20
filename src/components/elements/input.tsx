import { Component } from "solid-js";

interface InputProps {
  className?: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: string;
}

export const Input: Component<InputProps> = (props) => {
  return (
    <div
      class={`${
        props.className ? props.className : ""
      } group flex items-center justify-between rounded-md border border-gray-300 p-1 px-1.5 text-sm focus-within:border-indigo-500 dark:border-dark-light dark:bg-dark-light`}
    >
      <input
        type={props.type || "text"}
        value={props.value}
        onChange={(e) => props.onChange(e.currentTarget.value)}
        placeholder={props.placeholder}
        class="w-full appearance-none bg-transparent text-sm focus:outline-none focus:ring-0 dark:text-white"
      />
    </div>
  );
};
