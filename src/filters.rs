use std::f64::consts::PI;

const CARNEL_X: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
const CARNEL_Y: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

pub fn apply_sobel_filter_pixel(image_data: &Vec<Vec<u8>>, x: u32, y: u32) -> u8 {
    let mut gradient_x: i32 = 0;
    let mut gradient_y: i32 = 0;

    let width = image_data.len() as isize;
    let height = image_data[0].len() as isize;

    for x_position in 0..3 {
        for y_position in 0..3 {
            let search_x: isize = (x as isize) + x_position - 1;
            let search_y: isize = (y as isize) + y_position - 1;

            if search_x >= 0 && search_y >= 0 && search_x < width && search_y < height {
                let light = image_data[search_x as usize][search_y as usize];
                // X/Y
                gradient_x += (light as i32) * (CARNEL_X[x_position as usize][y_position as usize]);
                gradient_y += (light as i32) * (CARNEL_Y[x_position as usize][y_position as usize]);
            }
        }
    }

    ((gradient_x.pow(2) + gradient_y.pow(2)) as f32)
        .sqrt()
        .floor() as u8
}

pub fn apply_sobel_filter(imagedata: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let width = imagedata.len() as u32;
    let height = imagedata[0].len() as u32;

    let mut result: Vec<Vec<u8>> = Vec::new();
    for x in 0..width {
        let mut x_result: Vec<u8> = Vec::new();
        for y in 0..height {
            x_result.push(apply_sobel_filter_pixel(imagedata, x, y));
        }
        result.push(x_result)
    }
    result
}

fn gaussian(x: f64, mean: f64, sigma: f64) -> f64 {
    let exponent = -((x - mean) * (x - mean)) / (2.0 * sigma * sigma);
    (1.0 / (sigma * (2.0 * PI).sqrt())) * exponent.exp()
}

pub fn apply_rmnoise_filter(image: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let width: i32 = image.len() as i32;
    let height: i32 = image[0].len() as i32;

    let mut result: Vec<Vec<u8>> = Vec::new();
    for x_position in 0..width {
        let mut x_result: Vec<u8> = Vec::new();
        for y_position in 0..height {
            let mut this_pixel_result: u32 = 0;

            let mut i: u8 = 0;
            for x_bias in -2..3 {
                for y_bias in -2..3 {
                    let x = x_position + x_bias;
                    let y = y_position + y_bias;

                    if x >= 0 && y >= 0 && x < width && y < height {
                        let this_pixel = image[x as usize][y as usize] as u32;
                        let distance = (x_bias as i32).pow(2) + (x_bias as i32).pow(2);
                        let weight = gaussian((distance as f64) / 2.0, 0.0, 0.4);
                        this_pixel_result += (weight * (this_pixel as f64)).floor() as u32;
                        i += 1;
                    }
                }
            }
            x_result.push((this_pixel_result / (i as u32)) as u8);
        }
        result.push(x_result)
    }
    result
}
