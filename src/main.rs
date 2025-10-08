#![allow(unused_variables)]

use anyhow::Result;

fn main() {
    let q = get_target_quadrant().unwrap();

    println!("{q}");
}

fn get_target_quadrant() -> Result<u8> {
    Ok(ureq::get("http://192.168.0.177:31415/quadrant")
        .header("Authorization", "606545")
        .call()?
        .body_mut()
        .read_to_string()?
        .parse()?)
}
