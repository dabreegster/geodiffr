# geodiffr

Wild undescribed experiments for the moment.

## Data

`overpass_example.geojson`: search Overpass for `highway=cycleway` and manually export as GeoJSON

`cid.geojson`:
1.  Download https://cycling.data.tfl.gov.uk/CyclingInfrastructure/data/lines/cycle_lane_track.json
2.  Manually clip to the same area as Overpass
3.  Manually `s/"TRUE"/true/` and `s/"FALSE"/false/`
