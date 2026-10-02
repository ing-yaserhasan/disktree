//! Arabic reshaping and bidirectional text reordering for LTR text engines.
//!
//! GPUI renders glyphs left-to-right without contextual Arabic shaping or BiDi
//! level resolution. This module shapes Arabic characters into their connected
//! presentation forms (Unicode Arabic Presentation Forms-B) and reverses the
//! visual order of RTL segments so they render naturally in GPUI.

use std::borrow::Cow;

/// Returns true if the character belongs to an RTL/Arabic script block.
#[inline]
fn is_rtl_char(c: char) -> bool {
    matches!(
        c,
        '\u{0590}'..='\u{05FF}'
            | '\u{0600}'..='\u{06FF}'
            | '\u{0750}'..='\u{077F}'
            | '\u{08A0}'..='\u{08FF}'
            | '\u{FB50}'..='\u{FDFF}'
            | '\u{FE70}'..='\u{FEFF}'
    )
}

/// Convert Arabic and bidirectional text into visually reordered, reshaped
/// strings suitable for left-to-right GPUI rendering.
pub fn fix_rtl(text: &str) -> Cow<'_, str> {
    if !text.chars().any(is_rtl_char) {
        return Cow::Borrowed(text);
    }

    let reshaped = arabic_reshaper::arabic_reshape(text);
    let bidi_info = unicode_bidi::BidiInfo::new(&reshaped, None);
    if bidi_info.paragraphs.is_empty() {
        return Cow::Owned(reshaped);
    }

    let mut out = String::with_capacity(reshaped.len());
    for (i, para) in bidi_info.paragraphs.iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        let line = bidi_info.reorder_line(para, para.range.clone());
        out.push_str(&line);
    }
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_ascii_untouched() {
        let input = "Desktop 3.1GiB";
        assert_eq!(fix_rtl(input), Cow::Borrowed(input));
    }

    #[test]
    fn shapes_and_reorders_arabic() {
        let input = "تطبيقاتي";
        let output = fix_rtl(input);
        assert_ne!(output.as_ref(), input);
        assert!(!output.is_empty());
    }
}
