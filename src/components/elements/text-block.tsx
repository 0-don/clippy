import { IconTypes } from "solid-icons";
import { JSX } from "@solidjs/web";
import { Component } from "solid-js";

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
      class={`border-border mb-7 rounded-md border border-solid shadow-2xl ${props.className}`}
    >
      <div class="bg-secondary mb-2 flex items-center justify-between px-5 pt-3 pb-2.5">
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
