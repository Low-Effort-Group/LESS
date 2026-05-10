use log::*;

use image::{ImageBuffer, Rgba};
use std::io::Write;
use std::process::{Child, Command, Stdio};

pub struct VideoEncoder {
    process: Child,
    stdin: std::process::ChildStdin,
}

impl VideoEncoder {
    pub fn new(width: u32, height: u32, fps: u32) -> Option<Self> {
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
                "output.mp4",
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

    pub fn finish(mut self, output_path: &str) {
        drop(self.stdin);
        let _ = self.process.wait();
        // ffmpeg -i input_video.mp4 -i input_audio.mp3 -c:v copy -c:a aac output.mp4
        let mut process = Command::new("ffmpeg").args([
            "-i",
            "output.mp4",
            "-i",
            "audio.wav",
            "-c:v",
            "copy",
            "-c:a",
            "aac",
            "-y",
            output_path,
        ]).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().ok();
        let _ = process.unwrap().wait();

        trace!("Video encoder finished successfully.");
    }
}
