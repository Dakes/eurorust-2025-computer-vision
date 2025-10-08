use opencv::{
    aruco,
    core::{Mat, Scalar, Vector},
    prelude::*,
    imgcodecs,
};

pub struct Detection {
    pub ids: Vec<i32>,
    pub corners: Vector<Vector<opencv::core::Point2f>>,
    pub rejected: Vector<Vector<opencv::core::Point2f>>,
}

pub fn detect_markers(img: &Mat) -> opencv::Result<Detection> {
    let dict = aruco::get_predefined_dictionary(
        aruco::PREDEFINED_DICTIONARY_NAME::DICT_4X4_50
    )?;
    let params = aruco::DetectorParameters::create()?;

    let camera_matrix = Mat::default();
    let dist_coeffs = Mat::default();

    let mut corners = Vector::new();
    let mut ids_mat = Mat::default();
    let mut rejected = Vector::new();

    aruco::detect_markers(img, &dict, &mut corners, &mut ids_mat, &params, &mut rejected, &camera_matrix, &dist_coeffs)?;

    let ids = mat_ids_to_vec(&ids_mat)?;
    Ok(Detection { ids, corners, rejected })
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
pub fn save_with_overlays(img: &Mat, det: &Detection, out_path: &str) -> opencv::Result<()> {
    let mut vis = img.clone();
    aruco::draw_detected_markers(&mut vis, &det.corners, &Mat::default(), Scalar::all(0.0))?;
    imgcodecs::imwrite(out_path, &vis, &Vector::<i32>::new())?;
    Ok(())
}