use image::{ImageBuffer, Rgba};
use std::process::{Child, Command, Stdio};
use std::io::Write;

struct VideoEncoder {
    process: Child,
    stdin: std::process::ChildStdin,
    width: u32,
    height: u32,
}

impl VideoEncoder {
    fn new(width: u32, height: u32, fps: u32, output_path: &str) -> Option<Self> {
        let mut process = Command::new("ffmpeg")
            .args(&[
                "-f", "rawvideo",
                "-pixel_format", "rgba",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &fps.to_string(),
                "-i", "-",
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                "-crf", "23",
                "-preset", "ultrafast",
                "-y",
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;

        let stdin = process.stdin.take()?;
        Some(VideoEncoder { process, stdin, width, height })
    }

    fn write_frame(&mut self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let _ = self.stdin.write_all(img.as_raw());
        let _ = self.stdin.flush();
    }

    fn finish(mut self) {
        drop(self.stdin);
        let _ = self.process.wait();
    }
}

fn main() {
    let width = 1080u32;
    let height = 1920u32;
    let fps = 60u32;
    let duration_secs = 5;

    let mut encoder = VideoEncoder::new(width, height, fps, "output.mp4")
        .expect("Failed to start ffmpeg");

    let total_frames = fps * duration_secs;

    println!("Recording {} frames at {}x{}", total_frames, width, height);

    for frame_num in 0..total_frames {
        let mut img = ImageBuffer::new(width, height);

        // Fill background (black)
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }

        let time = frame_num as f32 / fps as f32;

        // Draw moving circle
        let circle_x = (width as f32 / 2.0 + (time * 2.0).sin() * 200.0) as i32;
        let circle_y = (height as f32 / 2.0 + (time * 1.5).cos() * 200.0) as i32;
        draw_circle(&mut img, circle_x, circle_y, 50, Rgba([255, 0, 0, 255]));

        // Draw rotating rectangle
        draw_rect(&mut img, 400, 800, 200, 100, Rgba([0, 255, 0, 255]));

        encoder.write_frame(&img);

        if frame_num % 60 == 0 {
            println!("Frame {}/{}", frame_num, total_frames);
        }
    }

    encoder.finish();
    println!("Video saved to output.mp4.");
}

fn draw_circle(
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

fn draw_rect(
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