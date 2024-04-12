# geodiffr-rs

A Rust library and CLI for generating a new GeoJSON diff from two input GeoJSON files.


## Example CLI usage
Running:
```bash
cargo run --bin geodiffr-cli -- old.geojson new.geojson
```
outputs a new GeoJSON of diffs to stdout.