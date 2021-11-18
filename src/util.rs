use image::{Rgba, RgbaImage};
use arrayfire::{Array, rgb2gray};

  pub fn convert_bgra_to_rgba(bgra_image: RgbaImage) -> RgbaImage {
    let mut new_rgba_image = RgbaImage::new(bgra_image.width(), bgra_image.height());
    for x in 0..bgra_image.width() {
        for y in 0..bgra_image.height() {
          let pixel = bgra_image.get_pixel(x, y);
          new_rgba_image.put_pixel(x, y, Rgba([pixel[2], pixel[1], pixel[0], pixel[3]]) );
        }
      };

      new_rgba_image
}
    
pub fn import_and_convert_array_img_to_grayscale(path: String) -> Array<u8> {
  let color_img: Array<u8> = arrayfire::load_image_native(path.to_string());
   rgb2gray(&color_img, 0.2126, 0.7152, 0.0722)
}