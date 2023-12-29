#![allow(dead_code)]

use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{
    fs::File,
    io::{self, BufReader},
    iter,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SinkPoolError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("no sink available (all playing)")]
    NoFreeSink,
}

pub struct SinkPool {
    pub sinks: Vec<Sink>,
}

impl SinkPool {
    pub fn new(stream_handle: &OutputStreamHandle, sink_cnt: usize) -> Self {
        SinkPool {
            sinks: iter::repeat_with(|| Sink::try_new(stream_handle).unwrap())
                .take(sink_cnt)
                .collect::<Vec<_>>(),
        }
    }

    pub fn play_one(&self, path: &str) -> Result<(), SinkPoolError> {
        match self.sinks.iter().find(|&sink| sink.empty()) {
            None => Err(SinkPoolError::NoFreeSink),
            Some(sink) => {
                sink.append(Decoder::new(BufReader::new(File::open(path)?)).unwrap());
                Ok(())
            }
        }
    }

    pub fn sleep_until_end(&self) {
        self.sinks.iter().for_each(|sink| sink.sleep_until_end());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_pool() {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let pool = super::SinkPool::new(&stream_handle, 4);
        pool.play_one("assets/test/1.mp3").unwrap();
        pool.sleep_until_end();
    }
}
