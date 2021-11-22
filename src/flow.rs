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
    total_heroes: i32,
    matched_elements: Vec<&Element>,
    hero_element: &Element,
    treasure_hunt_element: &Element,
    green_bar_element: &Element
) {

    if matched_elements.contains(&hero_element) && matched_elements.contains(&treasure_hunt_element) {
        // inside game menu
        if *check_rest {
        hero_element.go_to_location_and_click(mouse, 32, 32, 1);
        } else {
        treasure_hunt_element.go_to_location_and_click(mouse, 100, 100, 1);
     }

    }  else if matched_elements.contains(&green_bar_element) {
        // inside hero screen
        let green_bar_img = imgcodecs::imread("images-target/green-bar.png", 0).expect("Couldn't find green bar image");
        let go_work_img = imgcodecs::imread("images-target/go-work.png", 0).expect("Couldn't find green bar image");
       

        for x in 0..total_heroes / 5 {
            println!("execução do bloco na tela dos heroes");
            let screenshot = imgcodecs::imread("tmp/output.png", 0).expect("Couldn't find connect image");
            let green_bar_elements = match_multiples_elements(&screenshot, &green_bar_img, 0.99);
            let go_work_elements = match_multiples_elements(&screenshot, &go_work_img, 0.99);
            green_bar_elements.iter().for_each(|x| {
                go_work_elements.iter().for_each(|y| {
                    if y.position_y - x.position_y == -14 {
                        y.go_to_location(mouse, 20, 20, 1);
                    }
                })
             });
     
     
             let last_green_bar = green_bar_elements.last();
     
             match last_green_bar {
                 Some(y) => {
                     y.slide_down(mouse, 40);
                 },
                 _ => {
     
                 }
             }
        }

        std::thread::sleep(std::time::Duration::from_secs(4));

    } else {
    *screen = ScreenName::Connect;
  }
   std::thread::sleep(std::time::Duration::from_secs(10)); 
}