<script lang="ts">
  import clsx from "clsx";
  import { Status } from "./util";

  const { status = Status.IDLE } = $props<{
    status?: Status;
    result?: string;
  }>();

  let query = $state("");
  let animatedQuery = $state("");

  let inputError = $state(false);
</script>

<label
  class={clsx(
    "flex items-center mx-auto mt-8  max-w-96 gap-5",
    "border-2 border-black input input-bordered",
    { "shake input-error": inputError }
  )}
>
  <input
    type="text"
    onbeforeinput={(e) => {
      // if (e.data) e.preventDefault();
      const input = String(e.data || "").trim();
      const isInvalid = /[^a-zA-Z-]/g.test(input);

      if (isInvalid) {
        e.preventDefault();

        inputError = true;
        setTimeout(() => {
          inputError = false;
        }, 200);
      }

      // query += input;
      // console.log(
      //   `isInvalid: ${isInvalid}; input: "${input}"; e.target.value:"${e.target.value}"`
      // );
    }}
    oninput={(e) => {
      console.log("e.target.data:", e.target.value);
      query = "hi";
    }}
    placeholder="Search (Ctrl + k)"
    class="grow placeholder:select-none"
    disabled={status === Status.RUNNING}
    value={status === Status.RUNNING ? animatedQuery : query}
  />

  <div class="tooltip" data-tip="Enter">
    <button
      aria-label="Search"
      class={clsx("btn btn-sm btn-circle", {
        "btn-accent": status === Status.IDLE,
        "btn-error": status === Status.RUNNING,
      })}
    >
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
        {#if status === Status.RUNNING}
          <path d="m13.5 8.5-5 5" />
          <path d="m8.5 8.5 5 5" />
        {/if}

        <!-- Glass -->
        <circle cx="11" cy="11" r="8" />
        <path d="m21 21-4.3-4.3" />
      </svg>
    </button>
  </div>
</label>

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
