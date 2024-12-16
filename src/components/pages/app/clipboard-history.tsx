import { Component } from "solid-js";
import { Clipboards } from "./clipboard/clipboards";
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
