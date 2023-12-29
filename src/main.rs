mod player;

use mki::{Action, Keyboard};
use player::{SinkPool, SinkPoolError};
use rand::prelude::*;
use std::sync::Arc;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let pool = Arc::new(SinkPool::new(&stream_handle, 20));

    mki::bind_any_key(Action::handle_kb(move |key| {
        if key == Keyboard::Escape {
            std::process::exit(0);
        }
        let rng = thread_rng().gen_range(1..=6);
        if let Err(SinkPoolError::NoFreeSink) =
            pool.play_one(format!("assets/bad/{rng}.wav").as_str())
        {
            println!("no free sink");
        }
    }));

    loop {
        std::thread::sleep(std::time::Duration::from_secs(120));
    }
}
