import type { Feature } from "geojson";
import { writable, type Writable } from "svelte/store";

// An optional feature that's open on the sidebar.
export const activeFeature: Writable<Feature | null> = writable(null);

// An optional feature currently hovered from the map or sidebar.
export const mapHover: Writable<Feature | null> = writable(null);
export const sidebarHover: Writable<Feature | null> = writable(null);

// Like an event dispatcher, but easier to plumb around.
// TODO Reconsider if there's one big component we end up using here
export const openFromSidebar: Writable<Feature | null> = writable(null);
