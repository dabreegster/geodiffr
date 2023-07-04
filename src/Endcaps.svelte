<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { CircleLayer, GeoJSON } from "svelte-maplibre";

  export let source: string;
  export let lineStrings: FeatureCollection;
  export let layout: maplibregl.CircleLayerSpecification["layout"] | undefined =
    undefined;

  let id = `${source}-endpoints`;
  $: data = calculate(lineStrings);

  function calculate(input: FeatureCollection): FeatureCollection {
    let output = {
      type: "FeatureCollection",
      features: [],
    };
    for (let f of input.features) {
      if (f.geometry.type == "LineString") {
        for (let pt of [
          f.geometry.coordinates[0],
          f.geometry.coordinates[f.geometry.coordinates.length - 1],
        ]) {
          output.features.push({
            type: "Feature",
            properties: {},
            geometry: {
              type: "Point",
              coordinates: pt,
            },
          });
        }
      }
    }
    return output;
  }
</script>

<GeoJSON {id} {data}>
  <CircleLayer
    paint={{
      "circle-radius": 5,
      "circle-opacity": 0,
      "circle-stroke-color": "black",
      "circle-stroke-width": 2.0,
    }}
    {layout}
  />
</GeoJSON>
