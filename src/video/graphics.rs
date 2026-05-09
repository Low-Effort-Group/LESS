use image::{ImageBuffer, Rgba};

pub fn draw_circle(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Rgba<u8>,
) {
    let r_sq = radius * radius;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= r_sq {
                let x = (cx + dx) as u32;
                let y = (cy + dy) as u32;
                if x < img.width() && y < img.height() {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
}

pub fn draw_rect(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: Rgba<u8>,
) {
    for py in y..=(y + h).min(img.height() - 1) {
        for px in x..=(x + w).min(img.width() - 1) {
            img.put_pixel(px, py, color);
        }
    }
}
