<script>
  import { onMount } from "svelte";
  import { HsvPicker } from "svelte-color-picker";
  import Icon from "svelte-awesome";
  import { faPowerOff } from "@fortawesome/free-solid-svg-icons";

  import { getStatus, enable, disable, setBrightness, setColor } from "./api";

  let active = false;
  let lastColor = "#" + Math.floor(Math.random() * 16777215).toString(16);
  $: color = active ? color : "#181616";

  onMount(() => {
    getStatus().then((json) => {
      active = json.on;
      color = json.color;
    });
  });

  const handlePowerToggle = () => {
    active = !active;
    if (!active) {
      lastColor = color;
      disable();
    } else {
      color = lastColor;
      enable();
    }
  };

  const setBackgroundColor = (rgba) => {
    color = `rgb(${rgba.detail.r}, ${rgba.detail.g}, ${rgba.detail.b})`;
    if (active) {
      setColor(rgba);
    }
  };
</script>

<main style="--color: {color}">
  <div id="btn-bg" class:active on:click={handlePowerToggle}>
    <Icon data={faPowerOff} scale="3" style="vertical-align: middle;" />
  </div>
  <HsvPicker on:colorChange={setBackgroundColor} startColor={lastColor} />
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    min-height: calc(100% - 2em);
    margin: 0 auto;
    background: linear-gradient(var(--color), #181616);
    transition: all 200ms linear;
    -o-transition: all 200ms linear;
    -moz-transition: all 200ms linear;
    -webkit-transition: all 200ms linear;
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
    transition: all 200ms linear;
    -o-transition: all 200ms linear;
    -moz-transition: all 200ms linear;
    -webkit-transition: all 200ms linear;
  }

  #btn-bg.active {
    background: #2d3036;
    color: #61fc8c;
    border: 3px solid #61fc8c;
  }
</style>
