//! Manual WAV-based local model tests (NOT a real-time LINE call).
//!
//! cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- \
//!   stt PATH_TO_16KHZ_WAV PATH_TO_MODEL.ONNX PATH_TO_TOKENS.TXT
//! cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- \
//!   tts-ja PATH_TO_SUPERTONIC_MODEL_DIR OUTPUT.WAV "こんにちは"

use lingo_speech::sherpa::{sensevoice_wav, supertonic_japanese};
use std::path::Path;

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("stt") if args.len() == 5 => {
            let text = sensevoice_wav(
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
            )?;
            println!("{text}");
            Ok(())
        }
        Some("tts-ja") if args.len() >= 5 => {
            let content = args[4..].join(" ");
            let saved = supertonic_japanese(
                Path::new(&args[2]),
                &content,
                Path::new(&args[3]),
            )?;
            println!("Saved: {}", saved.display());
            Ok(())
        }
        _ => Err(
            "用法：stt INPUT_16KHZ.WAV MODEL.ONNX TOKENS.TXT | tts-ja MODEL_DIR OUTPUT.WAV JAPANESE_TEXT".into(),
        ),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
