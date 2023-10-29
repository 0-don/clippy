import { Component } from "solid-js";
import { Clipboards } from "./Clipboards";
import { SearchBar } from "./SearchBar";

interface StarredClipboardsProps {}

export const StarredClipboards: Component<StarredClipboardsProps> = ({}) => {
  return (
    <>
      <SearchBar />
      <Clipboards />
    </>
  );
};
