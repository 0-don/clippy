import { Component } from "solid-js";
import { Clipboards } from "./clipboards";
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
