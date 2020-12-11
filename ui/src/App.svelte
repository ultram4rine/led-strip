<script>
  import { HsvPicker } from "svelte-color-picker";
  import Icon from "svelte-awesome";
  import { faPowerOff } from "@fortawesome/free-solid-svg-icons";

  let active = false;

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

  #btn-bg {
    width: fit-content;
    height: fit-content;
    background: #cdd3c9;
    color: #2a2a2a;
    border-radius: 3em;
    margin: 0 auto 20px;
    border: 3px solid #2a2a2a;
    overflow: hidden;
    position: relative;
    cursor: pointer;
    padding: 1em;
  }

  #btn-bg.active {
    background: #2d3036;
    color: #61fc8c;
    border: 3px solid #61fc8c;
  }
</style>

<main style="--color: {color}">
  <div id="btn-bg" class:active on:click={() => (active = !active)}>
    <Icon data={faPowerOff} scale="3" style="vertical-align: middle;" />
  </div>
  <HsvPicker on:colorChange={setBackgroundColor} startColor={color} />
</main>
