<script lang="ts">
  let {
    visualizerLink = "https://trie-visualizer.netlify.app/",
    repoLink = "https://github.com/h-sifat/word-unscrambler",
  } = $props();
</script>

<div
  class="pt-5 mx-auto text-sm text-center border-t-2 border-black max-w-prose"
>
  <p>
    This app is a hobby project I built while learning Rust and Svelte 5. View
    the code on
    <a
      href={repoLink}
      target="_blank"
      rel="noreferrer"
      class="link link-accent"
    >
      GitHub
    </a>
    or
    <a
      target="_blank"
      rel="noreferrer"
      href={visualizerLink}
      class="link link-accent"
    >
      visualize
    </a>
    the algorithm here.
  </p>

  <p class="my-2 font-bold">⋆✩✧ Shortcuts ✧✩⋆</p>
  <p>
    Clear results:
    <kbd class="kbd kbd-xs">Ctrl/Cmd + l</kbd>
  </p>
  <p>
    Focus search bar:
    <kbd class="kbd kbd-xs">Ctrl/Cmd + k</kbd>
  </p>
  <p>
    Go to definition:
    <kbd class="kbd kbd-xs">Ctrl/Cmd + Click</kbd>
  </p>
</div>
