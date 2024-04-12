<script lang="ts">
  import bbox from "@turf/bbox";
  import type { Feature, FeatureCollection } from "geojson";
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
  let gjA: FeatureCollection = empty;
  let gjB: FeatureCollection = empty;

  let map: Map;
  let pinnedFeature: Feature | null = null;

  $: if (map) {
    map.on("click", onClick);
  }

  function onClick(e: MapMouseEvent) {
    // TODO Rethink this
    pinnedFeature = null;
    for (let rendered of map.queryRenderedFeatures(e.point, {
      layers: ["points", "lines", "polygons"],
    })) {
      // Find the original feature in the GJ, to avoid having to parse nested properties
      pinnedFeature = gjA.features.find((f) => f.id == rendered.id)!;
      break;
    }
  }

  let fileInput: HTMLInputElement;
  async function loadFiles(e: Event) {
    if (fileInput.files?.length != 2) {
      window.alert("Select two GeoJSON files to compare");
      return;
    }

    try {
      gjA = await loadFile(fileInput.files![0]);
      gjB = await loadFile(fileInput.files![1]);
      pinnedFeature = null;
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  async function loadFile(file: File): Promise<FeatureCollection> {
    let text = await file.text();
    let gj = JSON.parse(text);

    // Overwrite feature IDs
    // TODO This could mess up some diffing heuristics!
    let id = 1;
    for (let f of gj.features) {
      f.id = id++;
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
      Load a .geojson file
      <input bind:this={fileInput} on:change={loadFiles} type="file" multiple />
    </label>

    <div><button on:click={zoomFit}>Zoom to fit</button></div>
    <hr />

    {#if pinnedFeature}
      <JsonView json={pinnedFeature.properties} />
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
      <DatasetLayers gj={gjA} name="a" color="red" />
      <DatasetLayers gj={gjB} name="b" color="blue" />

      <GeoJSON data={pinnedFeature || empty}>
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
