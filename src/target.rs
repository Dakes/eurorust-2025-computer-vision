use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use opencv::{imgcodecs, prelude::*};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

pub fn get_target_quadrant() -> Result<u8> {
    Ok(ureq::get("http://192.168.0.177:31415/quadrant")
        .header("Authorization", "606545")
        .call()?
        .body_mut()
        .read_to_string()?
        .parse()?)
}
