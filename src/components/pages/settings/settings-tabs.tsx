import { Component, For, createSignal, onCleanup, onMount } from "solid-js";
import { SettingsStore } from "../../../store/settings-store";
import { useLanguage } from "../../provider/language-provider";

interface TabsProps {}

export const Tabs: Component<TabsProps> = ({}) => {
  const { t } = useLanguage();
  let isMouseDown = false;
  let startX: number;
  let scrollLeft: number;
  const [navRef, setNavRef] = createSignal<HTMLElement>();

  const scrollTabIntoView = (index: number) => {
    const nav = navRef();
    if (!nav) return;

    const tabElement = nav.querySelectorAll("button")[index];
    if (!tabElement) return;

    const navRect = nav.getBoundingClientRect();
    const tabRect = tabElement.getBoundingClientRect();

    if (tabRect.left < navRect.left) {
      nav.scrollTo({
        left: nav.scrollLeft - (navRect.left - tabRect.left),
        behavior: "smooth",
      });
    } else if (tabRect.right > navRect.right) {
      nav.scrollTo({
        left: nav.scrollLeft + (tabRect.right - navRect.right),
        behavior: "smooth",
      });
    }
  };

  const handleKeyNavigation = (e: KeyboardEvent) => {
    const tabs = SettingsStore.tabs();
    const currentIndex = tabs.findIndex((tab) => tab.current);

    if (e.key === "ArrowLeft") {
      const newIndex = currentIndex > 0 ? currentIndex - 1 : tabs.length - 1;
      SettingsStore.setCurrentTab(tabs[newIndex].name);
      scrollTabIntoView(newIndex);
    } else if (e.key === "ArrowRight") {
      const newIndex = currentIndex < tabs.length - 1 ? currentIndex + 1 : 0;
      SettingsStore.setCurrentTab(tabs[newIndex].name);
      scrollTabIntoView(newIndex);
    }
  };

  onMount(() => {
    window.addEventListener("keydown", handleKeyNavigation);
  });

  onCleanup(() => {
    window.removeEventListener("keydown", handleKeyNavigation);
  });

  const onMouseDown = (e: MouseEvent) => {
    isMouseDown = true;
    const nav = navRef();
    if (!nav) return;
    startX = e.pageX - nav.offsetLeft;
    scrollLeft = nav.scrollLeft;
  };

  const onMouseUp = () => {
    isMouseDown = false;
  };

  const onMouseLeave = () => {
    isMouseDown = false;
  };

  const onMouseMove = (e: MouseEvent) => {
    const nav = navRef();
    if (!isMouseDown || !nav) return;
    e.preventDefault();
    const x = e.pageX - nav.offsetLeft;
    const walk = (x - startX) * 2;
    nav.scrollLeft = scrollLeft - walk;
  };

  return (
    <div class="border-b border-gray-500">
      <div class="flex justify-center">
        <nav
          ref={setNavRef}
          class="scrollbar-hide -mb-px flex max-w-full cursor-grab overflow-x-auto active:cursor-grabbing"
          style="scroll-behavior: smooth"
          onMouseDown={onMouseDown}
          onMouseUp={onMouseUp}
          onMouseLeave={onMouseLeave}
          onMouseMove={onMouseMove}
        >
          <div class="flex flex-nowrap">
            <For each={SettingsStore.tabs()}>
              {({ Icon, current, name }) => (
                <button
                  type="button"
                  class={`${
                    current
                      ? "border-zinc-600 text-zinc-600 dark:border-white dark:text-white"
                      : "border-transparent hover:border-zinc-600 hover:text-zinc-600 dark:text-gray-500 dark:hover:border-white dark:hover:text-white"
                  } group inline-flex cursor-pointer items-center border-b-2 px-2 py-4 text-sm font-medium whitespace-nowrap`}
                  onClick={() => SettingsStore.setCurrentTab(name)}
                  title={t(name)}
                >
                  <Icon class="text-1xl mr-2 dark:text-white" />
                  <span class="max-w-20 truncate">{t(name)}</span>
                </button>
              )}
            </For>
          </div>
        </nav>
      </div>
    </div>
  );
};
