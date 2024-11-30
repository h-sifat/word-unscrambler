<script lang="ts">
  import { onDestroy, onMount, setContext } from "svelte";
  import WordGroup from "./WordGroup.svelte";
  import { type SearchResult } from "./interface";
  import ResultHeader from "./ResultHeader.svelte";

  let {
    result = {
      query: "test",
      searchTime: 0,
      sortedWords: [],
      possibleWords: [],
    },
    onDelete = () => console.log("delete result"),
  } = $props<{ result: SearchResult; onDelete: () => void }>();

  setContext("onDeleteResult", onDelete);

  let timerId: number = $state(null);
  let renderedWords: SearchResult["sortedWords"] = $state([]);

  /**
   * For large result set the UI may get stuck. That's why we're rendering
   * them in chunks in each event loop.
   */
  function renderInChunks(arg: SearchResult & { currentIdx?: number }) {
    const { possibleWords, sortedWords, currentIdx = 0 } = arg;

    if (currentIdx > sortedWords.length) return;

    if (possibleWords.length < 2000) {
      renderedWords = sortedWords;
      return;
    }

    renderedWords = sortedWords.slice(0, currentIdx + 1);

    timerId = setTimeout(
      () => renderInChunks({ ...arg, currentIdx: currentIdx + 1 }),
      0
    );
  }

  onMount(() => {
    renderInChunks(result);
  });

  onDestroy(() => {
    if (timerId) clearTimeout(timerId);
  });
</script>

<ResultHeader
  query={result.query}
  searchTime={result.searchTime}
  resultCount={result.possibleWords.length}
/>

{#each renderedWords as [length, words] (length)}
  <WordGroup {words} {length} />
{/each}
