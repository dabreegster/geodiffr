import type { GeoJSON } from "geojson";
import turfBbox from "@turf/bbox";

// Suitable for passing to map.fitBounds. Work around https://github.com/Turfjs/turf/issues/1807.
export function bbox(gj: GeoJSON): [number, number, number, number] {
  return turfBbox(gj) as [number, number, number, number];
}
