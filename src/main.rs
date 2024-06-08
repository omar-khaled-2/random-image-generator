use std::thread::sleep;

use image::{GenericImage, GenericImageView, ImageBuffer, Rgb};
use rand::Rng;
fn main() {

    let width = 100;
    let height = 100;

    let mut rng = rand::thread_rng();


    let mut imageBuffer = ImageBuffer::new(width, height);

    for i in 0..100{
        for pixel in imageBuffer.pixels_mut() {

        
            let r:u8 = rng.gen();
            let g:u8 = rng.gen();
            let b:u8 = rng.gen();
       
      
    
            *pixel = Rgb([r, g, b]);
        }
    
        imageBuffer.save("image.jpg");
        sleep(std::time::Duration::from_secs(1));
    }


}
