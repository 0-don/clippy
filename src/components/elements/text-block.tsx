import { IconTypes } from "solid-icons";
import { Component, JSX } from "solid-js";

interface TextBlockProps {
  children: JSX.Element;
  Icon: IconTypes;
  title: string;
  className?: string;
  header?: JSX.Element;
}

export const TextBlock: Component<TextBlockProps> = (props) => {
  return (
    <div
      class={`mb-7 rounded-md border border-solid border-border shadow-2xl ${props.className}`}
    >
      <div class="mb-2 flex items-center justify-between bg-secondary px-5 pt-3 pb-2.5">
        <div class="flex items-center gap-2">
          <props.Icon />
          <h2 class="font-semibold">{props.title}</h2>
        </div>
        <div>{props.header}</div>
      </div>
      {props.children}
    </div>
  );
};
