use hound::{WavReader, WavWriter, WavSpec};
use pitch_shift::Shifter;

pub struct Audio {
    spec: WavSpec,
    writer: WavWriter,//<std::io::BufWriter<std::fs::File>>,    
}

pub fn init_audio() {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
    };
    let writer = WavWriter::create("output.wav", spec).unwrap();
}

pub fn add_sound(frame: i32, filename: &str, pitch: f32) {


}

pub fn finish_audio() {

}