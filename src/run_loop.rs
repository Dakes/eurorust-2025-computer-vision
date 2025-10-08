#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};
use opencv::core::Mat;
use opencv::imgproc;
use opencv::prelude::*;
use crate::{camera::{convert, get_camera_1_vec, get_camera_2_vec}, car::move_car, detect, target::get_target_quadrant};

pub fn dynamic_image_to_mat_bgr(img: &DynamicImage) -> opencv::Result<Mat> {
    // Convert to RGB8 format
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    // Create Mat from RGB data using from_slice
    let rgb_data = rgb_img.as_raw();
    let temp_mat = Mat::from_slice(rgb_data)?;
    let rgb_mat = temp_mat.reshape(3, height as i32)?.try_clone()?;

    // Convert RGB to BGR (OpenCV's default format)
    let mut bgr_mat = Mat::default();
    imgproc::cvt_color(&rgb_mat, &mut bgr_mat, imgproc::COLOR_RGB2BGR, 0)?;

    Ok(bgr_mat)
}

pub fn run_loop() {
    loop {

        println!("start of loop");
        // get image data
        let i1 = get_camera_1_vec().unwrap();
        let x1 = convert(i1).unwrap();
        println!("{:?}", &x1.as_bytes()[..10]);

        println!("here -2");
        let i2 = get_camera_2_vec().unwrap();

        println!("here -1.25");
        let x2 = convert(i2).unwrap();
        println!("{:?}", &x2.as_bytes()[..10]);

        println!("here -1.5");
        // get target
        let t = get_target_quadrant().unwrap();
        println!("{t}");

        println!("here -1");
        // calculate move
        let command = 0.5;

        let img1 = &dynamic_image_to_mat_bgr(&x1).unwrap();
        let res1 = detect::detect_markers(img1);

        let img2 = &dynamic_image_to_mat_bgr(&x2).unwrap();
        let res2 = detect::detect_markers(img2);

        for marker in res1.unwrap() {
            println!("Camera 1 detected marker ID: {}", marker.id);
            if (marker.id == t as i32) {
                println!("Camera 1 sees the target!");
            }

        }

        for marker in res2.unwrap() {
            println!("Camera 2 detected marker ID: {}", marker.id);
            if (marker.id == t as i32) {
                println!("Camera 2 sees the target!");
            }

        }

        // move car
        let c = move_car(command, true).unwrap();
        println!("{c:?}");

        // sleep
        thread::sleep(Duration::from_secs(1));
    }
}
