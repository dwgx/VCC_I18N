use crate::locale::Catalog;

pub fn json_for(catalog: Catalog) -> &'static str {
    match catalog {
        Catalog::ZhCn => include_str!("../catalogs/zh-CN.json"),
        Catalog::ZhTw => "{}",
    }
}

pub fn injector_source(catalog: Catalog) -> String {
    include_str!("../assets/inject.js").replace("__CATALOG_JSON__", json_for(catalog))
}

pub fn extract_ui_strings(js: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for raw in quoted_strings(js) {
        if !is_ui_string(&raw) {
            continue;
        }
        if seen.insert(raw.clone()) {
            out.push(raw);
        }
    }
    out
}

fn quoted_strings(js: &str) -> Vec<String> {
    let bytes = js.as_bytes();
    let mut i = 0;
    let mut out = Vec::new();
    while i < bytes.len() {
        let q = bytes[i];
        if q == b'"' || q == b'\'' {
            i += 1;
            let mut buf = String::new();
            let mut escaped = false;
            while i < bytes.len() {
                let c = bytes[i];
                i += 1;
                if escaped {
                    match c {
                        b'n' => buf.push('\n'),
                        b't' => buf.push('\t'),
                        b'r' => buf.push('\r'),
                        other => buf.push(other as char),
                    }
                    escaped = false;
                    continue;
                }
                if c == b'\\' {
                    escaped = true;
                    continue;
                }
                if c == q {
                    break;
                }
                if c == b'\n' {
                    buf.clear();
                    break;
                }
                buf.push(c as char);
            }
            if !buf.is_empty() {
                out.push(buf);
            }
            continue;
        }
        i += 1;
    }
    out
}

fn is_ui_string(s: &str) -> bool {
    let t = s.trim();
    if t.len() < 3 || t.len() > 140 {
        return false;
    }
    if t.starts_with("http://") || t.starts_with("https://") {
        return false;
    }
    if t.contains('\n') || t.contains('{') || t.contains('}') {
        return false;
    }
    if t.contains(".json") || t.contains(".js") || t.contains('/') && t.contains('.') {
        return false;
    }
    let letters = t.chars().filter(|c| c.is_ascii_alphabetic()).count();
    if letters < 3 {
        return false;
    }
    let first = t.chars().next().unwrap();
    first.is_ascii_uppercase()
}
