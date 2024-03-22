pub fn get_rgba_value(
    image_data: &Vec<u8>,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
) -> Option<(u8, u8, u8, u8)> {
    if x < width && y < height {
        let pixel_index = ((y * width + x) * 4) as usize;
        if pixel_index + 3 < image_data.len() {
            let r = image_data[pixel_index];
            let g = image_data[pixel_index + 1];
            let b = image_data[pixel_index + 2];
            let a = image_data[pixel_index + 3];
            Some((r, g, b, a))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn rgba_distance(a: (u8, u8, u8, u8), b: (u8, u8, u8, u8)) -> f32 {
    (((a.0 - b.0).pow(2) +
    (a.1 - b.1).pow(2) +
    (a.2 - b.2).pow(2) +
    (a.3 - b.3).pow(2)) as f32).sqrt()
}

pub fn rgba_light (pixel: (u8, u8, u8, u8)) -> u8 {
    (pixel.0 + pixel.1 + pixel.2) / 3
}
pub fn rgba_light_distance(a: (u8, u8, u8, u8), b: (u8, u8, u8, u8)) -> f32 {
    let a_light = ((a.0 + a.1 + a.2) as f32) / 3.0;
    let b_light = ((b.0 + b.1 + b.2) as f32) / 3.0;
    
    return (a_light - b_light).abs()
}

