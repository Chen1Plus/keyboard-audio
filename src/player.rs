use rodio::{Decoder, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader};

pub fn new(stream_handle: &OutputStreamHandle, path: &str) -> Sink {
    let sink = Sink::try_new(stream_handle).unwrap();
    let source = Decoder::new(BufReader::new(
        File::open(path).expect("failed to open file"),
    ))
    .unwrap_or_else(|_| panic!("failed to decode file: {}", path));
    sink.append(source);
    sink
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_single() {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let a = super::new(&stream_handle, "assets/test/1.mp3");
        a.sleep_until_end();
    }
}
