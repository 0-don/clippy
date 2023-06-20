import { Component } from "solid-js";

interface SettingsHistoryProps {}

export const SettingsHistory: Component<SettingsHistoryProps> = ({}) => {
  const [text, setText] = useState<string>();

  useEffect(() => {
    const getDatabaseInfo = async () =>
      setText(await window.electron.getDatabaseInfo());
    getDatabaseInfo();
  }, [text]);

  return (
    <>
      <div className="rounded-md border border-solid border-zinc-700 shadow-2xl">
        <div className="mb-2 flex items-center space-x-2 bg-zinc-800 px-5 pb-2.5 pt-5">
          <FontAwesomeIcon
            icon={["far", "hdd"]}
            onClick={async () =>
              setText(await window.electron.getDatabaseInfo())
            }
          />
          <h2 className="font-semibold">Local Storage</h2>
        </div>

        <ul className="mx-5 list-disc px-5 pb-5">
          <li>{text}</li>
        </ul>
      </div>

      <div className="flex w-full justify-end pt-5">
        <button
          type="button"
          onClick={async () => setText(await window.electron.clearDatabase())}
          className="inline-flex items-center space-x-2 rounded bg-zinc-600 px-4 py-2 text-sm font-bold text-white hover:bg-zinc-700"
        >
          <FontAwesomeIcon icon={["fas", "trash-alt"]} />
          <span>Clear...</span>
        </button>
      </div>
    </>
  );
};
