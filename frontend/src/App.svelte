<script lang="ts">
  import { tick } from "svelte";
  import { API } from "./lib/api";
  import About from "./lib/About.svelte";
  import { Status } from "./lib/interface";
  import Results from "./lib/Results.svelte";
  import SearchBar from "./lib/SearchBar.svelte";

  const api = new API();

  const state = api.state;
  const onSubmit = (query: string) => api.search(query);

  let topMatch = $derived(
    $state.status === Status.COMPLETED ? $state.data.topMatch : ""
  );

  let searchBar: any;

  async function onDelete(arg: { resetQuery?: boolean } = {}) {
    api.stopOrReset();

    if (searchBar) {
      const { resetQuery = true } = arg;
      if (resetQuery) searchBar.resetQuery();

      await tick();

      searchBar.focusInput();
    }
  }

  window.onkeydown = (e: KeyboardEvent) => {
    const hasMetaKey = e.ctrlKey || e.metaKey;

    if (!hasMetaKey) return;

    if (e.key.toLowerCase() === "k") {
      e.preventDefault();
      searchBar.focusInput();
      return;
    }

    if (e.key.toLowerCase() === "l") {
      e.preventDefault();
      onDelete();
    }
  };
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="flex flex-col min-h-screen p-2 mx-auto max-w-screen-xl">
  <div>
    <h1 class="mt-10 text-4xl font-bold text-center">Word Unscrambler</h1>

    <SearchBar
      {onSubmit}
      result={topMatch}
      bind:this={searchBar}
      onStopQuery={onDelete}
      status={$state.status}
      prevQuery={$state.status === Status.COMPLETED ? $state.data.query : null}
    />
  </div>

  <div class="w-4/5 mx-auto grow mt-1">
    {#if $state.status === Status.COMPLETED}
      <Results {onDelete} result={$state.data} />
    {:else if $state.status === Status.ERROR}
      <p class="font-bold text-center text-error">{$state.message}</p>
    {/if}
  </div>

  <div class="mt-5">
    <About />
  </div>
</div>
