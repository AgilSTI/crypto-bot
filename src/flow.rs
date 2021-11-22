use crate::{element::Element, matching::match_multiples_elements, util::ScreenName};
use enigo::*;
use opencv::{imgcodecs};

pub fn connect_page_control_flow(
    mouse: &mut Enigo,
    screen: &mut ScreenName,
    matched_elements: Vec<&Element>,
    connect_element: &Element,
    metamask_element: &Element,
    metamask_blue_sign_element: &Element,
) {

    if matched_elements.contains(&metamask_blue_sign_element) {
        metamask_blue_sign_element.go_to_location_and_click(mouse, 80, 40, 1);
        std::thread::sleep(std::time::Duration::from_secs(10))
    } else if matched_elements.contains(&metamask_element) {
        metamask_element.go_to_location_and_click(mouse, 60, 25, 1);
        std::thread::sleep(std::time::Duration::from_secs(20))
    } else if matched_elements.len() == 1 && matched_elements.contains(&connect_element) {
        connect_element.go_to_location_and_click(mouse, 100, 20, 1);
        std::thread::sleep(std::time::Duration::from_secs(2))
    } else {
        println!("Achou nada, procurando elementos do game");

        *screen = ScreenName::Game;
    }
    
}

pub fn game_page_control_flow(
    check_rest: &mut bool,
    mouse: &mut Enigo,
    screen: &mut ScreenName,
    total_heroes: i32,
    sent_to_work: &mut i32,
    scann_attempt: &mut i32,
    matched_elements: Vec<&Element>,
    hero_element: &Element,
    treasure_hunt_element: &Element,
    green_bar_element: &Element,
    close_heroes_screen_element: &Element,
) {

    if matched_elements.contains(&hero_element) && matched_elements.contains(&treasure_hunt_element) {
        // inside game menu
        if *check_rest {
        hero_element.go_to_location_and_click(mouse, 32, 32, 1);
        } else {
        treasure_hunt_element.go_to_location_and_click(mouse, 100, 100, 1);
     }

    }  else if matched_elements.contains(&green_bar_element) && *check_rest {
        // inside hero screen
        

            
            let green_bar_img = imgcodecs::imread("images-target/green-bar.png", 0).expect("Couldn't find green bar image");
            let go_work_img = imgcodecs::imread("images-target/go-work.png", 0).expect("Couldn't find green bar image");
            let common_img = imgcodecs::imread("images-target/common-text.png", 0).expect("Couldn't find connect image");
            let screenshot = imgcodecs::imread("tmp/output.png", 0).expect("Couldn't find connect image");
            
            let mut able_to_work_heroes: Vec<Element> = Vec::new();
            let green_bar_elements = match_multiples_elements(&screenshot, &green_bar_img, 0.99);
            let go_work_elements = match_multiples_elements(&screenshot, &go_work_img, 0.99);
            let common_text_element = match_multiples_elements(&screenshot, &common_img, 0.99);
    
            green_bar_elements.iter().for_each(|x| {
                go_work_elements.iter().for_each(|y| {
                    if y.position_y - x.position_y == -14 {
                            able_to_work_heroes.push(*y);
                        }
                    })
                 });
    
                 if able_to_work_heroes.len() > 1 {
                    able_to_work_heroes[0].go_to_location_and_click(mouse, 20, 20, 1);
                 } else if able_to_work_heroes.len() == 1{
                    able_to_work_heroes[0].go_to_location_and_click(mouse, 20, 20, 1);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    able_to_work_heroes[0].slide_down(mouse, 100);
                 } else if *scann_attempt < 3 {
                    let last_common = common_text_element.last().unwrap();
                    last_common.go_to_location_and_click(mouse, 0, 0, 1);
                    *scann_attempt += 1;
                 } else {
                    close_heroes_screen_element.go_to_location_and_click(mouse, 0, 0, 1);
                    *check_rest = false;
                 }

                

    std::thread::sleep(std::time::Duration::from_secs(5));
    } else {
    *screen = ScreenName::Connect;
  }
}