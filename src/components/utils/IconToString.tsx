import {  FiType } from "solid-icons/fi";
import { Component } from "solid-js";

interface IconToStringProps {}

export const IconToString: Component<IconToStringProps> = ({}) => {
  const html = <FiType  />;
  // @ts-ignore
  log({ icon: JSON.stringify(html.outerHTML) });
  return <>{html}</>;
};
