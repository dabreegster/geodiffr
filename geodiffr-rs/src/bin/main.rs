use geodiffr_rs::{diff_geojson, read_geojson};

fn main() -> anyhow::Result<()> {
    let old_path = std::env::args().nth(1).expect("no old path given");
    let new_path = std::env::args().nth(2).expect("no new path given");

    let old = read_geojson(old_path)?;
    let new = read_geojson(new_path)?;

    let (geojson, _, _) = diff_geojson(&old, &new)?;
    println!("{}", geojson);
    Ok(())
}
