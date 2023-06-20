import { BsStarFill } from "solid-icons/bs";
import { Component } from "solid-js";

interface IconToStringProps {}

export const IconToString: Component<IconToStringProps> = ({}) => {
  const html = <BsStarFill />;
  // @ts-ignore
  console.log({ icon: JSON.stringify(html.outerHTML) });
  return <>{html}</>;
};
