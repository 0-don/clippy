import { Component, createSignal } from "solid-js";

type SliderProps = {
  value: number;
  min: number;
  max: number;
  step: number;
  format: (value: number) => string;
  onInput?: (value: number) => void;
  onChange?: (value: number) => void;
};

export const Slider: Component<SliderProps> = (props) => {
  const [dragging, setDragging] = createSignal<number>();

  return (
    <div class="flex items-center space-x-2">
      <input
        type="range"
        min={props.min}
        max={props.max}
        step={props.step}
        class="accent-primary"
        value={props.value}
        onInput={(e) => {
          const value = Number(e.currentTarget.value);
          setDragging(value);
          props.onInput?.(value);
        }}
        onChange={(e) => {
          setDragging();
          props.onChange?.(Number(e.currentTarget.value));
        }}
      />
      <span class="w-9 text-right text-xs text-muted-foreground tabular-nums">
        {props.format(dragging() ?? props.value)}
      </span>
    </div>
  );
};
