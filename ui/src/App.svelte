<script>
  import { HsvPicker } from "svelte-color-picker";
  import Button from "./Button.svelte";

  let color = "#fbfbfb";
  const setBackgroundColor = (rgba) => {
    color = `rgba(${rgba.detail.r}, ${rgba.detail.g}, ${rgba.detail.b}, ${rgba.detail.a})`;
  };
</script>

<style>
  main {
    text-align: center;
    padding: 1em;
    min-height: calc(100% - 2em);
    margin: 0 auto;
    background: linear-gradient(var(--color), #181616);
  }
</style>

<main style="--color: {color}">
  <Button />
  <HsvPicker on:colorChange={setBackgroundColor} startColor={color} />
</main>
