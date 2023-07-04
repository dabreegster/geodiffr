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
  import AuditControls from "./AuditControls.svelte";
  import AuditForm from "./AuditForm.svelte";
  import Layout from "./Layout.svelte";
  import AccordionItem from "./map_sidebar/AccordionItem.svelte";
  import MapSidebar from "./map_sidebar/MapSidebar.svelte";
  import { activeFeature, mapHover } from "./map_sidebar/stores";
  import PropertiesTable from "./PropertiesTable.svelte";
  import { auditData } from "./shared";

  let inputData: FeatureCollection;
  let comparisonData: FeatureCollection;
  let showComparison = true;

  $: comparisonLayer = {
    visibility: showComparison ? "visible" : "none",
  };

  onMount(async () => {
    let resp1 = await fetch("/geodiffr/overpass_example.geojson");
    inputData = fixIDs(fixInputData(await resp1.json()));

    let resp2 = await fetch("/geodiffr/cid.geojson");
    comparisonData = fixIDs(fixComparisonData(await resp2.json()));
  });

  function fixInputData(gj: FeatureCollection) {
    for (let f of gj.features) {
      if (f.properties["@id"]) {
        let id = f.properties["@id"].replace("way/", "");
        f.properties.url = `http://openstreetmap.org/way/${id}`;
        delete f.properties["@id"];
      }
    }
    return gj;
  }

  function fixComparisonData(gj: FeatureCollection) {
    // The input right now is only separated infrastructure, so temporarily
    // filter the CID data. This'd be per-dataset logic later on.
    gj.features = gj.features.filter((f) => f.properties.CLT_SEGREG);
    return gj;
  }

  // Assign our own IDs, excluding 0
  function fixIDs(gj: FeatureCollection) {
    var id = 1;
    for (let f of gj.features) {
      f.id = id++;
    }
    return gj;
  }
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>
    {#if comparisonData}
      <fieldset>
        <p>Comparison data: {comparisonData.features.length} objects</p>
        <label
          >Show: <input type="checkbox" bind:checked={showComparison} /></label
        >
      </fieldset>
    {/if}
    {#if inputData}
      <p>Input data: {inputData.features.length} objects</p>
      <AuditControls />
      <div class="list">
        {#each inputData.features as f (f.id)}
          <AccordionItem
            feature={f}
            label={f.properties.name ?? f.properties.url}
          >
            <AuditForm bind:data={$auditData[f.id]} />
            <PropertiesTable properties={f.properties} />
            {#if f.properties.url}
              <a href={f.properties.url} target="_blank">Open OSM way</a>
            {/if}
          </AccordionItem>
        {/each}
      </div>
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
      {#if inputData}
        <GeoJSON id="input" data={inputData}>
          <LineLayer
            manageHoverState
            paint={{
              "line-width": 5,
              "line-color": "red",
              "line-opacity": hoverStateFilter(1.0, 0.5),
            }}
            on:click={(e) => activeFeature.set(e.detail.features[0])}
            bind:hovered={$mapHover}
          >
            <Popup openOn="hover" let:features>
              <PropertiesTable properties={features[0].properties} />
            </Popup>
          </LineLayer>
        </GeoJSON>
      {/if}
      {#if comparisonData}
        <GeoJSON id="comparison" data={comparisonData}>
          <LineLayer
            manageHoverState
            paint={{
              "line-width": 5,
              "line-color": "green",
              "line-opacity": hoverStateFilter(1.0, 0.5),
            }}
            layout={comparisonLayer}
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

<style>
  .list {
    border: 1px solid;
    padding: 4px;
    height: 550px;
    overflow-y: scroll;
  }
</style>
