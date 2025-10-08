#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use clap::{Parser, Subcommand, command};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, thread, time::Duration};

mod run_loop;

#[derive(Parser)]
#[command(name = "robot")]
#[command(about = "Control robot car", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Loop,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Loop => {
            run_loop::run_loop();
        }
    }
}
