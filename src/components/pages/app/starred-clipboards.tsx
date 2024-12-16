import { Component } from "solid-js";
import { Clipboards } from "./clipboard/clipboards";
import { SearchBar } from "./search-bar";

interface StarredClipboardsProps {}

export const StarredClipboards: Component<StarredClipboardsProps> = ({}) => {
  return (
    <>
      <SearchBar />
      <Clipboards />
    </>
  );
};
