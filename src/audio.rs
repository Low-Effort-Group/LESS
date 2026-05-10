use hound::{WavReader, WavWriter, WavSpec, SampleFormat};
use tdpsola::{TdpsolaAnalysis, TdpsolaSynthesis, AlternatingHann, Speed};

pub struct Audio {
    spec: WavSpec,
    writer: WavWriter<std::io::BufWriter<std::fs::File>>,
    samples_written: usize,
    samples_vector: Vec<f32>,
}

impl Audio {
    pub fn init_audio() -> Audio {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };
        let writer = WavWriter::create("output.wav", spec).unwrap();
        Audio { spec, writer, samples_written: 0, samples_vector: Vec::new() }
    }

    pub fn add_sound(&mut self, frame: usize, filename: &str, pitch_shift: f32) {
        let sample_number = frame * (self.spec.sample_rate as usize) / 60;
        let reader = WavReader::open(filename).unwrap();
        let reader_samples: Vec<f32> = reader.into_samples::<i16>().map(|s| s.unwrap() as f32).collect();

        let source_wavelength = 4.0;
        let mut alternating_hann = AlternatingHann::new(source_wavelength);
        let mut analysis = TdpsolaAnalysis::new(&alternating_hann);
        for &sample in &reader_samples {
            analysis.push_sample(sample, &mut alternating_hann);
        }

        let mut synthesis = TdpsolaSynthesis::new(Speed::from_f32(pitch_shift), source_wavelength);
        let processed_samples: Vec<f32> = synthesis.iter(&analysis).collect();

        let required_len = sample_number + processed_samples.len();
        if required_len > self.samples_vector.len() {
            self.samples_vector.resize(required_len, 0.0);
        }

        for (i, sample) in processed_samples.into_iter().enumerate() {
            self.samples_vector[sample_number + i] += sample;
        }
    }

    pub fn finish_audio(mut self, total_frames: usize) {
        let samples_per_frame = 44100 / 60;
        let total_samples = total_frames * samples_per_frame;
        self.samples_vector.truncate(total_samples);
        self.samples_vector.resize(total_samples, 0.0);
        for &sample in &self.samples_vector {
            self.writer.write_sample(sample as i16).unwrap();
        }
        self.writer.finalize().unwrap();
    }
}