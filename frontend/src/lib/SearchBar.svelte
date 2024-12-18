<script lang="ts">
  import clsx from "clsx";
  import { Status } from "./interface";
  import { MAX_WORD_LEN } from "../config";
  import { shuffleStr, statusToBoolean, validateQuery } from "./util";

  let {
    result = "",
    prevQuery = null,
    status = Status.ERROR,
    onStopQuery = () => console.log("stop query!"),
    onSubmit = (query: string) => console.log(`submit: '${query}'`),
  } = $props<{
    result?: string;
    status?: Status;
    prevQuery?: string;
    onSubmit: (query: string) => void;
    onStopQuery: (arg?: { resetQuery?: boolean }) => void;
  }>();

  let inputElement: HTMLInputElement;

  let { isError, isCompleted, isIdle, isRunning } = $derived(
    statusToBoolean(status)
  );

  let query = $state("");
  let animatedQuery = $state("");
  let hasFocus = $state(false);
  let { isValid: isQueryValid, message: queryErrorMessage } = $derived(
    validateQuery({ query, prevQuery })
  );

  let inputError = $state(false);
  let shakeTimerId = $state(null);
  let shuffleIntervalId = $state(null);
  let displayedInputValue = $derived(
    isCompleted && result ? result : isRunning ? animatedQuery : query
  );

  $effect(() => {
    if (isRunning && !shuffleIntervalId) {
      shuffleIntervalId = setInterval(() => {
        animatedQuery = shuffleStr(animatedQuery);
      }, 50);
    }

    if (!isRunning && shuffleIntervalId) {
      clearInterval(shuffleIntervalId);
      shuffleIntervalId = null;
    }

    if (isCompleted && result) {
      query = animatedQuery = result;
    }
  });

  function validateDataBeforeInput(
    e: InputEvent & { currentTarget: EventTarget & HTMLInputElement }
  ) {
    const input = String(e.data || "").trim();
    const isInvalid = /[^a-zA-Z-\s]/g.test(input);

    if (isInvalid || (input && query.length > MAX_WORD_LEN)) {
      e.preventDefault();

      shakeInput();
    }
  }

  function handleInput(e: any) {
    const rawInput = String(e?.target?.value || "");

    const trimmedInput = rawInput
      .trim()
      .toLowerCase()
      .replace(/[^a-zA-Z-]/g, "");

    if (trimmedInput !== rawInput) {
      e.target.value = trimmedInput;
    }

    animatedQuery = query = trimmedInput;
  }

  function handleSubmit(e: any) {
    e?.preventDefault?.();

    if (!isQueryValid) {
      shakeInput();
      return;
    }

    if (query) onSubmit(query);
  }

  function handleBtnClick(e: any) {
    e?.preventDefault?.();

    if (isRunning) {
      onStopQuery({ resetQuery: false });
      return;
    }

    if (isCompleted || isError) {
      resetQuery();
      status = Status.IDLE;

      focusInput();

      return;
    }

    if (query && isQueryValid) onSubmit(query);
  }

  function shakeInput() {
    if (!shakeTimerId) {
      inputError = true;

      shakeTimerId = setTimeout(() => {
        inputError = false;
        shakeTimerId = null;
      }, 200);
    }
  }

  export function focusInput() {
    inputElement?.focus?.();
  }

  export function resetQuery() {
    query = "";
    animatedQuery = "";
  }
</script>

<form onsubmit={handleSubmit}>
  <label
    class={clsx(
      "flex items-center justify-between mx-auto mt-8 max-w-96",
      "border-2 border-black input input-bordered",
      { "shake input-error": inputError }
    )}
  >
    <input
      type="text"
      spellcheck="false"
      disabled={isRunning}
      oninput={handleInput}
      bind:this={inputElement}
      value={displayedInputValue}
      placeholder="Search (Ctrl + k)"
      class="w-0 grow placeholder:select-none"
      onbeforeinput={validateDataBeforeInput}
      onfocus={() => {
        hasFocus = true;
        if (isCompleted || isError) status = Status.IDLE;
      }}
      onblur={() => (hasFocus = false)}
    />

    <div class="tooltip" data-tip={isIdle && isQueryValid ? "Enter" : null}>
      <button
        type="button"
        aria-label="Search"
        onclick={handleBtnClick}
        disabled={!query || !isQueryValid}
        class={clsx("btn btn-sm btn-circle", {
          "btn-accent": isIdle,
          "btn-error": isRunning || isError,
        })}
      >
        {#if isIdle || isRunning}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-search-x"
          >
            <!-- X -->
            {#if isRunning}
              <path d="m13.5 8.5-5 5" />
              <path d="m8.5 8.5 5 5" />
            {/if}

            <!-- Search Glass -->
            <circle cx="11" cy="11" r="8" />
            <path d="m21 21-4.3-4.3" />
          </svg>
        {:else}
          <!-- X (clear) icon -->
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-x"
          >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
          </svg>
        {/if}
      </button>
    </div>
  </label>

  <p
    class={clsx("text-xs text-center mt-0.5 text-error invisible", {
      "!visible": !isQueryValid && query && hasFocus,
    })}
  >
    {queryErrorMessage}&nbsp;
  </p>
</form>

<style>
  .shake {
    animation: shake-animation 0.2s linear infinite;
  }

  @keyframes shake-animation {
    0% {
      transform: translateX(-5px);
    }

    25% {
      transform: translateX(0px);
    }

    50% {
      transform: translateX(-5px);
    }

    100% {
      transform: translateX(0px);
    }
  }
</style>
