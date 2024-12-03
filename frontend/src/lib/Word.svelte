<script context="module" lang="ts">
  /**
   * [top, left, size, transparent][]
   */
  const postions: [number, number, number, number][] = [
    [-50, 10, 5, 0],
    [12, 127, 7, 0],
    [40, 135, 8, 1],
    [-70, 50, 6, 1],
    [50, -40, 4, 0],
    [70, -20, 8, 1],
    [100, 100, 5, 0],
    [140, 50, 5.9, 0],
    [125, 20, 5, 0],
    [-10, -10, 6.5, 0],
    [-40, -40, 7.2, 0],
    [130, -40, 7.4, 1],
    [130, 120, 7.6, 0],
    [-50, 100, 6.8, 0],
  ];

  export function positionStyles() {
    return postions
      .map(
        ([top, left], idx) =>
          `.word-${idx} {\n\ttop: ${top}px;\n\tleft: ${left}px;\n}`
      )
      .join("\n");
  }

  const copiedMessageClasses = Object.freeze([
    "-rotate-6 left-2 -top-4",
    "rotate-6 right-2 -top-4",
  ]);

  function getRandomCopiedMessagePositionClsses() {
    return copiedMessageClasses[
      Math.floor(Math.random() * copiedMessageClasses.length)
    ];
  }
</script>

<script lang="ts">
  import clsx from "clsx";
  import { writeClipboardText } from "./util";

  let { word } = $props<{ word: string }>();

  let isAnimationRunning = $state(false);
  let hasClicked = $state(false);

  function handleClick() {
    writeClipboardText(word);

    if (!hasClicked) {
      hasClicked = true;
    }

    if (isAnimationRunning) return;
    isAnimationRunning = true;
  }
</script>

<button
  class={clsx(
    "px-1 py-0.5 border border-black border-dashed hover:border-solid relative",
    "hover:border-accent rounded ring-accent bg-base-100",
    { "active:ring-2 active:ring-inset": !isAnimationRunning }
  )}
  onclick={handleClick}
  >{word}

  <span
    class={clsx(
      "opacity-0 absolute z-10",
      "text-xs font-bold text-accent",
      getRandomCopiedMessagePositionClsses(),
      { "copy-message-animation": isAnimationRunning }
    )}
  >
    Copied!
  </span>

  <!-- Render them lazily -->
  {#if hasClicked}
    {#each postions as [_, __, size, isTransparent], idx (idx)}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span
        onanimationend={() => (isAnimationRunning = false)}
        class={clsx(
          "absolute border-accent rounded-full -z-10 invisible",
          `word-${idx}`,
          {
            "bg-orange-600": !isTransparent,
            "copy-confetti-animation": isAnimationRunning,
          }
        )}
        style="width: {size}px; height: {size}px; border-width: 1.5px"
      ></span>
    {/each}
  {/if}
</button>

<style>
  .copy-confetti-animation {
    opacity: 0;
    visibility: visible;
    animation: splash 0.8s ease-out forwards;
  }

  .copy-message-animation {
    opacity: 0%;
    animation: fade-in-n-out 0.6s 0.2s ease-in forwards;
  }

  @keyframes fade-in-n-out {
    10% {
      opacity: 0.3;
    }

    30% {
      opacity: 1;
    }

    90% {
      opacity: 0.7;
    }

    100% {
      opacity: 0;
    }
  }

  @keyframes splash {
    0% {
      top: 50%;
      left: 50%;
      opacity: 1;
    }

    70% {
      transform: scale(0.9);
    }

    80% {
      opacity: 0.8;
      transform: scale(0.7);
    }

    100% {
      opacity: 1;
      transform: scale(0);
      visibility: hidden;
    }
  }

  .word-0 {
    top: -50%;
    left: 10%;
  }
  .word-1 {
    top: 12%;
    left: 127%;
  }
  .word-2 {
    top: 40%;
    left: 135%;
  }
  .word-3 {
    top: -70%;
    left: 50%;
  }
  .word-4 {
    top: 50%;
    left: -40%;
  }
  .word-5 {
    top: 70%;
    left: -20%;
  }
  .word-6 {
    top: 100%;
    left: 100%;
  }
  .word-7 {
    top: 140%;
    left: 50%;
  }
  .word-8 {
    top: 125%;
    left: 20%;
  }
  .word-9 {
    top: -10%;
    left: -10%;
  }
  .word-10 {
    top: -40%;
    left: -40%;
  }
  .word-11 {
    top: 130%;
    left: -40%;
  }
  .word-12 {
    top: 130%;
    left: 120%;
  }
  .word-13 {
    top: -50%;
    left: 100%;
  }
</style>
