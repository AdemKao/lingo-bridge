//! Offline WAV smoke tests. No LINE capture and no cloud services.
//!
//! cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- stt IN.WAV STT.ONNX TOKENS.TXT
//! cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- tts-ja TTS_DIR OUT.WAV "こんにちは"
//! cargo run -p lingo-speech --features sherpa --bin lingo-speech-smoke -- tts-zh MATCHA_DIR VOCODER.ONNX OUT.WAV "你好"
//! cargo run -p lingo-speech --features offline-pipeline --bin lingo-speech-smoke --
//!   translate-wav zh-ja IN.WAV STT.ONNX TOKENS.TXT qwen3:4b TTS_DIR - OUT.WAV

use lingo_speech::sherpa::{matcha_chinese, sensevoice_wav, supertonic_japanese};
#[cfg(feature = "offline-pipeline")]
use lingo_speech::offline::{translate_text, translate_wav, Direction, Settings};
use std::path::Path;

const USAGE: &str = "使用方法：\
\n  stt INPUT_16KHZ.WAV SENSEVOICE.ONNX TOKENS.TXT\
\n  tts-ja SUPERTONIC_DIR OUTPUT.WAV JAPANESE_TEXT\
\n  tts-zh MATCHA_DIR VOCODER.ONNX OUTPUT.WAV CHINESE_TEXT\
\n  translate-wav zh-ja|ja-zh INPUT_16KHZ.WAV STT.ONNX TOKENS.TXT OLLAMA_MODEL TTS_DIR VOCODER_ONNX_OR_DASH OUTPUT.WAV [requires --features offline-pipeline]\
\n  translate-text zh-ja|ja-zh OLLAMA_MODEL TEXT [requires --features offline-pipeline]";

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
        Some("tts-zh") if args.len() >= 6 => {
            let content = args[5..].join(" ");
            let saved = matcha_chinese(
                Path::new(&args[2]),
                Path::new(&args[3]),
                &content,
                Path::new(&args[4]),
            )?;
            println!("Saved: {}", saved.display());
            Ok(())
        }
        #[cfg(feature = "offline-pipeline")]
        Some("translate-wav") if args.len() == 10 => {
            let direction = Direction::parse(&args[2])?;
            let vocoder = if args[8] == "-" {
                None
            } else {
                Some(Path::new(&args[8]))
            };
            let result = translate_wav(Settings {
                direction,
                input_wav: Path::new(&args[3]),
                stt_model: Path::new(&args[4]),
                stt_tokens: Path::new(&args[5]),
                ollama_model: &args[6],
                tts_model_dir: Path::new(&args[7]),
                chinese_vocoder: vocoder,
                output_wav: Path::new(&args[9]),
            })?;
            println!("Original: {}", result.source_text);
            println!("Translation: {}", result.translated_text);
            println!("Saved: {}", result.output_wav.display());
            Ok(())
        }
        #[cfg(feature = "offline-pipeline")]
        Some("translate-text") if args.len() >= 5 => {
            let direction = Direction::parse(&args[2])?;
            println!("{}", translate_text(&args[3], direction, &args[4..].join(" "))?);
            Ok(())
        }
        _ => Err(USAGE.into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
