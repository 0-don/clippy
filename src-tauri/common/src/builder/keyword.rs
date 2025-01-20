use crate::types::enums::{ClipboardTextType, ClipboardType, Language};
use std::collections::HashMap;

pub struct KeywordBuilder {
    clipboard_keywords: HashMap<Language, HashMap<ClipboardType, Vec<String>>>,
    text_keywords: HashMap<Language, HashMap<ClipboardTextType, Vec<String>>>,
}

impl KeywordBuilder {
    pub fn new() -> Self {
        Self {
            clipboard_keywords: HashMap::new(),
            text_keywords: HashMap::new(),
        }
    }

    pub fn add_type(mut self, lang: Language, type_: ClipboardType, keywords: &[&str]) -> Self {
        self.clipboard_keywords
            .entry(lang)
            .or_default()
            .insert(type_, keywords.iter().map(|&s| s.to_string()).collect());
        self
    }

    pub fn add_text_type(
        mut self,
        lang: Language,
        type_: ClipboardTextType,
        keywords: &[&str],
    ) -> Self {
        self.text_keywords
            .entry(lang)
            .or_default()
            .insert(type_, keywords.iter().map(|&s| s.to_string()).collect());
        self
    }

    pub fn build(
        self,
    ) -> (
        HashMap<Language, HashMap<ClipboardType, Vec<String>>>,
        HashMap<Language, HashMap<ClipboardTextType, Vec<String>>>,
    ) {
        (self.clipboard_keywords, self.text_keywords)
    }

    pub fn find_clipboard_type(
        term: &str,
        lang: &Language,
        keywords: &HashMap<Language, HashMap<ClipboardType, Vec<String>>>,
    ) -> Option<ClipboardType> {
        keywords.get(lang).and_then(|lang_keywords| {
            lang_keywords
                .iter()
                .find(|(_, words)| words.iter().any(|word| word.eq_ignore_ascii_case(term)))
                .map(|(clip_type, _)| clip_type.clone())
        })
    }

    pub fn find_text_type(
        term: &str,
        lang: &Language,
        keywords: &HashMap<Language, HashMap<ClipboardTextType, Vec<String>>>,
    ) -> Option<ClipboardTextType> {
        keywords.get(lang).and_then(|lang_keywords| {
            lang_keywords
                .iter()
                .find(|(_, words)| words.iter().any(|word| word.eq_ignore_ascii_case(term)))
                .map(|(text_type, _)| text_type.clone())
        })
    }

    pub fn build_default() -> (
        HashMap<Language, HashMap<ClipboardType, Vec<String>>>,
        HashMap<Language, HashMap<ClipboardTextType, Vec<String>>>,
    ) {
        let mut builder = KeywordBuilder::new();

        // English (en)
        builder = builder
            .add_type(
                Language::English,
                ClipboardType::Text,
                &["text", "txt", "plain"],
            )
            .add_type(
                Language::English,
                ClipboardType::Image,
                &["image", "img", "picture", "photo"],
            )
            .add_type(
                Language::English,
                ClipboardType::Html,
                &["html", "webpage", "markup"],
            )
            .add_type(
                Language::English,
                ClipboardType::Rtf,
                &["rtf", "rich text", "formatted"],
            )
            .add_type(
                Language::English,
                ClipboardType::File,
                &["file", "document", "files"],
            )
            .add_text_type(
                Language::English,
                ClipboardTextType::Text,
                &["text", "txt", "plain"],
            )
            .add_text_type(
                Language::English,
                ClipboardTextType::Link,
                &["link", "url", "website"],
            )
            .add_text_type(
                Language::English,
                ClipboardTextType::Hex,
                &["hex", "hexadecimal", "color code"],
            )
            .add_text_type(
                Language::English,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "color"],
            );

        // Mandarin (zh)
        builder = builder
            .add_type(Language::Mandarin, ClipboardType::Text, &["文本", "txt"]) // Text, txt
            .add_type(Language::Mandarin, ClipboardType::Image, &["图像", "图片"]) // Image, picture
            .add_type(Language::Mandarin, ClipboardType::Html, &["html", "网页"]) // Html, webpage
            .add_type(Language::Mandarin, ClipboardType::Rtf, &["富文本"]) // Rich Text
            .add_type(Language::Mandarin, ClipboardType::File, &["文件", "文档"]) // File, document
            .add_text_type(
                Language::Mandarin,
                ClipboardTextType::Text,
                &["文本", "txt"],
            )
            .add_text_type(
                Language::Mandarin,
                ClipboardTextType::Link,
                &["链接", "网址"],
            )
            .add_text_type(
                Language::Mandarin,
                ClipboardTextType::Hex,
                &["十六进制", "颜色代码"],
            )
            .add_text_type(
                Language::Mandarin,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "颜色"],
            );

        // Hindi (hi)
        builder = builder
            .add_type(
                Language::Hindi,
                ClipboardType::Text,
                &["पाठ", "txt", "टेक्स्ट"],
            ) // Text
            .add_type(
                Language::Hindi,
                ClipboardType::Image,
                &["चित्र", "तस्वीर", "इमेज"],
            ) // Image, Picture
            .add_type(
                Language::Hindi,
                ClipboardType::Html,
                &["html", "एचटीएमएल", "वेबपेज"],
            ) // HTML, Webpage
            .add_type(Language::Hindi, ClipboardType::Rtf, &["आरटीएफ", "रिच टेक्स्ट"]) // RTF, Rich Text
            .add_type(
                Language::Hindi,
                ClipboardType::File,
                &["फ़ाइल", "दस्तावेज़", "फाइलें"],
            ) // File, Document
            .add_text_type(
                Language::Hindi,
                ClipboardTextType::Text,
                &["पाठ", "txt", "टेक्स्ट"],
            )
            .add_text_type(
                Language::Hindi,
                ClipboardTextType::Link,
                &["लिंक", "यूआरएल", "वेबसाइट"],
            )
            .add_text_type(
                Language::Hindi,
                ClipboardTextType::Hex,
                &["हेक्स", "हेक्साडेसिमल", "कलर कोड"],
            )
            .add_text_type(
                Language::Hindi,
                ClipboardTextType::Rgb,
                &["आरजीबी", "आरजीबीए", "कलर"],
            );

        // Spanish (es)
        builder = builder
            .add_type(Language::Spanish, ClipboardType::Text, &["texto", "txt"])
            .add_type(
                Language::Spanish,
                ClipboardType::Image,
                &["imagen", "foto", "imágen"],
            )
            .add_type(
                Language::Spanish,
                ClipboardType::Html,
                &["html", "página web"],
            )
            .add_type(
                Language::Spanish,
                ClipboardType::Rtf,
                &["rtf", "texto enriquecido"],
            )
            .add_type(
                Language::Spanish,
                ClipboardType::File,
                &["archivo", "fichero", "archivos"],
            )
            .add_text_type(
                Language::Spanish,
                ClipboardTextType::Text,
                &["texto", "txt"],
            )
            .add_text_type(
                Language::Spanish,
                ClipboardTextType::Link,
                &["enlace", "url", "vinculo"],
            )
            .add_text_type(
                Language::Spanish,
                ClipboardTextType::Hex,
                &["hexadecimal", "código de color"],
            )
            .add_text_type(
                Language::Spanish,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "color"],
            );

        // French (fr)
        builder = builder
            .add_type(Language::French, ClipboardType::Text, &["texte", "txt"])
            .add_type(Language::French, ClipboardType::Image, &["image", "photo"])
            .add_type(Language::French, ClipboardType::Html, &["html", "page web"])
            .add_type(
                Language::French,
                ClipboardType::Rtf,
                &["rtf", "texte enrichi"],
            )
            .add_type(
                Language::French,
                ClipboardType::File,
                &["fichier", "document", "fichiers"],
            )
            .add_text_type(Language::French, ClipboardTextType::Text, &["texte", "txt"])
            .add_text_type(
                Language::French,
                ClipboardTextType::Link,
                &["lien", "url", "site"],
            )
            .add_text_type(
                Language::French,
                ClipboardTextType::Hex,
                &["hex", "hexadécimal", "code couleur"],
            )
            .add_text_type(
                Language::French,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "couleur"],
            );

        // Arabic (ar)
        builder = builder
            .add_type(Language::Arabic, ClipboardType::Text, &["نص", "txt"]) // Text
            .add_type(Language::Arabic, ClipboardType::Image, &["صورة", "صوره"]) // Image, picture
            .add_type(Language::Arabic, ClipboardType::Html, &["html", "صفحة ويب"]) // Html, webpage
            .add_type(Language::Arabic, ClipboardType::Rtf, &["نص منسق"]) // Rich Text
            .add_type(Language::Arabic, ClipboardType::File, &["ملف", "ملفات"]) // File, files
            .add_text_type(Language::Arabic, ClipboardTextType::Text, &["نص", "txt"])
            .add_text_type(Language::Arabic, ClipboardTextType::Link, &["رابط", "url"])
            .add_text_type(
                Language::Arabic,
                ClipboardTextType::Hex,
                &["سداسي عشري", "رمز اللون"],
            )
            .add_text_type(
                Language::Arabic,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "لون"],
            );

        // Bengali (bn)
        builder = builder
            .add_type(Language::Bengali, ClipboardType::Text, &["পাঠ", "txt"]) // Text
            .add_type(Language::Bengali, ClipboardType::Image, &["ছবি", "চিত্র"]) // Image, picture
            .add_type(Language::Bengali, ClipboardType::Html, &["html", "ওয়েবপেজ"]) // Html, webpage
            .add_type(Language::Bengali, ClipboardType::Rtf, &["রিচ টেক্সট"]) // Rich Text
            .add_type(Language::Bengali, ClipboardType::File, &["ফাইল", "নথি"]) // File, Document
            .add_text_type(Language::Bengali, ClipboardTextType::Text, &["পাঠ", "txt"])
            .add_text_type(
                Language::Bengali,
                ClipboardTextType::Link,
                &["লিঙ্ক", "ইউআরএল"],
            )
            .add_text_type(
                Language::Bengali,
                ClipboardTextType::Hex,
                &["হেক্স", "হেক্সাডেসিমেল", "কালার কোড"],
            )
            .add_text_type(
                Language::Bengali,
                ClipboardTextType::Rgb,
                &["আরজিবি", "আরজিবিএ", "কালার"],
            );

        // Portuguese (pt)
        builder = builder
            .add_type(Language::Portuguese, ClipboardType::Text, &["texto", "txt"])
            .add_type(
                Language::Portuguese,
                ClipboardType::Image,
                &["imagem", "foto"],
            )
            .add_type(
                Language::Portuguese,
                ClipboardType::Html,
                &["html", "página web"],
            )
            .add_type(
                Language::Portuguese,
                ClipboardType::Rtf,
                &["rtf", "texto formatado"],
            )
            .add_type(
                Language::Portuguese,
                ClipboardType::File,
                &["arquivo", "documento", "arquivos"],
            )
            .add_text_type(
                Language::Portuguese,
                ClipboardTextType::Text,
                &["texto", "txt"],
            )
            .add_text_type(
                Language::Portuguese,
                ClipboardTextType::Link,
                &["link", "url", "site"],
            )
            .add_text_type(
                Language::Portuguese,
                ClipboardTextType::Hex,
                &["hex", "hexadecimal", "código de cor"],
            )
            .add_text_type(
                Language::Portuguese,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "cor"],
            );

        // Russian (ru)
        builder = builder
            .add_type(Language::Russian, ClipboardType::Text, &["текст", "txt"]) // Text
            .add_type(
                Language::Russian,
                ClipboardType::Image,
                &["изображение", "картинка"],
            ) // Image, picture
            .add_type(
                Language::Russian,
                ClipboardType::Html,
                &["html", "веб-страница"],
            ) // Html, webpage
            .add_type(
                Language::Russian,
                ClipboardType::Rtf,
                &["форматированный текст"],
            ) // Rich Text
            .add_type(
                Language::Russian,
                ClipboardType::File,
                &["файл", "документ"],
            ) // File, document
            .add_text_type(
                Language::Russian,
                ClipboardTextType::Text,
                &["текст", "txt"],
            )
            .add_text_type(
                Language::Russian,
                ClipboardTextType::Link,
                &["ссылка", "url"],
            )
            .add_text_type(
                Language::Russian,
                ClipboardTextType::Hex,
                &["шестнадцатеричный", "код цвета"],
            )
            .add_text_type(
                Language::Russian,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "цвет"],
            );

        // Urdu (ur)
        builder = builder
            .add_type(Language::Urdu, ClipboardType::Text, &["متن", "txt"]) // Text
            .add_type(Language::Urdu, ClipboardType::Image, &["تصویر", "عکس"]) // Image, picture
            .add_type(Language::Urdu, ClipboardType::Html, &["html", "ویب پیج"]) // Html, webpage
            .add_type(Language::Urdu, ClipboardType::Rtf, &["محفوظ شدہ متن"]) // Rich Text
            .add_type(Language::Urdu, ClipboardType::File, &["فائل", "دستاویز"]) // File, document
            .add_text_type(Language::Urdu, ClipboardTextType::Text, &["متن", "txt"])
            .add_text_type(
                Language::Urdu,
                ClipboardTextType::Link,
                &["لنک", "یو آر ایل"],
            )
            .add_text_type(
                Language::Urdu,
                ClipboardTextType::Hex,
                &["ہیکسا ڈیسیمل", "رنگ کوڈ"],
            )
            .add_text_type(
                Language::Urdu,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "رنگ"],
            );

        // Japanese (ja)
        builder = builder
            .add_type(
                Language::Japanese,
                ClipboardType::Text,
                &["テキスト", "txt"],
            ) // Text
            .add_type(
                Language::Japanese,
                ClipboardType::Image,
                &["画像", "イメージ"],
            ) // Image
            .add_type(
                Language::Japanese,
                ClipboardType::Html,
                &["html", "ウェブページ"],
            ) // Html, Webpage
            .add_type(Language::Japanese, ClipboardType::Rtf, &["リッチテキスト"]) // Rich Text
            .add_type(
                Language::Japanese,
                ClipboardType::File,
                &["ファイル", "文書"],
            ) // File, Document
            .add_text_type(
                Language::Japanese,
                ClipboardTextType::Text,
                &["テキスト", "txt"],
            )
            .add_text_type(
                Language::Japanese,
                ClipboardTextType::Link,
                &["リンク", "url"],
            )
            .add_text_type(
                Language::Japanese,
                ClipboardTextType::Hex,
                &["16進数", "カラーコード"],
            )
            .add_text_type(
                Language::Japanese,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "色"],
            );

        // German (de)
        builder = builder
            .add_type(Language::German, ClipboardType::Text, &["text", "txt"])
            .add_type(
                Language::German,
                ClipboardType::Image,
                &["bild", "foto", "grafik"],
            )
            .add_type(Language::German, ClipboardType::Html, &["html", "webseite"])
            .add_type(
                Language::German,
                ClipboardType::Rtf,
                &["rtf", "formatierter text"],
            )
            .add_type(
                Language::German,
                ClipboardType::File,
                &["datei", "dokument", "dateien"],
            )
            .add_text_type(Language::German, ClipboardTextType::Text, &["text", "txt"])
            .add_text_type(
                Language::German,
                ClipboardTextType::Link,
                &["link", "url", "webseite"],
            )
            .add_text_type(
                Language::German,
                ClipboardTextType::Hex,
                &["hex", "farbcode"],
            )
            .add_text_type(
                Language::German,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "farbe"],
            );

        // Korean (ko)
        builder = builder
            .add_type(Language::Korean, ClipboardType::Text, &["텍스트", "txt"]) // Text
            .add_type(Language::Korean, ClipboardType::Image, &["이미지", "사진"]) // Image, picture
            .add_type(Language::Korean, ClipboardType::Html, &["html", "웹페이지"]) // Html, webpage
            .add_type(Language::Korean, ClipboardType::Rtf, &["서식 있는 텍스트"]) // Rich Text
            .add_type(Language::Korean, ClipboardType::File, &["파일", "문서"]) // File, document
            .add_text_type(
                Language::Korean,
                ClipboardTextType::Text,
                &["텍스트", "txt"],
            )
            .add_text_type(Language::Korean, ClipboardTextType::Link, &["링크", "url"])
            .add_text_type(
                Language::Korean,
                ClipboardTextType::Hex,
                &["16진수", "색상 코드"],
            )
            .add_text_type(Language::Korean, ClipboardTextType::Rgb, &["rgb", "rgba"]);

        // Vietnamese (vi)
        builder = builder
            .add_type(
                Language::Vietnamese,
                ClipboardType::Text,
                &["văn bản", "txt"],
            ) // Text
            .add_type(
                Language::Vietnamese,
                ClipboardType::Image,
                &["hình ảnh", "ảnh"],
            ) // Image, picture
            .add_type(
                Language::Vietnamese,
                ClipboardType::Html,
                &["html", "trang web"],
            ) // Html, webpage
            .add_type(
                Language::Vietnamese,
                ClipboardType::Rtf,
                &["văn bản đa dạng thức"],
            ) // Rich Text
            .add_type(
                Language::Vietnamese,
                ClipboardType::File,
                &["tệp", "tài liệu"],
            ) // File, document
            .add_text_type(
                Language::Vietnamese,
                ClipboardTextType::Text,
                &["văn bản", "txt"],
            )
            .add_text_type(
                Language::Vietnamese,
                ClipboardTextType::Link,
                &["liên kết", "url"],
            )
            .add_text_type(
                Language::Vietnamese,
                ClipboardTextType::Hex,
                &["hex", "mã màu hex"],
            )
            .add_text_type(
                Language::Vietnamese,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "màu"],
            );

        // Turkish (tr)
        builder = builder
            .add_type(Language::Turkish, ClipboardType::Text, &["metin", "txt"]) // Text
            .add_type(
                Language::Turkish,
                ClipboardType::Image,
                &["resim", "fotoğraf"],
            ) // Image, picture
            .add_type(
                Language::Turkish,
                ClipboardType::Html,
                &["html", "web sayfası"],
            ) // Html, webpage
            .add_type(Language::Turkish, ClipboardType::Rtf, &["zengin metin"]) // Rich Text
            .add_type(Language::Turkish, ClipboardType::File, &["dosya", "belge"]) // File, document
            .add_text_type(
                Language::Turkish,
                ClipboardTextType::Text,
                &["metin", "txt"],
            )
            .add_text_type(
                Language::Turkish,
                ClipboardTextType::Link,
                &["bağlantı", "url"],
            )
            .add_text_type(
                Language::Turkish,
                ClipboardTextType::Hex,
                &["hex", "onaltılık", "renk kodu"],
            )
            .add_text_type(
                Language::Turkish,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "renk"],
            );

        // Italian (it)
        builder = builder
            .add_type(Language::Italian, ClipboardType::Text, &["testo", "txt"])
            .add_type(
                Language::Italian,
                ClipboardType::Image,
                &["immagine", "foto"],
            )
            .add_type(
                Language::Italian,
                ClipboardType::Html,
                &["html", "pagina web"],
            )
            .add_type(
                Language::Italian,
                ClipboardType::Rtf,
                &["rtf", "testo formattato"],
            )
            .add_type(
                Language::Italian,
                ClipboardType::File,
                &["file", "documento", "files"],
            )
            .add_text_type(
                Language::Italian,
                ClipboardTextType::Text,
                &["testo", "txt"],
            )
            .add_text_type(
                Language::Italian,
                ClipboardTextType::Link,
                &["link", "url", "sito"],
            )
            .add_text_type(
                Language::Italian,
                ClipboardTextType::Hex,
                &["hex", "esadecimale", "codice colore"],
            )
            .add_text_type(
                Language::Italian,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "colore"],
            );

        // Thai (th)
        builder = builder
            .add_type(Language::Thai, ClipboardType::Text, &["ข้อความ", "txt"]) // Text
            .add_type(Language::Thai, ClipboardType::Image, &["รูปภาพ", "ภาพ"]) // Image, picture
            .add_type(Language::Thai, ClipboardType::Html, &["html", "หน้าเว็บ"]) // Html, webpage
            .add_type(Language::Thai, ClipboardType::Rtf, &["ริชเท็กซ์"]) // Rich Text
            .add_type(Language::Thai, ClipboardType::File, &["ไฟล์", "เอกสาร"]) // File, document
            .add_text_type(Language::Thai, ClipboardTextType::Text, &["ข้อความ", "txt"])
            .add_text_type(Language::Thai, ClipboardTextType::Link, &["ลิงก์", "url"])
            .add_text_type(
                Language::Thai,
                ClipboardTextType::Hex,
                &["เลขฐานสิบหก", "รหัสสี"],
            )
            .add_text_type(
                Language::Thai,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "สี"],
            );

        // Polish (pl)
        builder = builder
            .add_type(Language::Polish, ClipboardType::Text, &["tekst", "txt"])
            .add_type(
                Language::Polish,
                ClipboardType::Image,
                &["obraz", "zdjęcie", "foto"],
            )
            .add_type(
                Language::Polish,
                ClipboardType::Html,
                &["html", "strona internetowa"],
            )
            .add_type(
                Language::Polish,
                ClipboardType::Rtf,
                &["rtf", "tekst sformatowany"],
            )
            .add_type(
                Language::Polish,
                ClipboardType::File,
                &["plik", "dokument", "pliki"],
            )
            .add_text_type(Language::Polish, ClipboardTextType::Text, &["tekst", "txt"])
            .add_text_type(
                Language::Polish,
                ClipboardTextType::Link,
                &["link", "url", "strona"],
            )
            .add_text_type(
                Language::Polish,
                ClipboardTextType::Hex,
                &["hex", "szesnastkowy", "kod koloru"],
            )
            .add_text_type(
                Language::Polish,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "kolor"],
            );

        // Dutch (nl)
        builder = builder
            .add_type(Language::Dutch, ClipboardType::Text, &["tekst", "txt"])
            .add_type(
                Language::Dutch,
                ClipboardType::Image,
                &["afbeelding", "foto", "plaatje"],
            )
            .add_type(Language::Dutch, ClipboardType::Html, &["html", "webpagina"])
            .add_type(
                Language::Dutch,
                ClipboardType::Rtf,
                &["rtf", "opgemaakte tekst"],
            )
            .add_type(
                Language::Dutch,
                ClipboardType::File,
                &["bestand", "document", "bestanden"],
            )
            .add_text_type(Language::Dutch, ClipboardTextType::Text, &["tekst", "txt"])
            .add_text_type(
                Language::Dutch,
                ClipboardTextType::Link,
                &["link", "url", "website"],
            )
            .add_text_type(
                Language::Dutch,
                ClipboardTextType::Hex,
                &["hex", "hexadecimaal", "kleurcode"],
            )
            .add_text_type(
                Language::Dutch,
                ClipboardTextType::Rgb,
                &["rgb", "rgba", "kleur"],
            );

        builder.build()
    }
}
