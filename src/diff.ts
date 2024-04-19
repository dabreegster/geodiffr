import type { Feature, FeatureCollection } from "geojson";

// Modifies the list of features in-place.
// TODO Replace with a Rust version. This is slow, brute-force, and just temporary
export function removeIdenticalFeatures(
  gjA: FeatureCollection,
  gjB: FeatureCollection,
) {
  let setA = new Set(gjA.features.map(toKey));
  let setB = new Set(gjB.features.map(toKey));

  // TODO Set intersection isn't in Firefox yet
  let duplicates = new Set();
  for (let x of setA) {
    if (setB.has(x)) {
      duplicates.add(x);
    }
  }

  gjA.features = gjA.features.filter((f) => !duplicates.has(toKey(f)));
  gjB.features = gjB.features.filter((f) => !duplicates.has(toKey(f)));
}

// JS doesn't have great options for deep object equality
function toKey(f: Feature): string {
  let copy = JSON.parse(JSON.stringify(f));
  // This will always differ
  delete copy.dataset;
  // Order of features doesn't matter
  delete copy.id;
  return JSON.stringify(copy);
}
