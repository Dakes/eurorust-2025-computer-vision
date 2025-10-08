#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

fn main() {
    let q = get_target_quadrant().unwrap();
    println!("{q}");

    let i1 = get_camera_1_vec().unwrap();
    println!("{:?}", &i1[..10]);

    let i2 = get_camera_2_vec().unwrap();
    println!("{:?}", &i2[..10]);

    let c = move_car(0.5, false).unwrap();
    println!("{c:?}");

    let i = get_camera_1_vec().unwrap();
    let x = convert(i).unwrap();
    println!("{:?}", &x.as_bytes()[..10]);
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

#[derive(Serialize, Debug)]
struct ControlCarRequest {
    speed: f32,
    flip: bool,
}

#[derive(Deserialize, Debug)]
struct ControlCarResponse {
    status: String,
}

fn move_car(speed: f32, flip: bool) -> Result<ControlCarResponse> {
    Ok(ureq::put("http://hackathon-1-car.local:5000")
        .header("Authorization", "985898")
        .send_json(&ControlCarRequest { speed, flip })?
        .body_mut()
        .read_json::<ControlCarResponse>()?)
}

fn convert(i: Vec<u8>) -> Result<(DynamicImage)> {
    Ok(ImageReader::new(Cursor::new(i))
        .with_guessed_format()?
        .decode()?)
}
