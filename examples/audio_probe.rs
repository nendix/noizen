//! Developer-only backend probe. Quiet tones; Ctrl-C releases the stream.
use rodio::{OutputStreamBuilder, Sink, Source, source::SineWave};
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;
    let stream = OutputStreamBuilder::open_default_stream()
        .map_err(|e| format!("cannot open default audio output: {e}"))?;
    eprintln!(
        "Output: {:?}; quiet mixed tones; Ctrl-C to stop",
        stream.config()
    );
    let sink = Sink::connect_new(stream.mixer());
    sink.append(
        SineWave::new(440.0)
            .amplify(0.01)
            .mix(SineWave::new(660.0).amplify(0.01)),
    );
    rx.recv()?;
    sink.stop();
    drop(stream);
    Ok(())
}
