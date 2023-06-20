import { RiDeviceKeyboardFill } from "solid-icons/ri";
import { Component } from "solid-js";

interface IconToStringProps {}

export const IconToString: Component<IconToStringProps> = ({}) => {
  const html = <RiDeviceKeyboardFill />;
  // @ts-ignore
  console.log({ icon: JSON.stringify(html.outerHTML) });
  return <>{html}</>;
};
