use std::path::Path;

use geojson::{Feature, FeatureCollection, GeoJson};
use serde::{Deserialize, Serialize};

pub fn read_geojson<P: AsRef<Path>>(path: P) -> anyhow::Result<GeoJson> {
    let geojson_str = std::fs::read_to_string(path)?;
    Ok(geojson_str.parse::<GeoJson>()?)
}

#[derive(Debug, Serialize, Deserialize)]
/// An enum for the types of difference.
enum DiffType {
    Feature,
    Source,
    Geometry,
    Position,
    Semantic,
}

#[derive(Debug, Serialize, Deserialize)]
/// An enum for whether a difference is an addition or deletion.
enum DiffSign {
    Addition,
    Deletion,
}

const DIFF_KEY: &str = "diff";

#[derive(Debug, Serialize, Deserialize)]
struct Diff {
    r#type: DiffType,
    sign: DiffSign,
}

pub fn diff_geojson(old: &GeoJson, new: &GeoJson) -> anyhow::Result<(GeoJson, usize, usize)> {
    let mut match_count = 0;
    let mut diff_count = 0;
    let mut diff_features: Vec<Feature> = Vec::new();
    match (old, new) {
        (GeoJson::FeatureCollection(old), GeoJson::FeatureCollection(new)) => {
            // Get deletions
            for feature in &old.features {
                if !new.features.contains(feature) {
                    diff_count += 1;
                    let change = Diff {
                        r#type: DiffType::Feature,
                        sign: DiffSign::Deletion,
                    };
                    let mut map = serde_json::Map::new();
                    map.insert(DIFF_KEY.to_string(), serde_json::to_value(&change)?);
                    diff_features.push(Feature {
                        foreign_members: Some(map),
                        ..feature.clone()
                    });
                } else {
                    match_count += 1;
                }
            }
            // Get additions
            for feature in &new.features {
                if !old.features.contains(feature) {
                    diff_count += 1;
                    let change = Diff {
                        r#type: DiffType::Feature,
                        sign: DiffSign::Addition,
                    };
                    let mut map = serde_json::Map::new();
                    map.insert(DIFF_KEY.to_string(), serde_json::to_value(&change)?);
                    diff_features.push(Feature {
                        foreign_members: Some(map),
                        ..feature.clone()
                    });
                } else {
                    match_count += 1;
                }
            }
        }
        _ => todo!(),
    }
    let geojson_out: GeoJson = GeoJson::FeatureCollection(FeatureCollection {
        bbox: None,
        features: diff_features,
        foreign_members: None,
    });

    Ok((geojson_out, match_count, diff_count))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_geojson() {
        read_geojson("data/northgate_blocks/old.geojson").unwrap();
    }

    #[test]
    fn test_compare() {
        let old_geojson = read_geojson("data/san_jose_blocks/old.geojson").unwrap();
        let new_geojson = read_geojson("data/san_jose_blocks/new.json").unwrap();
        let (geojson, match_count, diff_count) = diff_geojson(&old_geojson, &new_geojson).unwrap();
        println!("{}", geojson);
        println!("Match count: {match_count}");
        assert_eq!(match_count, 2096);
        println!("Diff  count: {diff_count}");
        assert_eq!(diff_count, 267);
    }
}
