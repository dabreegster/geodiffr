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
  const colorA: string = colors[0];
  const colorB: string = colors[1];
  const colorC: string = colors[2];
  const colorHighlight: string = "yellow";

  let empty = {
    type: "FeatureCollection" as const,
    features: [],
  };
  // TODO Bundle together a type
  let gjA: FeatureCollection = empty;
  let gjB: FeatureCollection = empty;
  let gjC: FeatureCollection = empty;
  let filenameA = "";
  let filenameB = "";
  let filenameC = "";
  let opacityA = 0.5;
  let opacityB = 0.5;
  let opacityC = 0.5;

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
        "c-points",
        "c-lines",
        "c-polygons",
      ],
    })) {
      let dataset;
      if (rendered.layer.id.startsWith("a-")) {
        dataset = gjA;
      } else if (rendered.layer.id.startsWith("b-")) {
        dataset = gjB;
      } else {
        dataset = gjC;
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
    if (fileInput.files?.length != 3) {
      window.alert("Select two GeoJSON files to compare");
      return;
    }

    try {
      gjA = await loadFile(fileInput.files[0], "a");
      filenameA = fileInput.files[0].name;
      gjB = await loadFile(fileInput.files[1], "b");
      filenameB = fileInput.files[1].name;
      gjC = await loadFile(fileInput.files[2], "c");
      filenameC = fileInput.files[2].name;
      pinnedFeatures.features = [];
    } catch (err) {
      window.alert(`Bad input file: ${err}`);
    }
  }

  async function loadFile(
    file: File,
    dataset: "a" | "b" | "c",
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
      <LayerControl
        filename={filenameA}
        label="a"
        bind:opacity={opacityA}
        {pinnedFeatures}
        color={colorA}
      />
      <LayerControl
        filename={filenameB}
        label="b"
        bind:opacity={opacityB}
        {pinnedFeatures}
        color={colorB}
      />
      <LayerControl
        filename={filenameC}
        label="c"
        bind:opacity={opacityC}
        {pinnedFeatures}
        color="green"
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
      <DatasetLayers gj={gjC} name="c" color={colorC} opacity={opacityC} />

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
