<script lang="ts">
  import { tick } from "svelte";
  import { API } from "./lib/api";
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
</script>

<div class="min-h-screen p-2 mx-auto max-w-screen-xl">
  <h1 class="mt-10 text-4xl font-bold text-center">Word Unscrambler</h1>

  <SearchBar
    {onSubmit}
    result={topMatch}
    bind:this={searchBar}
    onStopQuery={onDelete}
    status={$state.status}
    prevQuery={$state.status === Status.COMPLETED ? $state.data.query : null}
  />

  <div class="mx-auto mt-8" style="width: 80%;">
    {#if $state.status === Status.COMPLETED}
      <Results {onDelete} result={$state.data} />
    {:else if $state.status === Status.ERROR}
      <p class="font-bold text-center text-error">{$state.message}</p>
    {/if}
  </div>
</div>
