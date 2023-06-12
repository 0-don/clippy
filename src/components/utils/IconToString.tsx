import { FaSolidArrowUp } from "solid-icons/fa";
import { Component } from "solid-js";

interface IconToStringProps {}

export const IconToString: Component<IconToStringProps> = ({}) => {
  const html = <FaSolidArrowUp />;
  // @ts-ignore
  log({ icon: JSON.stringify(html.outerHTML) });
  return <>{html}</>;
};
