<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { onMount } from "svelte";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    Popup,
  } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import AccordionItem from "./map_sidebar/AccordionItem.svelte";
  import MapSidebar from "./map_sidebar/MapSidebar.svelte";
  import { formOpen, mapHover } from "./map_sidebar/stores";
  import PropertiesTable from "./PropertiesTable.svelte";

  let sampleData: FeatureCollection;
  onMount(async () => {
    let resp = await fetch("/geodiffr/overpass_example.geojson");
    let gj = await resp.json();
    // Assign our own IDs, excluding 0
    var id = 1;
    for (let f of gj.features) {
      f.id = id++;
    }
    sampleData = gj;
  });
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>
    {#if sampleData}
      <p>{sampleData.features.length} objects</p>
      {#each sampleData.features as f (f.id)}
        <AccordionItem
          feature={f}
          label={f.properties.name ?? f.properties["@id"]}
        >
          <PropertiesTable properties={f.properties} />
        </AccordionItem>
      {/each}
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/streets/style.json?key=MZEJTanw3WpxRvt7qDfo"
      center={[-0.1095, 51.5076]}
      zoom={13}
      standardControls
    >
      <MapSidebar />
      {#if sampleData}
        <GeoJSON id="data" data={sampleData}>
          <LineLayer
            manageHoverState
            paint={{
              "line-width": 5,
              "line-color": "red",
              "line-opacity": hoverStateFilter(1.0, 0.5),
            }}
            on:click={(e) => formOpen.set(e.detail.features[0].id)}
            bind:hovered={$mapHover}
          >
            >
            <Popup openOn="hover" let:features>
              <PropertiesTable properties={features[0].properties} />
            </Popup>
          </LineLayer>
        </GeoJSON>
      {/if}
    </MapLibre>
  </div>
</Layout>
