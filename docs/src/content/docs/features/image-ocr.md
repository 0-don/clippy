---
title: Image OCR
description: Automatic text extraction from clipboard images.
---

Clippy's **Image OCR** feature automatically extracts text from images you copy to the clipboard. This makes image content searchable, so you can find screenshots, photos of documents, or any image containing text just by typing what you remember.

## Key Features

---

### Automatic Text Extraction

- When you copy an image to the clipboard, Clippy runs OCR in the background automatically.
- The extracted text is stored alongside the image and indexed for search.
- OCR runs asynchronously, so clipboard capture remains instant with no delay.

### 100+ Languages Supported

Clippy bundles all 12 PaddleOCR v5 script family models, covering over 100 languages:

| Script Family | Languages |
|---|---|
| **Default** | Chinese (Simplified/Traditional), English, Japanese, Pinyin |
| **Latin** | French, German, Spanish, Italian, Portuguese, Polish, Dutch, Turkish, Vietnamese, Finnish, Swedish, and 30+ more |
| **Cyrillic** | Russian, Serbian, Bulgarian, Mongolian, Kazakh, and 30+ more |
| **East Slavic** | Russian, Ukrainian, Belarusian |
| **Korean** | Korean |
| **Arabic** | Arabic, Persian, Uyghur, Urdu, Kurdish |
| **Devanagari** | Hindi, Marathi, Nepali, Sanskrit, Bengali |
| **Thai** | Thai |
| **Greek** | Greek |
| **Tamil** | Tamil |
| **Telugu** | Telugu |
| **English** | English (standalone optimized model) |

### Searchable Images

- Type any text that appeared in an image to find it in your clipboard history.
- Works with both the quick search bar and the image filter.
- Search matches OCR text alongside regular text, HTML, and file metadata.

---

## How It Works

1. **Copy an image** to your clipboard (screenshot, photo, or any image with text).
2. Clippy saves the image immediately and starts OCR in the background.
3. The OCR engine tries all script models and picks the result with the highest confidence.
4. Extracted text is saved to the database (encrypted if encryption is enabled).
5. **Search for any word** from the image and it appears in your results.

## Privacy & Security

- OCR runs entirely **offline** using bundled models. No data is sent to external services.
- When encryption is enabled, OCR text is encrypted before being stored in the database.
- OCR text is decrypted on the fly during search, just like other clipboard content.

## Technical Details

- Powered by [PaddleOCR v5](https://github.com/PaddlePaddle/PaddleOCR) models via the [ocr-rs](https://docs.rs/ocr-rs) Rust crate.
- Uses the MNN inference framework for fast, cross-platform performance.
- Models are loaded lazily on first image capture (adds ~60ms one-time initialization).
- Typical OCR processing time: 1 to 4 seconds per image depending on complexity.
