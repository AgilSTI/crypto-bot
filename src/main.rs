use image::{self, RgbaImage};
use enigo::{self, Enigo};
use scrap::{self, Capturer, Display};
use template::{util::*, cpu::*, gpu::*};
use std::{error::Error, thread::{self}};
use std::time::Duration;
use std::io::ErrorKind::WouldBlock;

fn main() -> Result<(), Box<dyn Error>> {

    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    let check_rest = true;
    let actual_page = "connect";
    let use_gpu = false;
    let mut mouse = Enigo::new();

    loop {
        thread::sleep(Duration::from_secs(2));
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
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
        normalized_rgba_image.save_with_format("tmp/output.jpg", image::ImageFormat::Jpeg).unwrap();
      
    match use_gpu {
        false => {
            process_on_gpu(check_rest, &mut mouse);
            break;
        },
        _  => {
            process_on_cpu(check_rest, &mut mouse);
        }
    }

   }

   Ok(())
}