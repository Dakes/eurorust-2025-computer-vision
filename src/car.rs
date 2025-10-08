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

// 1s full speed 360°
// speed for 180°:  0.458
// speed for 90°:  0.229
// speed for 45°:  0.1145

// speed: -1 - 1
pub fn move_car(speed: f32, flip: bool) -> Result<()> {
    // ureq::put("http://hackathon-1-car.local:5000")
    ureq::put("http://192.168.0.212:5000")
        .header("Authorization", "985898")
        .send_json(&ControlCarRequest { speed, flip })?
        .body_mut()
        .read_json::<ControlCarResponse>()?;
    Ok(())
}

pub fn drive() -> Result<()> {
    move_car(1., false)
}

// turn in clockwise direction
pub fn turn_car(right: bool) -> Result<()> {
    let speed_for_360 = 0.3;
    let speed = speed_for_360 / 1.0;
    if right {
        move_car(speed, true)
    } else {
        move_car(-speed, true)
    }
}