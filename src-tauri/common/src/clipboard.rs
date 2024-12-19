use crate::{constants::MAX_TEXT_PREVIEW, types::orm_query::ClipboardWithRelations};

pub fn trim_clipboard_data(
    mut clipboards: Vec<ClipboardWithRelations>,
) -> Vec<ClipboardWithRelations> {
    for clipboard in &mut clipboards {
        // Trim text content
        if let Some(text) = &mut clipboard.text {
            text.data = truncate_text(&text.data, MAX_TEXT_PREVIEW);
        }

        // Trim HTML content
        if let Some(html) = &mut clipboard.html {
            html.data = truncate_text(&html.data, MAX_TEXT_PREVIEW);
        }

        // Trim RTF content
        if let Some(rtf) = &mut clipboard.rtf {
            rtf.data = truncate_text(&rtf.data, MAX_TEXT_PREVIEW);
        }

        // Remove image binary data but keep metadata
        if let Some(image) = &mut clipboard.image {
            image.data = Vec::new(); // Clear binary data
                                     // Thumbnail, dimensions, size etc are preserved
        }

        // Clear file binary data but keep metadata
        for file in &mut clipboard.files {
            file.data = Vec::new(); // Clear binary data
                                    // Name, extension, size etc are preserved
        }
    }

    clipboards
}

fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length])
    }
}
