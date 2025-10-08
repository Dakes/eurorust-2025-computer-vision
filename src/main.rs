#![allow(unused_variables)]

use anyhow::Result;

fn main() {
    let q = get_target_quadrant().unwrap();
    println!("{q}");

    let i1 = get_camera_1_vec().unwrap();
    println!("{:?}", &i1[..10]);

    let i2 = get_camera_2_vec().unwrap();
    println!("{:?}", &i2[..10]);
}

fn get_target_quadrant() -> Result<u8> {
    Ok(ureq::get("http://192.168.0.177:31415/quadrant")
        .header("Authorization", "606545")
        .call()?
        .body_mut()
        .read_to_string()?
        .parse()?)
}

fn get_camera_1_vec() -> Result<Vec<u8>> {
    Ok(ureq::get("http://hackathon-11-camera.local:50051/frame")
        .header("Authorization", "983149")
        .call()?
        .body_mut()
        .read_to_vec()?)
}

fn get_camera_2_vec() -> Result<Vec<u8>> {
    Ok(ureq::get("http://hackathon-12-camera.local:50051/frame")
        .header("Authorization", "378031")
        .call()?
        .body_mut()
        .read_to_vec()?)
}
