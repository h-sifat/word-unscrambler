<script>
  import { getContext } from "svelte";

  const onDelete = getContext("onDeleteResult") || (() => {});
</script>

<div class="tooltip" data-tip="Ctrl + l">
  <button
    onclick={onDelete}
    aria-label="Delete Result"
    class="btn btn-sm btn-outline btn-error"
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
    >
      <path d="M3 6h18" />
      <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
      <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
      <line x1="10" x2="10" y1="11" y2="17" />
      <line x1="14" x2="14" y1="11" y2="17" />
    </svg>

    <span class="sm:hidden">
      Clear <span class="extra-label">Results </span></span
    >
  </button>
</div>

<style>
  @media (max-width: 320px) {
    .extra-label {
      display: none;
    }
  }
</style>
