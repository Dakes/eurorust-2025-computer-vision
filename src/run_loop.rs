use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

pub fn run_loop() {
    loop {
        // get image data
        let i1 = get_camera_1_vec().unwrap();
        let x1 = convert(i1).unwrap();
        println!("{:?}", &x1.as_bytes()[..10]);

        let i2 = get_camera_2_vec().unwrap();
        let x2 = convert(i2).unwrap();
        println!("{:?}", &x2.as_bytes()[..10]);

        // get target
        let t = get_target_quadrant().unwrap();
        println!("{t}");

        // calculate move
        let command = 0.5;

        // move car
        let c = move_car(command, true).unwrap();
        println!("{c:?}");

        // sleep
        thread::sleep(Duration::from_secs(1));
    }
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
