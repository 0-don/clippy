import { Component } from "solid-js";
import { Clipboards } from "./Clipboards";
import { SearchBar } from "./SearchBar";

interface ClipboardHistoryProps {}

export const ClipboardHistory: Component<ClipboardHistoryProps> = ({}) => {
  return (
    <>
      <SearchBar />
      <Clipboards />
    </>
  );
};
