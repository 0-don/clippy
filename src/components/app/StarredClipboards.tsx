import { Component } from "solid-js";
import { Clipboards } from "./Clipboards";

interface StarredClipboardsProps {}

export const StarredClipboards: Component<StarredClipboardsProps> = ({}) => {
  return <Clipboards star />;
};
