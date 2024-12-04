<script lang="ts">
  import DeleteResultBtn from "./DeleteResultBtn.svelte";

  let {
    query = "test",
    searchTime = 0,
    resultCount = 0,
  } = $props<{ query: string; resultCount: number; searchTime: number }>();
</script>

<div
  class="relative flex flex-col items-center justify-between mb-8 select-none sm:flex-row gap-1"
>
  <div></div>

  <div class="text-center">
    <h2 class="text-xl font-bold leading-none">Results</h2>
    <p class="text-sm lowercase">
      for (<span class="select-text">{query}</span>); found {resultCount} word(s)
      in {searchTime}ms
    </p>
  </div>

  <div class="mt-2 sm:mt-0">
    <DeleteResultBtn />
  </div>
</div>
