use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

mod point;
mod rgba;
mod filters;
mod scan;


#[wasm_bindgen]
pub struct Image {
    data: Uint8ClampedArray,
    width: u32,
    height: u32,
    
    pub result_width: u32,
    pub result_height: u32
}

pub type WASMInputEdges = Vec<u32>;

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, data: Uint8ClampedArray) -> Image {
        Image {
            width,
            height,
            data,
            result_width: 0,
            result_height: 0
        }
    }

    #[wasm_bindgen]
    pub fn scan(&mut self, inputed_edges: WASMInputEdges) -> Vec<u8> {
        let vec_image_data = self.data.to_vec();
        
        let edges: scan::Edges = [
            (inputed_edges[0], inputed_edges[1]),
            (inputed_edges[2], inputed_edges[3]),
            (inputed_edges[4], inputed_edges[5]),
            (inputed_edges[6], inputed_edges[7]),
        ];
        let scaned_result = scan::scan(self.width, self.height, vec_image_data, edges);
        self.result_width = scaned_result.width;
        self.result_height = scaned_result.height;

        return scaned_result.image
    }

    #[wasm_bindgen]
    pub fn pick_edge(&self) -> Vec<u8> {
        let vec_image_data = self.data.to_vec();

        // モノクロにする
        let mut mono_image_data: Vec<Vec<u8>> = Vec::new();
        for x_index in 0..self.width {
            let mut x_vec: Vec<u8> = Vec::new();

            for y_index in 0..self.height {
                let pixel_option = rgba::get_rgba_value(
                    &vec_image_data,
                    self.width,
                    self.height,
                    x_index,
                    y_index,
                );
                if let Some(pixel) = pixel_option {
                    let light = rgba::rgba_light(pixel);
                    x_vec.push(light);
                }
            }
            mono_image_data.push(x_vec);
        }

        // ノイズ除去
        let removed_noise: Vec<Vec<u8>> = filters::apply_rmnoise_filter(&mono_image_data);

        let after_sobel: Vec<Vec<u8>> = filters::apply_sobel_filter(&removed_noise);

        return after_sobel.concat()
    }
}
