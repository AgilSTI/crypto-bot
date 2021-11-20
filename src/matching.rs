use enigo::{Enigo};
use rayon::iter::IntoParallelRefIterator;
use crate::{element::Element, smooth_movement::smoothly_move_to, util::ScreenName};
use opencv::{imgproc, prelude::*};

pub fn matching_elements(
    check_rest: bool, 
    mouse: &mut Enigo,
    actual_screen: &mut ScreenName,
    screenshot: &Mat,
    connect_img: &Mat,
    metamask_no_hover_img: &Mat,
    metamask_blue_sign_img: &Mat,
    ) {


       match actual_screen {
           ScreenName::Connect => {
            let connect_element =  match_element(screenshot, connect_img, 0.99);
            let metamask_element =  match_element(screenshot, metamask_no_hover_img, 0.99);
            let metamask_blue_sign_element = match_element(screenshot, metamask_blue_sign_img, 0.99);
            let elements = vec![
                connect_element,
                metamask_element,
                metamask_blue_sign_element,
            ];

            
            let matched_elements: Vec<&Element> = elements.iter().filter(|x| {
                x.matching_probability > x.matching_probability_minimal
            }).collect();

            println!("{:?}", matched_elements);

            // smoothly_move_to(mouse, connect_element.position_x, connect_element.position_y, 1);
           },
           _ => {

           }
       }
     

}


fn match_element(screenshot: &Mat, template: &Mat, threshold: f32) -> Element {
    let mut match_result = Mat::default();
    imgproc::match_template(screenshot, template, &mut match_result, 3, &Mat::default()).expect("error at match template");
    let match_result_buffer: Vec<Vec<f32>> = match_result.to_vec_2d().expect("Error at creating matching result buffer");
    let mut max_value: f32 = 0.0;
    let mut max_value_index = 0;
    match_result_buffer.iter().flatten().enumerate().for_each(|x| {
        if *x.1 > max_value {
            max_value = *x.1;
            max_value_index = x.0;
        };
    });

    let px_max = max_value_index as i32 % match_result.mat_size()[1];
    let py_max = max_value_index as i32 / match_result.mat_size()[1];

    Element::new(1, px_max , py_max, max_value, threshold)
    
}
