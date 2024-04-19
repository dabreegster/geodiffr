<script lang="ts">
  import { JsonView } from "@zerodevx/svelte-json-view";
  import type { FeatureCollection } from "geojson";

  export let filename: string;
  export let name: string;
  export let opacity: number = 0.5;
  export let pinnedFeatures: FeatureCollection;
  export let color: string;

  let show: boolean = true;
  $: if (show) {
    opacity = 1.0;
  } else {
    opacity = 0.0;
  }
</script>

<div style="background: {color}"><u><b>{name}</b>: {filename}</u></div>
<label
  >Opacity:<input
    type="range"
    min="0.0"
    max="1.0"
    step="0.1"
    bind:value={opacity}
  /></label
>

<label>Show:<input type="checkbox" bind:checked={show} /></label>
{#each pinnedFeatures.features as f}
  {#if f.dataset == name}
    <JsonView json={f.properties} />
  {/if}
{/each}
