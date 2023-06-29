<script lang="ts">
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer, MapLibre, Popup } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";
  import { bbox } from "./utils";

  let sampleData;
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

  let accordionDivs = {};

  function zoomTo(feature) {
    map?.fitBounds(bbox(feature), {
      padding: 20,
      animate: true,
      duration: 500,
    });
  }

  function toggleSidebar(ev, f) {
    if (ev.target.open) {
      zoomTo(f);
    }
  }

  function openFromMap(ev) {
    let f = ev.detail.features?.[0];
    if (f) {
      accordionDivs[f.id].open = true;
      accordionDivs[f.id].scrollIntoView({ behavior: "smooth" });
    }
  }
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>
    {#if sampleData}
      <p>{sampleData.features.length} objects</p>
      {#each sampleData.features as f (f.id)}
        <details
          bind:this={accordionDivs[f.id]}
          on:toggle={(ev) => toggleSidebar(ev, f)}
        >
          <summary>{f.properties.name ?? f.properties["@id"]}</summary>
          <PropertiesTable properties={f.properties} />
        </details>
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
            paint={{ "line-width": 5, "line-color": "red" }}
            on:click={openFromMap}
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