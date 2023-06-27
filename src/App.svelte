<script lang="ts">
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer, MapLibre } from "svelte-maplibre";
  import Layout from "./Layout.svelte";

  let sampleData;
  onMount(async () => {
    let resp = await fetch("/geodiffr/overpass_example.geojson");
    sampleData = await resp.json();
  });
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/streets/style.json?key=MZEJTanw3WpxRvt7qDfo"
      center={[-0.1095, 51.5076]}
      zoom={13}
      standardControls
    >
      {#if sampleData}
        <GeoJSON id="data" data={sampleData}>
          <LineLayer paint={{ "line-width": 5, "line-color": "red" }} />
        </GeoJSON>
      {/if}
    </MapLibre>
  </div>
</Layout>
