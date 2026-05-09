use hound::{WavWriter, WavSpec};
use pitch_shift::Shifter;

pub struct AudioBuilder {
    samples: Vec<f32>,
    spec: WavSpec,
    chunk_size: usize,
    chunk_index: usize,
    output_samples: Vec<f32>,
    shifter: Shifter,
}

impl AudioBuilder {
    pub fn start() -> Result<Self, Box<dyn std::error::Error>> {
        // Load bamG.wav as the base
        let mut reader = hound::WavReader::open("bamG.wav")?;
        let spec = reader.spec();
        let samples: Vec<f32> = reader
            .into_samples::<f32>()
            .collect::<Result<Vec<_>, _>>()?;

        let chunk_size = spec.sample_rate as usize / 60; // 1/60 sec
        let shifter = Shifter::new(spec.sample_rate as usize, 1);

        Ok(AudioBuilder {
            samples,
            spec,
            chunk_size,
            chunk_index: 0,
            output_samples: Vec::new(),
            shifter,
        })
    }

    pub fn go(&mut self, note: &str) -> Result<(), Box<dyn std::error::Error>> {
        let semitones = self.note_to_semitones(note);
        
        let start = self.chunk_index * self.chunk_size;
        let end = (start + self.chunk_size).min(self.samples.len());
        let chunk = &self.samples[start..end];

        let shifted = self.shifter.shift(semitones, chunk);
        self.output_samples.extend(shifted);
        self.chunk_index += 1;

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = WavWriter::create("output.wav", self.spec)?;

        for sample in &self.output_samples {
            writer.write_sample(*sample)?;
        }

        writer.finalize()?;
        Ok(())
    }

    fn note_to_semitones(&self, note: &str) -> f32 {
        match note.to_lowercase().as_str() {
            "c" => 0.0,
            "d" => 2.0,
            "e" => 4.0,
            "f" => 5.0,
            "g" => 7.0,
            "a" => 9.0,
            "b" => 11.0,
            _ => 0.0,
        }
    }
}
