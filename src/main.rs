mod detect;

use anyhow::Result;
use opencv::{imgcodecs, prelude::*};

fn main() -> Result<()> {
    let img = imgcodecs::imread("/tmp/img.jpg", imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        anyhow::bail!("image is empty");
    }

    let result = detect::detect_markers(&img)?;
    println!("Detected {} markers: {:?}", result.ids.len(), result.ids);

    // Optional: draw and save an annotated image for quick visual feedback
    // detect::save_with_overlays(&img, &result, "/tmp/annotated.jpg")?;

    Ok(())
}
