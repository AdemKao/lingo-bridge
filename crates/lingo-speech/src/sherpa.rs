//! Opt-in sherpa-onnx proof-of-concept adapters.
//!
//! Model files are user-provided. Nothing is downloaded by inference functions.
//! These operations are blocking and deliberately not wired into live calls yet.

use sherpa_onnx::{
    GenerationConfig, OfflineRecognizer, OfflineRecognizerConfig, OfflineSenseVoiceModelConfig,
    OfflineTts, OfflineTtsConfig, OfflineTtsMatchaModelConfig,
    OfflineTtsSupertonicModelConfig, Wave,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn ensure_file(path: &Path) -> Result<String, String> {
    if !path.is_file() {
        return Err(format!("找不到模型檔案：{}", path.display()));
    }
    Ok(path.to_string_lossy().into_owned())
}

fn file_in(dir: &Path, filename: &str) -> Result<String, String> {
    ensure_file(&dir.join(filename))
}

/// SenseVoice supports Chinese and Japanese in a single offline recognizer.
/// The WAV file should be 16kHz mono; prepare/resample it before inference.
pub fn sensevoice_wav(wav: &Path, model: &Path, tokens: &Path) -> Result<String, String> {
    let wave_path = ensure_file(wav)?;
    let model_path = ensure_file(model)?;
    let tokens_path = ensure_file(tokens)?;
    let wave = Wave::read(&wave_path).ok_or("無法讀取 WAV，請確認為有效的 WAV 檔案")?;
    if wave.sample_rate() != 16_000 {
        return Err("SenseVoice 範例只接受 16 kHz WAV，請先轉換採樣率".into());
    }
    if wave.samples().is_empty() {
        return Err("WAV 不包含音訊資料".into());
    }

    let mut config = OfflineRecognizerConfig::default();
    config.model_config.sense_voice = OfflineSenseVoiceModelConfig {
        model: Some(model_path),
        language: Some("auto".into()),
        use_itn: true,
    };
    config.model_config.tokens = Some(tokens_path);
    config.model_config.num_threads = 2;

    let recognizer = OfflineRecognizer::create(&config).ok_or("無法載入 SenseVoice 模型")?;
    let stream = recognizer.create_stream();
    stream.accept_waveform(wave.sample_rate(), wave.samples());
    recognizer.decode(&stream);
    let result = stream.get_result().ok_or("沒有取得辨識結果")?;
    Ok(result.text.trim().to_owned())
}

/// Japanese TTS POC using Supertonic v3's local ONNX weights.
/// Note: Supertonic v3 does NOT support Mandarin Chinese voices.
pub fn supertonic_japanese(
    model_dir: &Path,
    text: &str,
    wav_out: &Path,
) -> Result<PathBuf, String> {
    if text.trim().is_empty() || text.chars().count() > 600 {
        return Err("請輸入 1–600 字的日文文字".into());
    }
    if let Some(parent) = wav_out.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err("輸出 WAV 資料夾不存在".into());
        }
    }
    let model = OfflineTtsSupertonicModelConfig {
        duration_predictor: Some(file_in(model_dir, "duration_predictor.int8.onnx")?),
        text_encoder: Some(file_in(model_dir, "text_encoder.int8.onnx")?),
        vector_estimator: Some(file_in(model_dir, "vector_estimator.int8.onnx")?),
        vocoder: Some(file_in(model_dir, "vocoder.int8.onnx")?),
        tts_json: Some(file_in(model_dir, "tts.json")?),
        unicode_indexer: Some(file_in(model_dir, "unicode_indexer.bin")?),
        voice_style: Some(file_in(model_dir, "voice.bin")?),
    };
    let config = OfflineTtsConfig {
        model: sherpa_onnx::OfflineTtsModelConfig {
            supertonic: model,
            num_threads: 2,
            ..Default::default()
        },
        ..Default::default()
    };
    let tts = OfflineTts::create(&config).ok_or("無法載入 Supertonic 模型")?;
    let mut extra = HashMap::new();
    extra.insert("lang".to_owned(), serde_json::json!("ja"));
    let generation = GenerationConfig {
        sid: 0,
        speed: 1.0,
        num_steps: 8,
        extra: Some(extra),
        ..Default::default()
    };
    let audio = tts
        .generate_with_config(
            text,
            &generation,
            Some(|_samples: &[f32], _progress: f32| true),
        )
        .ok_or("日文語音合成失敗")?;
    let output = wav_out.to_str().ok_or("輸出路徑包含無法處理的字元")?;
    if !audio.save(output) {
        return Err("無法儲存合成後的 WAV 檔案".into());
    }
    Ok(wav_out.to_path_buf())
}


/// Simplified/Traditional Chinese speech with Matcha-TTS Chinese Baker ONNX.
///
/// The model normally expects Chinese lexicon/dictionary data and a separate
/// 22kHz Vocos vocoder. All weights are supplied by the user and are not
/// redistributed. Test Traditional Chinese pronunciation independently:
/// this voice was trained primarily on Mainland Mandarin material.
pub fn matcha_chinese(
    model_dir: &Path,
    vocoder_path: &Path,
    text: &str,
    wav_out: &Path,
) -> Result<PathBuf, String> {
    if text.trim().is_empty() || text.chars().count() > 600 {
        return Err("請輸入 1–600 字的中文文字".into());
    }
    if let Some(parent) = wav_out.parent() {
        if !parent.as_os_str().is_empty() && !parent.is_dir() {
            return Err("輸出 WAV 資料夾不存在".into());
        }
    }
    let dict_dir = model_dir.join("dict");
    if !dict_dir.is_dir() {
        return Err(format!("找不到 Matcha 中文字典：{}", dict_dir.display()));
    }
    let model = OfflineTtsMatchaModelConfig {
        acoustic_model: Some(file_in(model_dir, "model-steps-3.onnx")?),
        vocoder: Some(ensure_file(vocoder_path)?),
        lexicon: Some(file_in(model_dir, "lexicon.txt")?),
        tokens: Some(file_in(model_dir, "tokens.txt")?),
        dict_dir: Some(dict_dir.to_string_lossy().into_owned()),
        noise_scale: 0.667,
        length_scale: 1.0,
        ..Default::default()
    };

    let rules = [
        file_in(model_dir, "phone.fst")?,
        file_in(model_dir, "date.fst")?,
        file_in(model_dir, "number.fst")?,
    ]
    .join(",");
    let config = OfflineTtsConfig {
        model: sherpa_onnx::OfflineTtsModelConfig {
            matcha: model,
            num_threads: 2,
            ..Default::default()
        },
        rule_fsts: Some(rules),
        ..Default::default()
    };
    let tts = OfflineTts::create(&config).ok_or("無法載入 Matcha 中文語音模型")?;
    let generation = GenerationConfig {
        sid: 0,
        speed: 1.0,
        ..Default::default()
    };
    let audio = tts
        .generate_with_config(
            text,
            &generation,
            Some(|_samples: &[f32], _progress: f32| true),
        )
        .ok_or("中文語音合成失敗")?;
    let output = wav_out.to_str().ok_or("輸出路徑包含無法處理的字元")?;
    if !audio.save(output) {
        return Err("無法儲存中文語音 WAV".into());
    }
    Ok(wav_out.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_chinese_model_files_fail_before_inference() {
        let root = std::env::temp_dir().join("lingo-nonexistent-matcha-2026");
        let result = matcha_chinese(
            &root,
            &root.join("vocos.onnx"),
            "你好",
            &root.join("output.wav"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn empty_chinese_input_is_rejected() {
        let result = matcha_chinese(
            Path::new("."),
            Path::new("vocos-22khz-univ.onnx"),
            "   ",
            Path::new("out.wav"),
        );
        assert!(result.is_err());
    }
}
