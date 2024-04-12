<!-- Script section -->
<script lang="ts">
  import { JsonView } from "@zerodevx/svelte-json-view";
  import type { FeatureCollection } from "geojson";
  export let filename: string;
  export let label: string;
  export let opacity: number = 0.5;
  export let pinnedFeatures: FeatureCollection;
  export let color: string;
  let show: boolean = true;
  // If variable changes in the '$:', it reruns the reactive block
  $: if (show) {
    opacity = 1.0;
  } else {
    opacity = 0.0;
  }
</script>

<div style="background: {color}"><u><b>A</b>: {filename}</u></div>
<label
  >Opacity:<input
    type="range"
    min="0.0"
    max="1.0"
    step="0.1"
    bind:value={opacity}
  /></label
>

<label>Show/hide:<input type="checkbox" bind:checked={show} /></label>
{#each pinnedFeatures.features as f}
  {#if f.dataset == label}
    <JsonView json={f.properties} />
  {/if}
{/each}
