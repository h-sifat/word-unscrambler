<script lang="ts">
  import clsx from "clsx";
  import Word from "./Word.svelte";

  const { words, length } = $props<{ words: string[]; length: number }>();
</script>

<div
  class={clsx(
    "flex flex-wrap justify-center px-4 pt-6 gap-2.5 mt-5",
    "border-t-2 border-black border-dashed relative"
  )}
>
  <div
    class={clsx(
      "absolute top-0 bg-base-100 leading-none left-3 -translate-y-1/2",
      "px-2 font-bold select-none"
    )}
  >
    {length} Letters
  </div>

  {#each words as word (word)}
    <Word {word} />
  {/each}
</div>
