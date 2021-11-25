use crate::{config::Config, element::Element, matching::match_multiples_elements, util::ScreenName};
use enigo::*;
use opencv::{imgcodecs};

pub fn connect_page_control_flow(
    mouse: &mut Enigo,
    screen: &mut ScreenName,
    matched_elements: Vec<&Element>,
    connect_element: &Element,
    metamask_element: &Element,
    metamask_blue_sign_element: &Element,
    config: &Config,
) {

    if matched_elements.contains(&metamask_blue_sign_element) {
        println!("confirming metamask...");
        metamask_blue_sign_element.go_to_location_and_click(mouse, 80, 40, 1);
        println!("metamask confirmed, wait {} seconds for next action", config.after_click_metamask_sign_blue_btn_delay);
        std::thread::sleep(std::time::Duration::from_secs(config.after_click_metamask_sign_blue_btn_delay))
    } else if matched_elements.contains(&metamask_element) {
        println!("starting metamask login.");
        metamask_element.go_to_location_and_click(mouse, 60, 25, 1);
        println!("Waiting for {} seconds for the metamask tab to open...", config.after_click_metamask_connect_delay);
        std::thread::sleep(std::time::Duration::from_secs(config.after_click_metamask_connect_delay))
    } else if matched_elements.len() == 1 && matched_elements.contains(&connect_element) {
        println!("Connect button found. Starting the login...");
        connect_element.go_to_location_and_click(mouse, 100, 20, 1);
        println!("Waiting for {} seconds for the metamask popup to open", config.after_click_connect_orange_btn_delay);
        std::thread::sleep(std::time::Duration::from_secs(config.after_click_connect_orange_btn_delay))
    } else {
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
    go_back_arrow_element: &Element,
    common_text_element: &Element,
    new_map_element: &Element,
    ok_element: &Element,
    config: &Config,
) {

    if matched_elements.contains(&ok_element) {

        println!("Accepting the error and refresh the page");
        ok_element.go_to_location_and_click(mouse, 100, 32, 1);
        std::thread::sleep(std::time::Duration::from_secs(2));
        mouse.key_click(Key::F5);
        println!("waiting for 120 seconds for page reload");
        std::thread::sleep(std::time::Duration::from_secs(120));

    } else if matched_elements.contains(&new_map_element){
        *check_rest = true;
        println!("Confirming new map");
        new_map_element.go_to_location_and_click(mouse, 100, 32, 1);
        println!("waiting for 60 seconds to load a new map");
        std::thread::sleep(std::time::Duration::from_secs(60));

    } else if matched_elements.contains(&hero_element) && matched_elements.contains(&treasure_hunt_element) {
        // inside game menu
        if *check_rest {
        println!("Checking for fresh heroes.");
        hero_element.go_to_location_and_click(mouse, 32, 32, 1);
        println!("Waiting {} seconds to execute the first action in heroes screen...", config.heroes_page_first_action_delay);
        std::thread::sleep(std::time::Duration::from_secs(config.heroes_page_first_action_delay));
        } else {
        println!("Opening treasure hunt screen...");
        treasure_hunt_element.go_to_location_and_click(mouse, 100, 100, 1);
        std::thread::sleep(std::time::Duration::from_secs(config.treasure_hunt_first_action_delay));
     } 
    
    } else if matched_elements.contains(&common_text_element) && *check_rest {
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
                    println!("Sending heroes to work...");
                    able_to_work_heroes[0].go_to_location_and_click(mouse, 20, 20, 1);
                    println!("Hero sent to work, waiting for {} seconds to sent the next one", config.after_sent_to_work_delay);
                    std::thread::sleep(std::time::Duration::from_secs(config.after_sent_to_work_delay));
                 } else if able_to_work_heroes.len() == 1{
                    println!("Sending heroes to work...");
                    able_to_work_heroes[0].go_to_location_and_click(mouse, 20, 20, 1);
                    println!("Hero sent to work, waiting for {} seconds for scroll up and find next one", config.after_sent_to_work_delay);
                    std::thread::sleep(std::time::Duration::from_secs(config.after_sent_to_work_delay));
                    able_to_work_heroes[0].slide_down(mouse, 100);

                 } else if *scann_attempt < 3 {
                    println!("Scrolling up to find next hero able to work: attempt {}", *scann_attempt);
                    let last_common = common_text_element.last().unwrap();
                    last_common.slide_down(mouse, 140);
                    *scann_attempt += 1;
                 } else {
                    println!("Closing Heroes page and returning to the treasure hunt screen");
                    close_heroes_screen_element.go_to_location_and_click(mouse, 0, 0, 1);
                    *check_rest = false;
                 }

    } else if matched_elements.contains(&go_back_arrow_element) {
        *scann_attempt = 0;
        *check_rest = true;
        println!("Watching the heroes work. Waiting for {} seconds to check the heroes who are rested.", config.check_for_heroes_able_to_work_delay);
        std::thread::sleep(std::time::Duration::from_secs(config.check_for_heroes_able_to_work_delay));
        go_back_arrow_element.go_to_location_and_click(mouse, 32, 23, 1);
    } else {
    *screen = ScreenName::Connect;
  }
}