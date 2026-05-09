use hound::{WavReader, WavWriter};
use pitch_shift::Shifter;

static mut OUTPUT: Vec<f32> = Vec::new();
static mut SPEC: Option<hound::WavSpec> = None;
static mut SHIFTER: Option<Shifter> = None;
static mut CHUNK_SIZE: usize = 0;
static mut CHUNK_IDX: usize = 0;
static mut SAMPLES: Vec<f32> = Vec::new();

pub fn start() {
    unsafe {
        let mut reader = WavReader::open("bamG.wav").unwrap();
        SPEC = Some(reader.spec());
        SAMPLES = reader.into_samples::<f32>().map(|s| s.unwrap()).collect();
        
        let spec = SPEC.unwrap();
        SHIFTER = Some(Shifter::new(spec.sample_rate as usize, 1));
        CHUNK_SIZE = spec.sample_rate as usize / 60;
        CHUNK_IDX = 0;
        OUTPUT = Vec::new();
    }
}

pub fn go(note: &str) {
    unsafe {
        let semitones = match note {
            "c" => 0.0,
            "d" => 2.0,
            "e" => 4.0,
            "f" => 5.0,
            "g" => 7.0,
            "a" => 9.0,
            "b" => 11.0,
            _ => 0.0,
        };

        let start = CHUNK_IDX * CHUNK_SIZE;
        let end = (start + CHUNK_SIZE).min(SAMPLES.len());
        let chunk = &SAMPLES[start..end];

        let shifted = SHIFTER.as_mut().unwrap().shift(semitones, chunk);
        OUTPUT.extend(shifted);
        CHUNK_IDX += 1;
    }
}

pub fn stop() {
    unsafe {
        let spec = SPEC.unwrap();
        let mut writer = WavWriter::create("output.wav", spec).unwrap();
        for sample in &OUTPUT {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();
    }
}
