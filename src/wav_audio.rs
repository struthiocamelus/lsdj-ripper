use rboy::AudioPlayer;
use std::path::Path;
use std::sync::{Arc, Mutex};
use wavers::{Samples, write};

pub struct Player {
    outfile: String,
    sample_rate: u32,
    buffer: Arc<Mutex<Vec<i16>>>,
}

impl Player {
    fn new(outfile: String, sample_rate: u32) -> Self {
        Self {
            outfile,
            sample_rate,
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn boxed_new(outfile: String, sample_rate: u32) -> Box<Self> {
        Box::new(Self::new(outfile, sample_rate))
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        write(
            Path::new(self.outfile.as_str()),
            self.buffer.lock().unwrap().as_mut(),
            self.sample_rate as i32,
            2,
        )
        .expect("Failed to write to file");
    }
}

impl AudioPlayer for Player {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]) {
        let buf: Vec<f32> = left_channel
            .iter()
            .zip(right_channel.iter())
            .flat_map(|t| [t.0.to_owned(), t.1.to_owned()])
            .collect();
        let mut vec = Samples::from(buf).convert::<i16>().to_vec();
        self.buffer
            .lock()
            .expect("Failed to lock player buffer")
            .append(&mut vec);
    }

    fn samples_rate(&self) -> u32 {
        self.sample_rate
    }

    fn underflowed(&self) -> bool {
        false
    }
}
