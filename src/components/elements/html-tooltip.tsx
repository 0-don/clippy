import { JSX, Portal } from "@solidjs/web";
import { Component, createSignal, onSettled, Show } from "solid-js";

const SIZE = 300;
const GAP = 4;

interface HtmlTooltipProps {
  html?: string;
  delay: number;
  children: JSX.Element;
}

export const HtmlTooltip: Component<HtmlTooltipProps> = (props) => {
  const [position, setPosition] = createSignal<{
    left: number;
    top: number;
    above: boolean;
  }>();
  let timer: ReturnType<typeof setTimeout> | undefined;

  const show = (trigger: HTMLElement) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      const rect = trigger.getBoundingClientRect();
      const above = rect.bottom + GAP + SIZE > window.innerHeight;
      setPosition({
        left: Math.max(
          GAP,
          Math.min(rect.left, window.innerWidth - SIZE - GAP),
        ),
        top: above ? rect.top - GAP : rect.bottom + GAP,
        above,
      });
    }, props.delay);
  };

  const hide = () => {
    clearTimeout(timer);
    setPosition(undefined);
  };

  onSettled(() => hide);

  return (
    <div
      onMouseEnter={(e) => props.html && show(e.currentTarget)}
      onMouseLeave={hide}
      onMouseDown={hide}
    >
      {props.children}
      <Show when={props.html ? position() : undefined}>
        {(pos) => (
          <Portal>
            <div
              class="bg-popover text-popover-foreground pointer-events-none fixed z-9999 truncate rounded-md p-1 shadow-lg"
              style={{
                left: `${pos().left}px`,
                top: `${pos().top}px`,
                "max-width": `${SIZE}px`,
                "max-height": `${SIZE}px`,
                transform: pos().above ? "translateY(-100%)" : undefined,
              }}
              innerHTML={props.html}
            />
          </Portal>
        )}
      </Show>
    </div>
  );
};
