use crate::prelude::*;
use crate::tao::global::get_app;
use ocr_rs::OcrEngine;
use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::Manager;

struct OcrModel {
    engine: OcrEngine,
}

static OCR_ENGINES: OnceLock<Vec<OcrModel>> = OnceLock::new();

fn models_dir() -> PathBuf {
    get_app()
        .path()
        .resource_dir()
        .expect("Failed to get resource dir")
        .join("models")
}

fn init_engines() -> Vec<OcrModel> {
    let dir = models_dir();
    let det_path = dir.join("PP-OCRv5_mobile_det.mnn");
    let det = det_path.to_str().expect("Invalid det model path");

    let models: Vec<(&str, &str, &str)> = vec![
        (
            "default",
            "PP-OCRv5_mobile_rec.mnn",
            "ppocr_keys_v5.txt",
        ),
        (
            "english",
            "en_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_en.txt",
        ),
        (
            "latin",
            "latin_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_latin.txt",
        ),
        (
            "cyrillic",
            "cyrillic_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_cyrillic.txt",
        ),
        (
            "eslav",
            "eslav_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_eslav.txt",
        ),
        (
            "korean",
            "korean_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_korean.txt",
        ),
        (
            "arabic",
            "arabic_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_arabic.txt",
        ),
        (
            "devanagari",
            "devanagari_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_devanagari.txt",
        ),
        (
            "thai",
            "th_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_th.txt",
        ),
        (
            "greek",
            "el_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_el.txt",
        ),
        (
            "tamil",
            "ta_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_ta.txt",
        ),
        (
            "telugu",
            "te_PP-OCRv5_mobile_rec_infer.mnn",
            "ppocr_keys_te.txt",
        ),
    ];

    let mut engines = Vec::new();

    for (name, rec_file, charset_file) in models {
        let rec_path = dir.join(rec_file);
        let charset_path = dir.join(charset_file);

        if !rec_path.exists() || !charset_path.exists() {
            printlog!("OCR model files missing for {}, skipping", name);
            continue;
        }

        match OcrEngine::new(
            det,
            rec_path.to_str().expect("Invalid rec model path"),
            charset_path.to_str().expect("Invalid charset path"),
            None,
        ) {
            Ok(engine) => {
                engines.push(OcrModel { engine });
            }
            Err(e) => {
                printlog!("OCR: failed to load {} model: {:?}", name, e);
            }
        }
    }

    printlog!("OCR: loaded {} models", engines.len());
    engines
}

fn get_engines() -> &'static Vec<OcrModel> {
    OCR_ENGINES.get_or_init(init_engines)
}

pub fn extract_text_from_image(img_bytes: &[u8]) -> Option<String> {
    let image = image::load_from_memory(img_bytes).ok()?;

    let engines = get_engines();
    if engines.is_empty() {
        return None;
    }

    let mut best_text = String::new();
    let mut best_confidence: f32 = 0.0;

    for model in engines {
        if let Ok(results) = model.engine.recognize(&image) {
            let total_confidence: f32 = if results.is_empty() {
                0.0
            } else {
                results.iter().map(|r| r.confidence).sum::<f32>() / results.len() as f32
            };

            let text: String = results
                .iter()
                .map(|r| r.text.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            if !text.is_empty() && total_confidence > best_confidence {
                best_confidence = total_confidence;
                best_text = text;
            }
        }
    }

    if best_text.is_empty() {
        None
    } else {
        printlog!("OCR: {} chars, {:.0}% confidence", best_text.len(), best_confidence * 100.0);
        Some(best_text)
    }
}
