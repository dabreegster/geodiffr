<script lang="ts">
  import bbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
  import { JsonView } from "@zerodevx/svelte-json-view";
  import type { Map, MapMouseEvent } from "maplibre-gl";
  import {
    FillLayer,
    GeoJSON,
    MapLibre,
    LineLayer,
    CircleLayer,
  } from "svelte-maplibre";
  import Layout from "./Layout.svelte";
  import DatasetLayers from "./DatasetLayers.svelte";

  let empty = {
    type: "FeatureCollection" as const,
    features: [],
  };
  // TODO Bundle together a type
  let gjA: FeatureCollection = empty;
  let gjB: FeatureCollection = empty;
  let filenameA = "";
  let filenameB = "";
  let opacityA = 0.5;
  let opacityB = 0.5;

  let map: Map;
  let pinnedFeatures: FeatureCollection = empty;

  $: if (map) {
    map.on("click", onClick);
  }

  function onClick(e: MapMouseEvent) {
    pinnedFeatures.features = [];
    // TODO Dedupe by feature ID, since these could repeat across tile boundaries
    for (let rendered of map.queryRenderedFeatures(e.point, {
      layers: [
        "a-points",
        "a-lines",
        "a-polygons",
        "b-points",
        "b-lines",
        "b-polygons",
      ],
    })) {
      let dataset = rendered.layer.id.startsWith("a-") ? gjA : gjB;
      // Find the original feature in the GJ, to avoid having to parse nested properties
      pinnedFeatures.features.push(
        dataset.features.find((f) => f.id == rendered.id)!,
      );
    }
    pinnedFeatures = pinnedFeatures;
  }

  let fileInput: HTMLInputElement;
  async function loadFiles(e: Event) {
    if (fileInput.files?.length != 2) {
      window.alert("Select two GeoJSON files to compare");
      return;
    }

    try {
      gjA = await loadFile(fileInput.files[0], "a");
      filenameA = fileInput.files[0].name;
      gjB = await loadFile(fileInput.files[1], "b");
      filenameB = fileInput.files[1].name;
      pinnedFeatures.features = [];
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  async function loadFile(
    file: File,
    dataset: "a" | "b",
  ): Promise<FeatureCollection> {
    let text = await file.text();
    let gj = JSON.parse(text);

    // Overwrite feature IDs
    // TODO This could mess up some diffing heuristics!
    let id = 1;
    for (let f of gj.features) {
      f.id = id++;
      // Add a new attribute to track later
      f.dataset = dataset;
    }

    return gj;
  }

  function zoomFit() {
    // Just use one dataset, assuming both cover similar extents
    map?.fitBounds(bbox(gjA) as [number, number, number, number], {
      animate: false,
      padding: 10,
    });
  }
</script>

<Layout>
  <div slot="left">
    <h1>GeoDiffr</h1>

    <label>
      Load two .geojson files
      <input bind:this={fileInput} on:change={loadFiles} type="file" multiple />
    </label>

    <div><button on:click={zoomFit}>Zoom to fit</button></div>
    <hr />

    {#if filenameA}
      <div style="background: red"><u><b>A</b>: {filenameA}</u></div>
      <label
        >Opacity:<input
          type="range"
          min="0.0"
          max="1.0"
          step="0.1"
          bind:value={opacityA}
        /></label
      >
      {#each pinnedFeatures.features as f}
        {#if f.dataset == "a"}
          <JsonView json={f.properties} />
        {/if}
      {/each}

      <div style="background: blue"><u><b>B</b>: {filenameB}</u></div>
      <label
        >Opacity:<input
          type="range"
          min="0.0"
          max="1.0"
          step="0.1"
          bind:value={opacityB}
        /></label
      >
      {#each pinnedFeatures.features as f}
        {#if f.dataset == "b"}
          <JsonView json={f.properties} />
        {/if}
      {/each}
    {/if}
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style="https://api.maptiler.com/maps/dataviz/style.json?key=MZEJTanw3WpxRvt7qDfo"
      standardControls
      hash
      bind:map
      on:error={(e) => {
        // @ts-ignore ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      <DatasetLayers gj={gjA} name="a" color="red" opacity={opacityA} />
      <DatasetLayers gj={gjB} name="b" color="blue" opacity={opacityB} />

      <GeoJSON data={pinnedFeatures}>
        <FillLayer
          filter={["==", ["geometry-type"], "Polygon"]}
          paint={{
            "fill-color": "yellow",
          }}
        />
        <LineLayer
          filter={["==", ["geometry-type"], "LineString"]}
          paint={{
            "line-width": 8,
            "line-color": "yellow",
          }}
        />
        <CircleLayer
          filter={["==", ["geometry-type"], "Point"]}
          paint={{
            "circle-radius": 10,
            "circle-color": "yellow",
          }}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
