use std::borrow::Borrow;
use enigo::{Enigo};
use arrayfire::*;
use crate::{element::Element, smooth_movement::smoothly_move_to, util::{self, ScreenName}};


pub fn process_on_gpu(check_rest: bool, mouse: &mut Enigo, actual_scren: &mut ScreenName) {
    
    arrayfire::set_backend(arrayfire::Backend::OPENCL);

    let screenshot_gray = util::import_and_convert_array_img_to_grayscale("tmp/output.jpg".to_string());
    let target_connect_gray = util::import_and_convert_array_img_to_grayscale("images-target/connect.png".to_string());
    let target_metamask = util::import_and_convert_array_img_to_grayscale("images-target/select-wallet-1-no-hover.png".to_string());
    let target_sign_in_metamask = util::import_and_convert_array_img_to_grayscale("images-target/select-wallet-2.jpg".to_string());

    

    match actual_scren {
        ScreenName::Connect => {
            let connect_element = find_element_gpu_in_cpu_multithread(screenshot_gray.borrow(), target_connect_gray, 3.8);
            println!("connect button {:?}", connect_element.matching_probability);
        
            let metamask_element = find_element_gpu_in_cpu_multithread(screenshot_gray.borrow(), target_metamask, 3.8);
            println!("metamask {:?}", metamask_element.matching_probability);
        
            let metamask_sign_in_element = find_element_gpu_in_cpu_multithread(screenshot_gray.borrow(), target_sign_in_metamask, 4.5);
            println!(" metamask_sign_in: {:?}", metamask_sign_in_element.matching_probability);
        
            let elements = vec![
                connect_element,
                metamask_element,
                metamask_sign_in_element
            ];

            let mut matched_elements: Vec<Element> = Vec::new();
            for element in elements {
                if element.matching_probability < element.matching_probability_minimal {
                   matched_elements.push(element);
                } 
    
            }
            connect_screen_procedures(matched_elements, connect_element, metamask_element, metamask_sign_in_element, mouse);
        },
        ScreenName::Game => {

        },
        _ => {

        }
    }

}

fn find_element_gpu_in_cpu_multithread(image: &Array<u8>, template: Array<u8>, threshold: f32) -> Element {
    let match_probability = match_template(&image, &template, MatchType::SAD);
    let (min, _, min_idx) = imin_all(&match_probability);
    let dims = match_probability.dims();
    let [height, _, _, _] = dims.get();
    let px_min = min_idx as u64 / height;
    let py_min = min_idx as u64 % height;

    Element::new(1, px_min as u32 , py_min as u32, min * 0.0001, threshold)
}

fn connect_screen_procedures(
    matched_elements: Vec<Element>,
    connect_element: Element,
    metamask_element: Element,
    metamask_sign_element: Element,
    mouse: &mut Enigo,
) {

    if matched_elements.contains(&metamask_sign_element) {
        smoothly_move_to(mouse, metamask_sign_element.position_x as i32, metamask_sign_element.position_y as i32, 1);
    } else if matched_elements.contains(&metamask_element) {
        smoothly_move_to(mouse, metamask_element.position_x as i32, metamask_element.position_y as i32, 1);
        println!("deu match com metamask");
        std::thread::sleep(std::time::Duration::from_secs(10));
    } else if matched_elements.contains(&connect_element) {
        println!("deu match com connect, {:?}", connect_element.matching_probability);
        smoothly_move_to(mouse, connect_element.position_x as i32, connect_element.position_y as i32, 1);
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

}