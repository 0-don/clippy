import { IconTypes } from 'solid-icons'
import { Component, JSX } from 'solid-js'

interface TextBlockProps {
  children: JSX.Element
  Icon: IconTypes
  title: string
  className?: string
}

export const TextBlock: Component<TextBlockProps> = (props) => {
  return (
    <div class={`mb-7 rounded-md border border-solid shadow-2xl dark:border-zinc-700 ${props.className}`}>
      <div class="mb-2 flex items-center space-x-2 bg-zinc-200 px-5 pb-2.5 pt-3 dark:bg-zinc-800">
        <props.Icon />
        <h2 class="font-semibold">{props.title}</h2>
      </div>
      {props.children}
    </div>
  )
}
