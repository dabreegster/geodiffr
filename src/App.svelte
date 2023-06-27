<script lang="ts">
  import { MapLibre } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/streets/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
    />
  </div>
</Layout>
