use image::{self, RgbaImage};
use enigo::{self, Enigo};
use scrap::{self, Capturer, Display};
use template::{matching::{self, matching_elements}, util::{*, self}};
use std::{borrow::Borrow, error::Error, thread::{self}};
use std::time::Duration;
use std::io::ErrorKind::WouldBlock;
use opencv::{imgcodecs, prelude::*};


fn main() -> Result<(), Box<dyn Error>> {

    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    //importing target element assets and covert to OpenCV elements
    let target_connect_img = imgcodecs::imread("images-target/connect.png", 0).expect("Couldn't find connect image");
    let metamask_connect_img = imgcodecs::imread("images-target/select-wallet-1-no-hover.png", 0).expect("Couldn't find connect image");
    let metamask_blue_sign_img =  imgcodecs::imread("images-target/select-wallet-2.png", 0).expect("Couldn't find connect image");
    let hero_img = imgcodecs::imread("images-target/hero-icon.png", 0).expect("Couldn't find connect image");
    let treasure_hunt_img = imgcodecs::imread("images-target/treasure-hunt-icon.png", 0).expect("Couldn't find treasure hunt image");
    let green_bar_img = imgcodecs::imread("images-target/green-bar.png", 0).expect("Couldn't find green bar image");

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    let mut check_rest = true;
    let mut mouse = Enigo::new();
    let mut actual_screen = ScreenName::Connect;
    thread::sleep(std::time::Duration::from_secs(3));


    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => {
                buffer
            },
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        let new_buff=  buffer.to_vec();
        let bgra_image = RgbaImage::from_vec(w as u32, h as u32, new_buff).unwrap();
        let normalized_rgba_image = convert_bgra_to_rgba(bgra_image);
        normalized_rgba_image.save_with_format("tmp/output.png", image::ImageFormat::Png).unwrap();
        let screenshot = imgcodecs::imread("tmp/output.png", 0).expect("Couldn't find connect image");
        
        matching_elements(
            &mut check_rest, 
            &mut mouse, 
            &mut actual_screen,
            screenshot.borrow(),
            target_connect_img.borrow(),
            metamask_connect_img.borrow(),
            metamask_blue_sign_img.borrow(),
            hero_img.borrow(),
            treasure_hunt_img.borrow(),
            &green_bar_img.borrow(),
        );
   }

   Ok(())
}