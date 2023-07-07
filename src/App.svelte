<script lang="ts">
  import type { Feature, FeatureCollection } from "geojson";
  import layers from "protomaps-themes-base";
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
  import { fixComparisonData } from "./cid";
  import Endcaps from "./Endcaps.svelte";
  import Layout from "./Layout.svelte";
  import AccordionItem from "./map_sidebar/AccordionItem.svelte";
  import MapSidebar from "./map_sidebar/MapSidebar.svelte";
  import { activeFeature, mapHover } from "./map_sidebar/stores";
  import PmTiles from "./PmTiles.svelte";
  import PropertiesTable from "./PropertiesTable.svelte";
  import { auditData } from "./shared";

  let basemapStyle = {
    version: 8,
    // TODO Also host locally for offline
    glyphs: "https://cdn.protomaps.com/fonts/pbf/{fontstack}/{range}.pbf",
    sources: {
      protomaps: {
        type: "vector",
        url: "pmtiles:///geodiffr/southbank.pmtiles",
      },
    },
    layers: layers("protomaps", "light"),
  };

  let inputData: FeatureCollection;
  let comparisonData: FeatureCollection;
  let showComparison = true;

  $: comparisonLayer = {
    visibility: showComparison ? "visible" : "none",
  };

  onMount(async () => {
    try {
      let resp1 = await fetch("/geodiffr/overpass_example.geojson");
      inputData = fixIDs(fixInputData(await resp1.json()));

      let resp2 = await fetch("/geodiffr/cid.geojson");
      comparisonData = fixIDs(fixComparisonData(await resp2.json()));
    } catch (err) {
      window.alert(`Input error: ${err}`);
    }
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

  // Assign our own IDs, excluding 0
  function fixIDs(gj: FeatureCollection) {
    var id = 1;
    for (let f of gj.features) {
      f.id = id++;
    }
    return gj;
  }

  function openPhotos(f: Feature) {
    if (f.properties.PHOTO1_URL) {
      window.open(f.properties.PHOTO1_URL, "_blank");
    }
    if (f.properties.PHOTO2_URL) {
      window.open(f.properties.PHOTO2_URL, "_blank");
    }
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
    <PmTiles />
    <MapLibre
      style={basemapStyle}
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
              "line-width": 10,
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
        <Endcaps source="input" lineStrings={inputData} />
      {/if}
      {#if comparisonData}
        <GeoJSON id="comparison" data={comparisonData}>
          <LineLayer
            manageHoverState
            paint={{
              "line-width": 10,
              "line-color": "green",
              "line-opacity": hoverStateFilter(1.0, 0.5),
            }}
            layout={comparisonLayer}
            on:click={(e) => openPhotos(e.detail.features[0])}
          >
            <Popup openOn="hover" let:features>
              <PropertiesTable properties={features[0].properties} />
            </Popup>
          </LineLayer>
        </GeoJSON>
        <Endcaps
          source="comparison"
          lineStrings={comparisonData}
          layout={comparisonLayer}
        />
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
