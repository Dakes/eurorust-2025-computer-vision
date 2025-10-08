#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use opencv::{imgcodecs, prelude::*};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

mod detect;
mod camera;
mod car;
mod run_loop;
mod target;

#[derive(Parser)]
#[command(name = "robot")]
#[command(about = "Control robot car", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Detect,
    Loop,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Detect => {
            run_detect();
        }
        Commands::Loop => {
            run_loop::run_loop();
        }
    }
}

fn run_detect() -> Result<()> {
    let img = imgcodecs::imread("/tmp/img.jpg", imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        anyhow::bail!("image is empty");
    }

    let result = detect::detect_markers(&img)?;
    // println!("Detected {} markers: {:?}", result.ids.len(), result.ids);

    // Optional: draw and save an annotated image for quick visual feedback
    // detect::save_with_overlays(&img, &result, "/tmp/annotated.jpg")?;

    Ok(())
}
