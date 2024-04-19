<script lang="ts">
  import { diff } from "jsondiffpatch";
  // @ts-expect-error TODO Works fine at runtime, but check breaks?
  import { format } from "jsondiffpatch/formatters/html";
  import "jsondiffpatch/formatters/styles/html.css";
  import type { Feature } from "geojson";

  export let a: Feature;
  export let b: Feature;

  let delta = diff(a.properties, b.properties);
</script>

{#if JSON.stringify(a.geometry) == JSON.stringify(b.geometry)}
  <p>Geometry is identical</p>
{:else}
  <p>Geometry differs</p>
{/if}

{@html format(delta, a.properties)}
