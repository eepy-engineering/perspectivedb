use unicode_normalization::UnicodeNormalization;

/// Slugify core: ASCII [a-z0-9-], lowercase, normalized, collapsed separators.
fn slug_core(input: &str) -> String {
    let normalized = input.nfkd().collect::<String>();
    let mut out = String::with_capacity(normalized.len());
    let mut last_dash = false;

    for ch in normalized.chars() {
        let c = ch.to_ascii_lowercase();
        match c {
            'a'..='z' | '0'..='9' => {
                out.push(c);
                last_dash = false;
            }
            '_' | '-' | ' ' | '\t' | '\n' | '\r' | '\u{00A0}' => {
                if !last_dash {
                    out.push('-');
                    last_dash = true;
                }
            }
            _ if c.is_ascii() => {
                if !last_dash {
                    out.push('-');
                    last_dash = true;
                }
            }
            _ => {} // drop non-ASCII after NFKD
        }
    }

    let mut s = out.trim_matches('-').trim_matches('.').trim().to_string();
    if s.is_empty() {
        s = "unnamed".to_string();
    }

    const RESERVED: &[&str] = &[
        "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
        "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
    ];
    if RESERVED.contains(&s.as_str()) {
        s.push('_');
    }
    while s.ends_with('.') || s.ends_with(' ') {
        s.pop();
    }
    if s.is_empty() {
        s = "unnamed".to_string();
    }
    s
}

/// Deterministic, path-safe, no-overlap slug.
/// Always appends a 10-hex-char BLAKE3 hash suffix for uniqueness.
pub fn path_safe_slug(input: &str, max_len: usize) -> String {
    let mut base = slug_core(input);

    let hash = blake3::hash(input.as_bytes());
    let suffix = format!("--{}", &hash.to_hex()[..10]);

    // Truncate base to leave room for suffix
    let keep = max_len.saturating_sub(suffix.len());
    if base.len() > keep {
        base.truncate(keep);
        while base.ends_with('-') && !base.is_empty() {
            base.pop();
        }
        if base.is_empty() {
            base = "unnamed".to_string();
            if base.len() > keep {
                base.truncate(keep);
            }
        }
    }

    let mut out = base;
    out.push_str(&suffix);

    if out.len() > max_len {
        out.truncate(max_len);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::path_safe_slug;

    #[test]
    fn basics() {
        assert_eq!(
            path_safe_slug("Hello, World!", 64),
            "hello-world--288a86a79f"
        );
        assert_eq!(path_safe_slug("résumé.pdf", 64), "resume-pdf--200483fb55");
        assert_eq!(path_safe_slug("NUL", 64), "nul_--1233be0930");
    }
}
