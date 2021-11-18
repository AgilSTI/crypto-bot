use enigo::{Enigo};
use arrayfire::*;

use crate::{element::Element, smooth_movement::smoothly_move_to, util};


pub fn process_on_gpu(check_rest: bool, mouse: &mut Enigo) {
    
    arrayfire::set_backend(arrayfire::Backend::OPENCL);

    let screenshot_gray = util::import_and_convert_array_img_to_grayscale("tmp/output.jpg".to_string());
    let target_connect_gray = util::import_and_convert_array_img_to_grayscale("images-target/connect.png".to_string());
    
    let connect_element = find_element_gpu_in_cpu_multithread(screenshot_gray, target_connect_gray);
    println!("{:?}", connect_element.matching_probability);

    smoothly_move_to(mouse, connect_element.position_x as i32,connect_element.position_y as i32, 2);

}

fn find_element_gpu_in_cpu_multithread(image: Array<u8>, template: Array<u8>) -> Element {
    let match_probability = match_template(&image, &template, MatchType::SAD);
    let (min, _, min_idx) = imin_all(&match_probability);
    let dims = match_probability.dims();
    let [height, _, _, _] = dims.get();
    let px_min = min_idx as u64 / height;
    let py_min = min_idx as u64 % height;

    Element::new(1, px_min as u32 , py_min as u32, min, 0.2)
}