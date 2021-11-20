use crate::{element::Element, smooth_movement::smoothly_move_to, util::ScreenName};
use enigo::*;

pub fn connect_page_control_flow(
    mouse: &mut Enigo,
    screen: &mut ScreenName,
    matched_elements: Vec<&Element>,
    connect_element: &Element,
    metamask_element: &Element,
    metamask_blue_sign_element: &Element,
) {

    if matched_elements.contains(&metamask_blue_sign_element) {
        println!("Achou blue sign");
        metamask_blue_sign_element.go_to_location_and_click(mouse, 80, 40, 1);
        std::thread::sleep(std::time::Duration::from_secs(10))
    } else if matched_elements.contains(&metamask_element) {
        println!("Achou Metamask");
        metamask_element.go_to_location_and_click(mouse, 60, 25, 1);
        std::thread::sleep(std::time::Duration::from_secs(20))
    } else if matched_elements.len() == 1 && matched_elements.contains(&connect_element) {
        println!("achou Connect");
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
    matched_elements: Vec<&Element>,
    hero_element: &Element,
    treasure_hunt_element: &Element,
) {

    if matched_elements.contains(&hero_element) && matched_elements.contains(&treasure_hunt_element) {
        if *check_rest {
        hero_element.go_to_location_and_click(mouse, 32, 32, 1);
        } else {
        treasure_hunt_element.go_to_location_and_click(mouse, 100, 100, 1);
     }
    } else {
    *screen = ScreenName::Connect;
}
    
}