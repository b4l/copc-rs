use copc_rs::reader::CopcReader;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() -> laz::Result<()> {
    let lazfn = env::args()
        .nth(1)
        .unwrap_or("tests/data/autzen.laz".to_string());
    let laz_file = BufReader::new(File::open(lazfn)?);
    let mut copc_reader = CopcReader::create(laz_file)?;
    for (i, point) in copc_reader.points(0)?.enumerate().take(5) {
        if i == 0 {
            dbg!(&point);
        }
        println!("Point coordinates: ({}, {}, {})", point.x, point.y, point.z);
    }

    Ok(())
}
