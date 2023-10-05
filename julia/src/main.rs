use image::{ImageBuffer, Rgb};
use nalgebra::{Complex, Normed};
use hsv::hsv_to_rgb;

fn julia(c: Complex<f64>, x: f64, y: f64) -> u8 {
    let mut z = Complex::new(x, y);

    for i in 0..255 {
        if z.norm() > 2.0 {
            return i as u8;
        }

        z = z * z + c;
    }

    255
}

fn main() {
    let width = 800;
    let height = 800;

    let scale_x = 3.0 / width as f64;
    let scale_y = 3.0 / height as f64;

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx = x as f64 * scale_x - 1.5;
        let cy = y as f64 * scale_y - 1.5;

        let c = Complex::new(-0.8, 0.156);
        let value = julia(c, cx, cy);

        let rgb = hsv_to_rgb(value as f64 * 0.2, 1., value as f64 / 255.);

        *pixel = Rgb(rgb.into());
    }

    img.save("julia.png").expect("Failed to save the image");
}
