<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import {
    FillLayer,
    GeoJSON,
    LineLayer,
    CircleLayer,
    hoverStateFilter,
  } from "svelte-maplibre";

  export let gj: FeatureCollection;
  export let name: string;
  export let color: string;
  export let opacity: number;
</script>

<GeoJSON data={gj}>
  <FillLayer
    id="{name}-polygons"
    filter={["==", ["geometry-type"], "Polygon"]}
    manageHoverState
    eventsIfTopMost
    hoverCursor="pointer"
    paint={{
      "fill-color": color,
      "fill-opacity": hoverStateFilter(opacity, 1.0),
    }}
    layout={{
      visibility: opacity == 0 ? "none" : "visible",
    }}
  />

  <LineLayer
    id="{name}-lines"
    filter={["==", ["geometry-type"], "LineString"]}
    manageHoverState
    eventsIfTopMost
    hoverCursor="pointer"
    paint={{
      "line-width": 8,
      "line-color": color,
      "line-opacity": hoverStateFilter(opacity, 1.0),
    }}
    layout={{
      visibility: opacity == 0 ? "none" : "visible",
    }}
  />

  <CircleLayer
    id="{name}-points"
    filter={["==", ["geometry-type"], "Point"]}
    manageHoverState
    eventsIfTopMost
    hoverCursor="pointer"
    paint={{
      "circle-radius": 10,
      "circle-color": color,
      "circle-opacity": hoverStateFilter(opacity, 1.0),
    }}
    layout={{
      visibility: opacity == 0 ? "none" : "visible",
    }}
  />
</GeoJSON>
