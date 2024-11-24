<script lang="ts">
  import { setContext } from "svelte";
  import ResultHeader from "./ResultHeader.svelte";
  import WordGroup from "./WordGroup.svelte";
  import { type SearchResult } from "./interface";
  import { sortWordsBasedOnLength } from "./util";

  let {
    result = {
      query: "test",
      searchTime: 0,
      possibleWords: [],
    },
    onDelete = () => console.log("delete result"),
  } = $props<{ result: SearchResult; onDelete: () => void }>();

  setContext("onDeleteResult", onDelete);

  let sortedWords = $derived(sortWordsBasedOnLength(result.possibleWords));
</script>

{#if result.possibleWords.length}
  <ResultHeader
    query={result.query}
    searchTime={result.searchTime}
    resultCount={result.possibleWords.length}
  />

  {#each sortedWords as [length, words] (length)}
    <WordGroup {words} {length} />
  {/each}
{:else}
  <p class="font-bold text-center text-error">Sorry no words found :(</p>
{/if}
