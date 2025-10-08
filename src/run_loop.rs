#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

use crate::{
    camera::{convert, get_camera_1_vec, get_camera_2_vec},
    car::move_car,
    target::get_target_quadrant,
};

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

fn turn_right() -> Result<()> {
    move_car(0.27, true)
}

fn turn_left() -> Result<()> {
    move_car(-0.27, true)
}

fn drive_forward() -> Result<()> {
    move_car(1.0, false)
}
