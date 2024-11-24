<script lang="ts">
  import DeleteResultBtn from "./DeleteResultBtn.svelte";

  let {
    query = "test",
    searchTime = 0,
    resultCount = 0,
  } = $props<{ query: string; resultCount: number; searchTime: number }>();
</script>

<div class="relative flex items-center justify-between mb-8 select-none">
  <div></div>

  <div class="text-center">
    <h2 class="text-xl font-bold leading-none">Results</h2>
    <p class="text-sm lowercase">
      for ({query}); found {resultCount} words {searchTime}ms
    </p>
  </div>

  <DeleteResultBtn />
</div>
