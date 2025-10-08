#![allow(unused_variables, dead_code, unused_imports)]

use anyhow::Result;
use opencv::{
    aruco,
    core::{Mat, Point2f, Scalar, Vector},
    imgcodecs,
    prelude::*,
};

pub struct DetectedObject {
    pub id: i32,
    pub corners: Vec<Point2f>,
}

pub fn detect_markers(img: &Mat) -> Result<Vec<DetectedObject>> {
    let dict = aruco::get_predefined_dictionary(aruco::PREDEFINED_DICTIONARY_NAME::DICT_4X4_50)?;
    let params = aruco::DetectorParameters::create()?;

    let mut corners = Vector::<Vector<Point2f>>::new();
    let mut ids_mat = Mat::default();
    let mut def = Mat::default();
    let mut rejected = Vector::<Vector<Point2f>>::new();

    aruco::detect_markers(
        img,
        &dict,
        &mut corners,
        &mut ids_mat,
        &params,
        &mut rejected,
        &def,
        &def
    )?;

    let ids = mat_ids_to_vec(&ids_mat)?;

    let mut result = Vec::new();
    for (idx, id) in ids.iter().enumerate() {
        let corner_points = corners.get(idx)?;
        let mut points = Vec::new();
        for i in 0..corner_points.len() {
            points.push(corner_points.get(i)?);
        }
        result.push(DetectedObject {
            id: *id,
            corners: points,
        });
    }

    Ok(result)
}

fn mat_ids_to_vec(m: &Mat) -> opencv::Result<Vec<i32>> {
    if m.empty() {
        return Ok(Vec::new());
    }
    let rows = m.rows();
    let mut v = Vec::with_capacity(rows as usize);
    for r in 0..rows {
        v.push(*m.at_2d::<i32>(r, 0)?);
    }
    Ok(v)
}

#[allow(dead_code)]
pub fn save_with_overlays(img: &Mat, corners: &Vector<Vector<Point2f>>, ids: &Mat, out_path: &str) -> opencv::Result<()> {
    let mut vis = img.clone();
    aruco::draw_detected_markers(&mut vis, corners, ids, Scalar::all(0.0))?;
    imgcodecs::imwrite(out_path, &vis, &Vector::<i32>::new())?;
    Ok(())
}