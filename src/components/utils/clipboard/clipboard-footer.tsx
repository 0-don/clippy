import dayjs from "dayjs";
import { Component } from "solid-js";
import { ClipboardWithRelations } from "../../../types";

interface ClipboardFooterProps {
  data: ClipboardWithRelations;
  index: number;
}

export const ClipboardFooter: Component<ClipboardFooterProps> = (props) => {
  return (
    <>
      <div class="mb-2.5 ml-10 text-left text-xs font-thin text-zinc-700 dark:text-zinc-300">
        {dayjs.utc(props.data.clipboard.created_date).fromNow()}
      </div>
      <hr class="border-zinc-400 dark:border-zinc-700" />
    </>
  );
};
