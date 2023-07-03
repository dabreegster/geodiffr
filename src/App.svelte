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
  import { writable, type Writable } from "svelte/store";
  import AccordionItem from "./AccordionItem.svelte";
  import Layout from "./Layout.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";
  import { bbox } from "./utils";

  // State to manage sidebar/map correspondance
  // TODO Bundle somehow?
  let formOpen: Writable<number | null> = writable(null);
  let mapHover: Writable<number | null> = writable(null);
  let sidebarHover: Writable<number | null> = writable(null);
  let openFromSidebar: Writable<number | null> = writable(null);

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

  let map;
  let hovered;
  // Glue together
  $: {
    if (hovered) {
      mapHover.set(hovered.id);
    } else {
      mapHover.set(null);
    }
  }

  function zoomTo(feature) {
    map?.fitBounds(bbox(feature), {
      padding: 20,
      animate: true,
      duration: 500,
    });
  }

  $: if ($openFromSidebar) {
    // TODO Hack!
    zoomTo(sampleData.features[$openFromSidebar - 1]);
  }

  function openFromMap(ev) {
    let f = ev.detail.features?.[0];
    formOpen.set(f.id);
    // TODO This isn't triggered for clicking emptiness
  }
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>
    {#if sampleData}
      <p>{sampleData.features.length} objects</p>
      {#each sampleData.features as f (f.id)}
        <AccordionItem
          id={f.id}
          label={f.properties.name ?? f.properties["@id"]}
          {formOpen}
          {mapHover}
          {sidebarHover}
          {openFromSidebar}
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
      bind:map
    >
      {#if sampleData}
        <GeoJSON id="data" data={sampleData}>
          <LineLayer
            manageHoverState
            paint={{
              "line-width": 5,
              "line-color": "red",
              "line-opacity": hoverStateFilter(1.0, 0.5),
            }}
            on:click={openFromMap}
            bind:hovered
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
