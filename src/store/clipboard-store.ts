import { Channel } from "@tauri-apps/api/core";
import { createRoot, createSignal } from "solid-js";
import { invokeCommand } from "../lib/tauri";
import {
  ClipboardWhere,
  ClipboardWithRelations,
  DecryptEvent,
  Progress,
  SearchEvent,
} from "../types";
import { InvokeCommand } from "../types/tauri-invoke";
import { AppStore } from "./app-store";

export const initialWhere: ClipboardWhere = {
  cursor: undefined,
  search: undefined,
  star: undefined,
  img: undefined,
};

function createClipboardStore() {
  const [clipboardSyncProgress, setClipboardSyncProgress] =
    createSignal<Progress>();
  const [clipboardRef, setClipboardRef] = createSignal<
    HTMLDivElement | undefined
  >();
  const [clipboards, setClipboards] = createSignal<ClipboardWithRelations[]>(
    [],
  );
  const [where, setWhere] = createSignal<ClipboardWhere>(initialWhere);
  const [hasMore, setHasMore] = createSignal(true);
  // Monotonic id to discard stale streamed search results (latest call wins).
  let searchRequestId = 0;
  const resetWhere = () => setWhere(initialWhere);
  const [selectedIndex, setSelectedIndex] = createSignal(-1);
  const [isSearching, setIsSearching] = createSignal(false);
  const [expandedIds, setExpandedIds] = createSignal<Set<number>>(new Set());

  const toggleExpanded = (id: number) => {
    setExpandedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const isExpanded = (id: number) => expandedIds().has(id);

  const getClipboards = async () => {
    setIsSearching(true);
    try {
      const response = await invokeCommand(InvokeCommand.GetClipboards, where());
      setHasMore(response.has_more);
      return response.clipboards;
    } finally {
      setIsSearching(false);
    }
  };

  const searchStream = async (
    search?: string,
    star?: boolean,
    img?: boolean,
  ) => {
    // Supersede any in-flight search: a newer call bumps the id, so stale Channel
    // batches (the backend can't hard-abort the promise) are ignored on arrival.
    const requestId = searchRequestId + 1;
    searchRequestId = requestId;

    setIsSearching(true);
    setClipboards([]);
    setHasMore(false);

    const onChunk = new Channel<SearchEvent>();
    onChunk.onmessage = (message) => {
      if (requestId !== searchRequestId) return;
      if (message.event === "batch") {
        setClipboards((prev) => [...prev, ...message.data.clipboards]);
      } else if (message.event === "done") {
        setIsSearching(false);
      }
    };

    try {
      await invokeCommand(InvokeCommand.SearchClipboards, {
        search: search || undefined,
        star: star || undefined,
        img: img || undefined,
        onChunk,
      });
    } catch {
      if (requestId === searchRequestId) setIsSearching(false);
    }
  };

  // Merge incoming clipboards into the list: dedupe by id (incoming wins), then sort
  // newest-first by created_at. Streaming decrypt batches arrive in arbitrary page
  // order, so the client always sorts and server page order does not matter.
  const mergeClipboards = (
    prev: ClipboardWithRelations[],
    incoming: ClipboardWithRelations[],
  ): ClipboardWithRelations[] => {
    const incomingIds = new Set(incoming.map((item) => item.clipboard.id));
    const merged = [
      ...incoming,
      ...prev.filter((item) => !incomingIds.has(item.clipboard.id)),
    ];
    return merged.sort((a, b) => {
      const dateA = new Date(a.clipboard.created_at).getTime();
      const dateB = new Date(b.clipboard.created_at).getTime();
      return dateB - dateA;
    });
  };

  const newClipboard = (clipboard: ClipboardWithRelations) => {
    setClipboards((prev) => mergeClipboards(prev, [clipboard]));
  };

  const unlockStream = async (
    command:
      | InvokeCommand.PasswordUnlockStream
      | InvokeCommand.DisableEncryptionStream,
    password: string,
  ) => {
    setIsSearching(true);

    const onChunk = new Channel<DecryptEvent>();
    onChunk.onmessage = (message) => {
      if (message.event === "batch") {
        setClipboards((prev) =>
          mergeClipboards(prev, message.data.clipboards),
        );
        setClipboardSyncProgress({
          label: "SETTINGS.ENCRYPT.DECRYPTION_PROGRESS_LOCAL",
          current: message.data.current,
          total: message.data.total,
        });
      } else if (message.event === "done") {
        setIsSearching(false);
        setClipboardSyncProgress(undefined);
      }
    };

    try {
      await invokeCommand(command, { password, onChunk });
    } finally {
      setIsSearching(false);
    }
  };

  const init = async () => {
    const currentTab = AppStore.getCurrentTab();

    if (currentTab?.name !== "MAIN.HOTKEY.RECENT_CLIPBOARDS") {
      return;
    }

    setWhere(initialWhere);
    setExpandedIds(new Set<number>());
    const clipboards = await getClipboards();
    setClipboards(clipboards);
    ClipboardStore.clipboardRef()?.scrollTo(0, 0);
  };

  const resetClipboards = async () => {
    setWhere(initialWhere);
    setHasMore(true);
    setExpandedIds(new Set<number>());
    const clipboards = await getClipboards();
    setClipboards(clipboards);
  };

  const handleKeyDown = async (e: KeyboardEvent) => {
    if (!clipboards().length) return;

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        const isLastItem = selectedIndex() === clipboards().length - 1;

        // If we're at the last item and there's more data, load more
        if (isLastItem && hasMore()) {
          setWhere((prev) => ({ ...prev, cursor: clipboards().length }));
          const newClipboards = await getClipboards();
          setClipboards((prev) => [...prev, ...newClipboards]);
          // Move to next item after loading more
          setSelectedIndex((prev) => prev + 1);
        } else {
          // Normal navigation if not at last item or no more data
          setSelectedIndex((prev) =>
            prev < clipboards().length - 1 ? prev + 1 : prev,
          );
        }

        // Ensure selected item is visible
        const nextElement = clipboardRef()?.children[selectedIndex() + 1];
        nextElement?.scrollIntoView({ block: "nearest" });
        break;

      case "ArrowUp":
        e.preventDefault();
        setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
        // Ensure selected item is visible
        const prevElement = clipboardRef()?.children[selectedIndex()];
        prevElement?.scrollIntoView({ block: "nearest" });
        break;

      case "Enter":
        if (selectedIndex() >= 0) {
          const clipboard = clipboards()[selectedIndex()];
          const type = clipboard.clipboard.types[0];
          await invokeCommand(InvokeCommand.CopyClipboard, {
            id: clipboard.clipboard.id,
            type,
          });
        }
        break;
    }
  };

  return {
    clipboards,
    newClipboard,
    setClipboards,
    where,
    setWhere,
    hasMore,
    setHasMore,
    resetClipboards,
    resetWhere,
    getClipboards,
    searchStream,
    unlockStream,
    mergeClipboards,
    clipboardRef,
    setClipboardRef,
    init,
    handleKeyDown,
    selectedIndex,
    setSelectedIndex,
    clipboardSyncProgress,
    setClipboardSyncProgress,
    isSearching,
    setIsSearching,
    expandedIds,
    toggleExpanded,
    isExpanded,
  };
}

export const ClipboardStore = createRoot(createClipboardStore);
export type ClipboardStore = ReturnType<typeof createClipboardStore>;
