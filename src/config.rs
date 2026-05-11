use serde::{Deserialize, Serialize};
use serde_json::Result;
use clap::Parser;
use crate::types::ball::Ball;
use crate::types::HSL;

const WIDTH: u32 = 1080;
const HEIGHT: u32 = 1920;
const FPS: u32 = 60;
const DURATION: u32 = 20;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub width: u32,
    #[serde(default)]
    pub height: u32,
    #[serde(default)]
    pub fps: u32,
    #[serde(default)]
    pub duration: u32,
    #[serde(default = "default_balls")]
    pub balls: Vec<Ball>,
}

fn default_balls() -> Vec<Ball> {
    vec![BALL]
}

fn create_json(filename: &str, config: Config) -> Result<()> {
    let data = serde_json::to_string_pretty(&config)?;
    std::fs::write(filename, data).unwrap();
    Ok(())
}

impl Config {
    pub fn read(filename: &str) -> Result<Config> {
        if !std::path::Path::new(filename).exists() {
            create_json(filename, Config::default())?;
        }

        let data = std::fs::read_to_string(filename).unwrap();
        let mut c: Config = serde_json::from_str(&data)?;
        if c.width == 0 { c.width = WIDTH; }
        if c.height == 0 { c.height = HEIGHT; }
        if c.fps == 0 { c.fps = FPS; }
        if c.duration == 0 { c.duration = DURATION; }
        for i in 0..c.balls.len() {
            if c.balls[i].x == 0.0 { c.balls[i].x = BALL.x; }
            if c.balls[i].y == 0.0 { c.balls[i].y = BALL.y; }
            if c.balls[i].vx == 0.0 { c.balls[i].vx = BALL.vx; }
            if c.balls[i].vy == 0.0 { c.balls[i].vy = BALL.vy; }
            if c.balls[i].radius == 0.0 { c.balls[i].radius = BALL.radius; }
            if c.balls[i].gravity == 0.0 { c.balls[i].gravity = BALL.gravity; }
            if c.balls[i].restitution == 0.0 { c.balls[i].restitution = BALL.restitution; }
            if c.balls[i].friction == 0.0 { c.balls[i].friction = BALL.friction; }
            if c.balls[i].color.h == 0.0 { c.balls[i].color.h = BALL.color.h; }
            if c.balls[i].color.s == 0.0 { c.balls[i].color.s = BALL.color.s; }
            if c.balls[i].color.l == 0.0 { c.balls[i].color.l = BALL.color.l; }
        }

        Ok(c)
    }

    pub fn read_args(filename: &str) -> Result<Config> {    
        let args = Args::parse();
        let mut config = Config::read(&filename)?;
        if args.fps.is_some() { config.fps = args.fps.unwrap(); }
        if args.duration.is_some() { config.duration = args.duration.unwrap(); }
        Ok(config)
    }

    // pub fn to_file(&self, filename: &str) -> Result<()> {
    //     let data = serde_json::to_string_pretty(self)?;
    //     std::fs::write(filename, data).unwrap();
    //     Ok(())
    // }

    pub fn default() -> Config {
        Config {
            width: WIDTH,
            height: HEIGHT,
            fps: FPS,
            duration: DURATION,
            balls: vec![BALL],
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "LESS", about = "Low Effort Slop System", long_about = "Low Effort Slop System, A short-form braindead content generator.")]
pub struct Args {
    /// Frames per second
    #[arg(long="fps")]
    fps: Option<u32>,

    /// Max duration in seconds
    #[arg(long="duration", short='d')]
    duration: Option<u32>,
}

/// This is the default ball
const BALL: Ball = Ball {
    x: WIDTH as f32 / 2.0 + 100.0,
    y: HEIGHT as f32 / 2.0,
    vx: 200.0,
    vy: 150.0,
    radius: 30.0,
    gravity: 1000.0,
    restitution: 1.05,
    friction: 0.8,
    color: HSL { h: 240.0, s: 1.0, l: 0.5 },
};