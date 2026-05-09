use image::{ImageBuffer, Rgba};
use std::io::Write;
use std::process::{Child, Command, Stdio};

pub struct VideoEncoder {
    process: Child,
    stdin: std::process::ChildStdin,
}

impl VideoEncoder {
    pub fn new(width: u32, height: u32, fps: u32, output_path: &str) -> Option<Self> {
        let mut process = Command::new("ffmpeg")
            .args([
                "-f",
                "rawvideo",
                "-pixel_format",
                "rgba",
                "-video_size",
                &format!("{}x{}", width, height),
                "-framerate",
                &fps.to_string(),
                "-i",
                "-",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-crf",
                "23",
                "-preset",
                "ultrafast",
                "-y",
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;

        let stdin = process.stdin.take()?;
        Some(VideoEncoder { process, stdin })
    }

    pub fn write_frame(&mut self, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let _ = self.stdin.write_all(img.as_raw());
        let _ = self.stdin.flush();
    }

    pub fn finish(mut self) {
        drop(self.stdin);
        let _ = self.process.wait();
    }
}
