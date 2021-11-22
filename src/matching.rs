use enigo::{Enigo};
use crate::{element::Element, smooth_movement::smoothly_move_to, util::ScreenName, flow::*};
use opencv::{imgproc, prelude::*};

pub fn matching_elements(
    check_rest: &mut bool, 
    mouse: &mut Enigo,
    actual_screen: &mut ScreenName,
    total_heroes: i32,
    sent_to_work: &mut i32,
    scan_attempt: &mut i32,
    screenshot: &Mat,
    connect_img: &Mat,
    metamask_no_hover_img: &Mat,
    metamask_blue_sign_img: &Mat,
    hero_img: &Mat,
    treasure_hunt_img: &Mat,
    green_bar_img: &Mat,
    close_heroes_screen_img: &Mat,
    go_back_arrow_img: &Mat,
    common_text_img: &Mat,
    ) {

       match actual_screen {
           ScreenName::Connect => {
            let connect_element =  match_element(screenshot, connect_img, 0.99);
            let metamask_element =  match_element(screenshot, metamask_no_hover_img, 0.99);
            let metamask_blue_sign_element = match_element(screenshot, metamask_blue_sign_img, 0.97);
           
            let elements = vec![
                connect_element,
                metamask_element,
                metamask_blue_sign_element,
            ];

            
            let matched_elements: Vec<&Element> = elements.iter().filter(|x| {
                x.matching_probability > x.matching_probability_minimal
            }).collect();



           connect_page_control_flow(mouse, actual_screen, matched_elements, &connect_element, &metamask_element, &metamask_blue_sign_element)
           },
           _ => {
            println!("********************");
            let hero_element =  match_element(screenshot, hero_img, 0.99);
            let treasure_hunt_element =  match_element(screenshot, treasure_hunt_img, 0.99);
            let green_bar_element = match_element(screenshot, green_bar_img, 0.99);
            let close_heroes_screen_element = match_element(screenshot, close_heroes_screen_img, 0.99);
            let go_back_arrow_element = match_element(screenshot, go_back_arrow_img, 0.99);
            let common_text_element = match_element(screenshot, common_text_img, 0.99);
            let elements = vec![
                hero_element,
                treasure_hunt_element,
                green_bar_element,
                close_heroes_screen_element,
                go_back_arrow_element,
                common_text_element,
            ];
            let matched_elements: Vec<&Element> = elements.iter().filter(|x| {
                x.matching_probability > x.matching_probability_minimal
            }).collect();
            game_page_control_flow(
                check_rest, 
                mouse, 
                actual_screen, 
                total_heroes, 
                sent_to_work,
                scan_attempt,
                matched_elements, 
                &hero_element, 
                &treasure_hunt_element, 
                &green_bar_element, 
                &close_heroes_screen_element,
                &go_back_arrow_element,
                &common_text_element
            );
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

pub fn match_multiples_elements(screenshot: &Mat, template: &Mat, threshold: f32) -> Vec<Element> {
    let mut match_result = Mat::default();
    imgproc::match_template(&screenshot, &template, &mut match_result, 3, &Mat::default()).expect("error at match multiples items");
    let match_result_buffer: Vec<Vec<f32>> = match_result.to_vec_2d().expect("Error at creating multiple matching result buffer");
    let elements: Vec<Element> = match_result_buffer.iter().flatten().enumerate().filter( |x | {
        *x.1 > 0.99
    }).map( |y | {
        let px_max = y.0 as i32 % match_result.mat_size()[1];
        let py_max = y.0 as i32 / match_result.mat_size()[1];
        Element::new(1, px_max, py_max, *y.1, threshold)
    }).collect();

    let mut normalized_elements: Vec<Element> = Vec::new();

    for x in 0..elements.len() {
        if x < elements.len() - 1 && elements[x+1].position_y - elements[x].position_y < 5 {
            normalized_elements.push(elements[x]);
        }
    }
    normalized_elements
}

