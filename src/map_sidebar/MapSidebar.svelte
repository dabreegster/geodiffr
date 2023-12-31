<script lang="ts">
  import turfBbox from "@turf/bbox";
  import type { FeatureCollection, GeoJSON } from "geojson";
  import {
    GeoJSON as GeoJSONSource,
    LineLayer,
    mapContext,
  } from "svelte-maplibre";
  import {
    activeFeature,
    mapHover,
    openFromSidebar,
    sidebarHover,
  } from "./stores";

  // This ties together AccordionItems with objects drawn somewhere on a map,
  // in the following ways:
  //
  // 1) Zero or one objects can be active at a time. activeFeature specifies this.
  // 2) The current active object is drawn with special styling on the map.
  // 3) Clicking empty space on the map deselects the active object.
  // 4) Hovering on something in the sidebar also shows a hover effect in the map.
  // 4) Hovering on something in the map also shows a hover effect in the sidebar (in the form of underlining).
  // 5) Selecting an object from the sidebar zooms to it on the map. (But not when selecting it from the map)
  // 6) Selecting an object from the map scrolls the AccordionItem into view in
  //    the sidebar. (But not when selecting it from the sidebar? TODO)

  const { map } = mapContext();

  $: if ($openFromSidebar) {
    zoomTo($openFromSidebar);
  }

  function zoomTo(feature) {
    $map?.fitBounds(bbox(feature), {
      padding: 20,
      animate: true,
      duration: 500,
    });
  }

  // Suitable for passing to map.fitBounds. Work around https://github.com/Turfjs/turf/issues/1807.
  function bbox(gj: GeoJSON): [number, number, number, number] {
    return turfBbox(gj) as [number, number, number, number];
  }

  function emptyGeojson(): FeatureCollection {
    return {
      type: "FeatureCollection",
      features: [],
    };
  }

  $: hoverGj = $activeFeature || $mapHover || $sidebarHover || emptyGeojson();
</script>

<GeoJSONSource id="hover" data={hoverGj}>
  <LineLayer
    paint={{
      "line-width": 15,
      "line-color": "blue",
      "line-opacity": 0.5,
    }}
  />
</GeoJSONSource>
