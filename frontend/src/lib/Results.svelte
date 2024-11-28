<script lang="ts">
  import { setContext } from "svelte";
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
</script>

<ResultHeader
  query={result.query}
  searchTime={result.searchTime}
  resultCount={result.possibleWords.length}
/>

{#each result.sortedWords as [length, words] (length)}
  <WordGroup {words} {length} />
{/each}
