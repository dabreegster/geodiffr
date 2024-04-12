use std::path::Path;

use geojson::{Feature, FeatureCollection, GeoJson};

pub fn read_geojson<P: AsRef<Path>>(path: P) -> anyhow::Result<GeoJson> {
    let geojson_str = std::fs::read_to_string(path)?;
    Ok(geojson_str.parse::<GeoJson>()?)
}

pub fn diff_geojson(
    geojson1: &GeoJson,
    geojson2: &GeoJson,
) -> anyhow::Result<(GeoJson, usize, usize)> {
    let mut match_count = 0;
    let mut diff_count = 0;
    let mut diff_features: Vec<Feature> = Vec::new();
    match (geojson1, geojson2) {
        (GeoJson::FeatureCollection(old), GeoJson::FeatureCollection(new)) => {
            for feature in &old.features {
                if !new.features.contains(feature) {
                    diff_count += 1;
                    diff_features.push(feature.clone());
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
        assert_eq!(match_count, 1048);
        println!("Diff  count: {diff_count}");
        assert_eq!(diff_count, 131);
    }
}
