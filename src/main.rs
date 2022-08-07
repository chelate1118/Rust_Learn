#![allow(non_snake_case)]

fn main()
{
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, source::Source};

// Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("src/resource/046.wav").unwrap());
// Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
// Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples()).ok();

// The sound plays in a separate audio thread,
// so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_millis(700));
}