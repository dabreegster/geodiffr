use std::path::Path;

use geojson::GeoJson;

pub fn read_geojson<P: AsRef<Path>>(path: P) -> anyhow::Result<GeoJson> {
    let geojson_str = std::fs::read_to_string(path)?;
    Ok(geojson_str.parse::<GeoJson>()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_geojson() {
        let path = format!(
            "{}/data/northgate_blocks/old.geojson",
            env!("CARGO_MANIFEST_DIR")
        );
        read_geojson(path).unwrap();
    }
}
