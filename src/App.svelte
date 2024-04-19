<script lang="ts">
  import bbox from "@turf/bbox";
  import type { FeatureCollection } from "geojson";
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
  import LayerControl from "./LayerControl.svelte";

  // Vivid from https://carto.com/carto-colors/
  let colors = [
    "#66C5CC",
    "#F6CF71",
    "#F89C74",
    "#DCB0F2",
    "#87C55F",
    "#9EB9F3",
    "#FE88B1",
    "#C9DB74",
    "#8BE0A4",
    "#B497E7",
    "#D3B484",
    "#B3B3B3",
  ];
  const colorA = colors[0];
  const colorB = colors[1];
  const colorDiff = colors[2];
  const colorHighlight = "yellow";

  let empty = {
    type: "FeatureCollection" as const,
    features: [],
  };
  // TODO Bundle together a type
  let gjA: FeatureCollection = empty;
  let gjB: FeatureCollection = empty;
  let gjDiff: FeatureCollection = empty;
  let filenameA = "";
  let filenameB = "";
  let filenameDiff = "";
  let opacityA = 0.5;
  let opacityB = 0.5;
  let opacityDiff = 0.5;

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
        "diff-points",
        "diff-lines",
        "diff-polygons",
      ],
    })) {
      let dataset;
      if (rendered.layer.id.startsWith("a-")) {
        dataset = gjA;
      } else if (rendered.layer.id.startsWith("b-")) {
        dataset = gjB;
      } else {
        dataset = gjDiff;
      }

      // Find the original feature in the GJ, to avoid having to parse nested properties
      pinnedFeatures.features = [
        ...pinnedFeatures.features,
        dataset.features.find((f) => f.id == rendered.id)!,
      ];
    }
  }

  let fileInput: HTMLInputElement;
  async function loadFiles(e: Event) {
    if (!fileInput.files) {
      return;
    }
    let len = fileInput.files.length;
    if (len != 2 && len != 3) {
      window.alert(
        "Select two GeoJSON files to compare, or three (including a diff)",
      );
      return;
    }

    try {
      gjA = await loadFile(fileInput.files[0], "a");
      filenameA = fileInput.files[0].name;
      gjB = await loadFile(fileInput.files[1], "b");
      filenameB = fileInput.files[1].name;
      if (len == 3) {
        gjDiff = await loadFile(fileInput.files[2], "diff");
        filenameDiff = fileInput.files[2].name;
      }
      pinnedFeatures.features = [];
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  async function loadFile(
    file: File,
    dataset: "a" | "b" | "diff",
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
      Load two .geojson files (or three with a diff)
      <input bind:this={fileInput} on:change={loadFiles} type="file" multiple />
    </label>

    <div><button on:click={zoomFit}>Zoom to fit</button></div>
    <hr />

    {#if filenameA}
      <LayerControl
        filename={filenameA}
        name="a"
        bind:opacity={opacityA}
        {pinnedFeatures}
        color={colorA}
      />
      <LayerControl
        filename={filenameB}
        name="b"
        bind:opacity={opacityB}
        {pinnedFeatures}
        color={colorB}
      />
    {/if}
    {#if filenameDiff}
      <LayerControl
        filename={filenameDiff}
        name="diff"
        bind:opacity={opacityDiff}
        {pinnedFeatures}
        color={colorDiff}
      />
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
      <DatasetLayers gj={gjA} name="a" color={colorA} opacity={opacityA} />
      <DatasetLayers gj={gjB} name="b" color={colorB} opacity={opacityB} />
      <DatasetLayers
        gj={gjDiff}
        name="diff"
        color={colorDiff}
        opacity={opacityDiff}
      />

      <GeoJSON data={pinnedFeatures}>
        <FillLayer
          filter={["==", ["geometry-type"], "Polygon"]}
          paint={{
            "fill-color": colorHighlight,
          }}
        />
        <LineLayer
          filter={["==", ["geometry-type"], "LineString"]}
          paint={{
            "line-width": 8,
            "line-color": colorHighlight,
          }}
        />
        <CircleLayer
          filter={["==", ["geometry-type"], "Point"]}
          paint={{
            "circle-radius": 10,
            "circle-color": colorHighlight,
          }}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>
