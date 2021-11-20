use std::{thread::JoinHandle};
use enigo::{Enigo, MouseButton, MouseControllable};
use image::{GrayImage};
use imageproc::template_matching::{self, MatchTemplateMethod};
use crate::element::Element;


pub fn process_on_cpu(check_rest: bool, mouse: &mut Enigo) {
    let target_connect_img = image::open("images-target/connect.png").unwrap().to_luma8();
    let target_metamask_img = image::open("images-target/select-wallet-1-no-hover.png").unwrap().to_luma8();
    let target_metamask_hover_img = image::open("images-target/select-wallet-1-hover.png").unwrap().to_luma8();
    let target_sign_metamask_img = image::open("images-target/sign-btn.png").unwrap().to_luma8();
    let target_hero_icon_img = image::open("images-target/normalized-hero-icon.png").unwrap().to_luma8();
    let target_go_work_img = image::open("images-target/go-work-normalized.png").unwrap().to_luma8();
    let target_green_bar_img = image::open("images-target/green-bar.png").unwrap().to_luma8();
    
    let mut element_list: Vec<Element> = Vec::new();
    let screenshot = image::open("tmp/output.jpg").expect("canno't open screenshot image");
    let gray_screenshot = screenshot.to_luma8();

    let gray_screenshot_for_connect_btn = gray_screenshot.clone();
    let target_connect_img_for_connect_btn = target_connect_img.clone();
    let find_connect_btn: JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding connect btn");        
        let connect_btn_element = find_element_in_multithread(&gray_screenshot_for_connect_btn, &target_connect_img_for_connect_btn, 0.99);
        connect_btn_element
    });

    let gray_screenshot_for_metamask_btn = gray_screenshot.clone();
    let target_metamask_img_for_connect_btn = target_metamask_img.clone();
    let find_metamask_btn:JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding metamask btn");        
        let metamask_btn_element = find_element_in_multithread(&gray_screenshot_for_metamask_btn, &target_metamask_img_for_connect_btn, 0.99);
        metamask_btn_element
    });

    let gray_screenshot_for_metamask_hover_btn = gray_screenshot.clone();
    let target_metamask_hover_img_for_connect_btn = target_metamask_hover_img.clone();
    let find_metamask_hover_btn:JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding metamask hover btn");        
        let metamask_hover_btn_element = find_element_in_multithread(&gray_screenshot_for_metamask_hover_btn, &target_metamask_hover_img_for_connect_btn, 0.99);
        metamask_hover_btn_element
    });

    let gray_screenshot_for_sign_btn = gray_screenshot.clone();
    let target_sign_metamask_img_for_sign_btn = target_sign_metamask_img.clone();
    let find_sign_btn:JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding sign button");        
        let sign_in_btn_element = find_element_in_multithread(&gray_screenshot_for_sign_btn, &target_sign_metamask_img_for_sign_btn, 0.99);
        sign_in_btn_element
    });

    let gray_screenshot_for_hero_icon_btn = gray_screenshot.clone();
    let target_hero_icon_img_for_btn = target_hero_icon_img.clone();
    let find_hero_btn:JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding hero button");        
        let hero_btn_element = find_element_in_multithread(&gray_screenshot_for_hero_icon_btn, &target_hero_icon_img_for_btn, 0.99);
        hero_btn_element
    });

    let gray_screenshot_for_go_work_icon_btn = gray_screenshot.clone();
    let target_go_work_btn = target_go_work_img.clone();
    let find_go_work_btn:JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding hero button");        
        let go_work_btn = find_element_in_multithread(&gray_screenshot_for_go_work_icon_btn, &target_go_work_btn, 0.99);
        go_work_btn
    });

    let gray_screenshot_for_green_bar_btn = gray_screenshot.clone();
    let target_green_bar_btn = target_green_bar_img.clone();
    let find_green_bar_btn:JoinHandle<Element> = std::thread::spawn( move || {
        println!("finding green bar");        
        let green_bar_btn = find_element_in_multithread(&gray_screenshot_for_green_bar_btn, &target_green_bar_btn, 0.99);
        green_bar_btn
    });

    

    let connect_element = find_connect_btn.join().expect("Error in connect_btn trhead");
    let metamask_btn_element = find_metamask_btn.join().expect("Error in find_metamask_btn trhead");
    let metamask_hover_btn_element = find_metamask_hover_btn.join().expect("Error in find_metamask_hover_btn trhead");
    let find_sign_btn_element = find_sign_btn.join().expect("Error in find_sign_btn thread");
    let find_hero_btn_element = find_hero_btn.join().expect("Error in find hero_btn thread");
    let find_go_work_btn_element = find_go_work_btn.join().expect("Error in find go_work_btn thread");
    let find_green_bar_element = find_green_bar_btn.join().expect("Error in find green_bar thread");

    element_list.push(connect_element);
    element_list.push(metamask_btn_element);
    element_list.push(find_sign_btn_element);
    element_list.push(find_hero_btn_element);
    element_list.push(find_go_work_btn_element);
    element_list.push(metamask_hover_btn_element);
    element_list.push(find_green_bar_element);

    let mut matched_elements: Vec<Element> = Vec::new();
    for element in element_list {
        if element.matching_probability >= element.matching_probability_minimal {
            matched_elements.push(element);
        } else {
            println!("no matching for {:?}", element.id);
        }
    }

    check_step_and_go_to_target(
        matched_elements, &connect_element.clone(), 
        &metamask_btn_element.clone(), 
        &metamask_hover_btn_element.clone(),
        &find_sign_btn_element.clone(), 
        &find_hero_btn_element.clone(),
        &find_go_work_btn_element.clone(),
        &find_green_bar_element,
        mouse,
        check_rest,
     ); 

}

fn find_element_in_multithread(screenshot: &GrayImage, template: &GrayImage, matching_probability_minimal: f32) -> Element {
    let result = imageproc::template_matching::match_template(screenshot, template, MatchTemplateMethod::CrossCorrelationNormalized);
    let resultant = template_matching::find_extremes(&result);
    println!("{:?}", resultant);
    let element = Element::new(5, resultant.max_value_location.0, resultant.max_value_location.1, resultant.max_value, matching_probability_minimal);
    element
}


pub fn check_step_and_go_to_target(
    matched_elements: Vec<Element>, 
    connect_element: &Element,
    metamask_element: &Element,
    metamask_hover_element: &Element,
    sign_metamask_element: &Element,
    hero_btn_element: &Element,
    go_work_btn_element: &Element,
    green_bar_btn_element: &Element,
    mouse: &mut Enigo,
    check_rest: bool,

) {


   if matched_elements.contains(sign_metamask_element) {
       println!("contains sign element");
       mouse.mouse_move_to(sign_metamask_element.position_x as i32, sign_metamask_element.position_y as i32);
       mouse.mouse_click(MouseButton::Left);

   } else if matched_elements.contains(metamask_element) {
    println!("contains matamask btn");
       mouse.mouse_move_to(metamask_element.position_x as i32, metamask_element.position_y as i32);
       mouse.mouse_click(MouseButton::Left);
       std::thread::sleep(std::time::Duration::from_secs(10));

   } else if matched_elements.len() == 1 && matched_elements.contains(connect_element) {
    println!("contains connect btn");
     mouse.mouse_move_to(connect_element.position_x as i32 , connect_element.position_y as i32);
    mouse.mouse_click(MouseButton::Left);

   } else if !matched_elements.contains(sign_metamask_element) && !matched_elements.contains(metamask_element) && !matched_elements.contains(connect_element) {
     println!("no contains connect elements");
        if matched_elements.contains(&hero_btn_element) && check_rest {
            println!("contains hero btn element");
            mouse.mouse_move_to(hero_btn_element.position_x as i32, hero_btn_element.position_y as i32);
            mouse.mouse_click(MouseButton::Left);
        }

        if matched_elements.contains(&go_work_btn_element) && check_rest {
            mouse.mouse_move_to(go_work_btn_element.position_x as i32, go_work_btn_element.position_y as i32);
            let total_heroes = 10;
            let screenshot = image::open("tmp/output.png").unwrap().to_luma8();
            let target = image::open("images-target/go-work-normalized.png").unwrap().to_luma8();
            println!("contains go_work_btn");

            std::thread::sleep(std::time::Duration::from_secs(3));

            for iter in 0..=total_heroes {

                if iter == 5 {
                    mouse.mouse_scroll_y(-1000);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
                mouse.mouse_move_to(go_work_btn_element.position_x as i32, go_work_btn_element.position_y as i32);
                let element = find_element_in_multithread(&screenshot, &target,0.97);
                if element.matching_probability >= element.matching_probability_minimal {
                    mouse.mouse_move_to(element.position_x as i32, element.position_y as i32);
                    mouse.mouse_click(MouseButton::Left);
                }

            }
        }
   }



}