import { Component } from "solid-js";
import { Clipboards } from "./clipboards";
import { SearchBar } from "./search-bar";

interface ClipboardHistoryProps {}

export const ClipboardHistory: Component<ClipboardHistoryProps> = ({}) => {
  return (
    <>
      <SearchBar />
      <Clipboards />
    </>
  );
};
