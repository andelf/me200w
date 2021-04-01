use image::imageops;
use image::imageops::FilterType;
use image::{DynamicImage, Pixel, Rgba, RgbaImage};

fn your_200w(origin_img: &DynamicImage) {
    let bg_color = Rgba::from_channels(0x36, 0x39, 0x3d, 0xff);
    let mut img = RgbaImage::from_pixel(640 * 6 + 320, 640 * 4 + 320, bg_color);

    let offset = 4;
    let resized_center_img = origin_img.resize(
        480 - (offset + offset) * 2,
        480 - (offset + offset) * 2,
        FilterType::Gaussian,
    );

    for y in 0..4 {
        for x in 0..6 {
            let mut center_img = resized_center_img.to_rgba8();
            let n = 2.0 + 0.1 * (y * 6 + x) as f64;
            println!("Me, the 200w designer, n={:.1}", n);
            xiaomi_corner(&mut center_img, n);

            imageops::overlay(&mut img, &center_img, 640 * x + 240, 640 * y + 240);
        }
    }

    println!("saving ....");
    img.save("out.200w.png").expect("must save");
    println!("saved in `out.200w.png`");
}

// Note: log_2(8.0) is the feather-radius.
fn xiaomi_corner(img: &mut RgbaImage, n: f64) {
    let a = img.width() as f64 / 2.0;
    let b = img.height() as f64 / 2.0;

    let scale = (a.powf(2.0) + b.powf(2.0)).powf(0.5) as f64;

    img.enumerate_pixels_mut().for_each(|(x, y, p)| {
        let x1 = ((x as f64) - a) / a;
        let y1 = ((y as f64) - b) / b;

        let diff = (x1.abs().powf(n) + y1.abs().powf(n) - 1.0) * scale / 8.0;

        if diff >= 1.0 {
            p[3] = 0; // or use bg_color
        } else if diff >= 0.0 {
            p[3] = 255 - (diff * 255.0) as u8;
        }
    });
}

fn main() {
    let img = image::open("./head.jpg").unwrap();

    your_200w(&img)
}
