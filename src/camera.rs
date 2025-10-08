#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

pub fn get_camera_1_vec() -> Result<Vec<u8>> {
    // Ok(ureq::get("http://hackathon-11-camera.local:50051/frame")
   Ok(ureq::get("http://192.168.0.118:50051/frame")
        .header("Authorization", "983149")
        .call()?
        .body_mut()
        .read_to_vec()?)
}

pub fn get_camera_2_vec() -> Result<Vec<u8>> {
    // Ok(ureq::get("http://hackathon-12-camera.local:50051/frame")
   Ok(ureq::get("http://192.168.0.107:50051/frame")
        .header("Authorization", "378031")
        .call()?
        .body_mut()
        .read_to_vec()?)
}

pub fn convert(i: Vec<u8>) -> Result<DynamicImage> {
    Ok(ImageReader::new(Cursor::new(i))
        .with_guessed_format()?
        .decode()?)
}
