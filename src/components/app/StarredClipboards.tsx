import { Component } from "solid-js";

interface StarredClipboardsProps {}

export const StarredClipboards: Component<StarredClipboardsProps> = ({}) => {
  const setClipboards = useAppStore((state) => state.setClipboards);

  useEffect(() => {
    const getClipboards = async () =>
      setClipboards(await window.electron.getClipboards({ star: true }));
    getClipboards();
  }, [setClipboards]);

  return <Clipboards star />;
};
