#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};
use opencv::core::{Mat, Point2f};
use opencv::imgproc;
use opencv::prelude::*;
use crate::{camera::{convert, get_camera_1_vec, get_camera_2_vec}, car::move_car, detect, target::get_target_quadrant};
use crate::car::turn_car;

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
        let t = 14;//get_target_quadrant().unwrap();
        println!("{t}");

        println!("here -1");
        // calculate move
        let command = 0.5;

        let img1 = &dynamic_image_to_mat_bgr(&x1).unwrap();
        let res1 = detect::detect_markers(img1);


        let img2 = &dynamic_image_to_mat_bgr(&x2).unwrap();
        let res2 = detect::detect_markers(img2);

        let mut tar1 = Vec::new();
        let mut car1= Vec::new();


        for marker in res1.unwrap() {
            println!("Camera 1 detected marker ID: {}", marker.id);
            if (marker.id == t as i32) {
                tar1 = marker.corners;
                continue;
            }

            if (marker.id == 1) {
                car1 = marker.corners;
            }
        }

        if car1.len() == 0 {
            move_car(0.0, false);
            thread::sleep(Duration::from_millis(50));

            continue
        }

        let mut index = 0;
        let mut nearest_point = 0;
        let mut min_dis = 100000.0;

        for p in car1 {
            let new_d = get_distance(p);
            if new_d < min_dis {
                nearest_point = index;
                min_dis = new_d;
            }

            index += 1;
        }

        let flip = nearest_point != 0;

        if (min_dis < 50.0) {
            println!("{}", min_dis);
            println!("stop");
            continue;
        }

        // move car
        if nearest_point == 0 {

            println!("move");
            println!("{}", min_dis);
            move_car(0.5, false);
        } else if nearest_point < 2 {
            println!("move rigth");
            println!("{}", min_dis);

            turn_car(true);
        } else {
            println!("move left");
            println!("{}", min_dis);


            turn_car(false);
        }

        // sleep
        thread::sleep(Duration::from_millis(50));
    }
}

fn get_distance(car: Point2f) -> f32 {
    return (car.x - 512.0).abs() + (car.y - 512.0).abs()
}
