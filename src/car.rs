#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use opencv::{imgcodecs, prelude::*};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

#[derive(Serialize, Debug)]
struct ControlCarRequest {
    speed: f32,
    flip: bool,
}

#[derive(Deserialize, Debug)]
struct ControlCarResponse {
    status: String,
}

pub fn move_car(speed: f32, flip: bool) -> Result<()> {
    // ureq::put("http://hackathon-1-car.local:5000")
    ureq::put("http://192.168.0.212:5000")
        .header("Authorization", "985898")
        .send_json(&ControlCarRequest { speed, flip })?
        .body_mut()
        .read_json::<ControlCarResponse>()?;
    Ok(())
}
