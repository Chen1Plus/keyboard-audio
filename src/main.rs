mod player;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    player::new(&stream_handle, "assets/knock_metal_door1.mp3");
}
