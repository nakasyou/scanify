use wasm_bindgen::prelude::*;

use crate::rgba::get_rgba_value;

pub type Edge = (u32, u32);
pub type Edges = [Edge; 4];

pub struct Result {
    pub width: u32,
    pub height: u32,
    pub image: Vec<u8>,
}

/**
 * 2点の距離を計算
 * ピタゴラスに感謝!!!
 */
fn calc_distance(a: Edge, b: Edge) -> f32 {
    (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt()
}

struct RectSize {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
    width: f32,
    height: f32,
}
fn calc_rect_size(edges: Edges) -> RectSize {
    let top = calc_distance(edges[0], edges[1]);
    let right = calc_distance(edges[1], edges[2]);
    let bottom: f32 = calc_distance(edges[2], edges[3]);
    let left: f32 = calc_distance(edges[3], edges[0]);

    return RectSize {
        top,
        right,
        bottom,
        left,
        width: (top + bottom) / 2.,
        height: (right + left) / 2.,
    };
}

/**
 * 線分ABがあるとする
 * AB上にPがあるとき、Aの座標、Bの座標、Ab、APの長さからPの座標を求める
 */
fn calc_point_on_segment(a: (u32, u32), b: (u32, u32), ap: f64) -> (u32, u32) {
    let x1 = a.0 as i64;
    let y1 = a.1 as i64;
    let x2 = b.0 as i64;
    let y2 = b.1 as i64;

    let ab = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
    let ratio = ap / ab;
    let px = (x1 as f64) + ratio * ((x2 - x1) as f64);
    let py: f64 = (y1 as f64) + ratio * ((y2 - y1) as f64);

    return (px.floor() as u32, py.floor() as u32)
}

pub fn scan(width: u32, height: u32, image: Vec<u8>, edges: Edges) -> Result {
    let paper_size = calc_rect_size(edges);

    let (max_paper_height, min_paper_height, is_left_longer) = if paper_size.left > paper_size.right
    {
        (paper_size.left, paper_size.right, true)
    } else {
        (paper_size.right, paper_size.left, false)
    };

    let mut result_image: Vec<u8> = Vec::new();
    for y_position in 0..(paper_size.height.floor() as u32) {
        let y_ratio = y_position as f32 / max_paper_height;

        let (
            left_length, // 左の辺の長さのうち、どこか
            right_length // 右の辺の長さのうち、どこか
        ) = if is_left_longer {
            (y_position, (y_ratio * min_paper_height).floor() as u32)
        } else {
            ((y_ratio * min_paper_height).floor() as u32, y_position)
        };

        let left_position_on_image = calc_point_on_segment(
            edges[0],
            edges[3],
            left_length as f64
        );
        let right_position_on_image = calc_point_on_segment(
            edges[1],
            edges[2],
            right_length as f64
        );

        let width_length_on_image = calc_distance(left_position_on_image, right_position_on_image);
        /*log(&format!("{:?}", (
            width_length_on_image
        )));*/

        for x_position in 0..paper_size.width.floor() as u32 {
            let x_ratio = x_position as f32 / paper_size.width;

            let pixel_position_on_image = calc_point_on_segment(
                left_position_on_image,
                right_position_on_image,
                (x_ratio * width_length_on_image) as f64
            );

            let pixel_data_option: Option<(u8, u8, u8, u8)> = get_rgba_value(&image, width, height, pixel_position_on_image.0, pixel_position_on_image.1);
            if let Some(pixel_data) = pixel_data_option {
                result_image.push(pixel_data.0);
                result_image.push(pixel_data.1);
                result_image.push(pixel_data.2);
                result_image.push(pixel_data.3);
            }
        }
    }

    Result {
        width: paper_size.width.floor() as u32,
        height: paper_size.height.floor() as u32,
        image: result_image,
    }
}
