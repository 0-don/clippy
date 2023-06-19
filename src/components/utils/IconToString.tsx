import { CgInfo } from "solid-icons/cg";
import { Component } from "solid-js";

interface IconToStringProps {}

export const IconToString: Component<IconToStringProps> = ({}) => {
  const html = <CgInfo />;
  // @ts-ignore
  console.log({ icon: JSON.stringify(html.outerHTML) });
  return <>{html}</>;
};
