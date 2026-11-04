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

const JSX_KEYS: &[&str] = &[
    "children",
    "title",
    "content",
    "placeholder",
    "label",
    "text",
    "header",
    "description",
    "alt",
    "aria-label",
];

pub fn extract_jsx_ui_strings(js: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (start, raw) in quoted_with_index(js) {
        if !is_jsx_ui_context(js, start, &raw) {
            continue;
        }
        if is_noise(&raw) || !is_catalog_candidate(&raw) {
            continue;
        }
        if seen.insert(raw.clone()) {
            out.push(raw);
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverage {
    pub missing: Vec<String>,
    pub unused: Vec<String>,
}

pub fn coverage(js: &str, catalog_json: &str) -> Coverage {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(catalog_json) else {
        return Coverage {
            missing: Vec::new(),
            unused: Vec::new(),
        };
    };
    let Some(map) = value.as_object() else {
        return Coverage {
            missing: Vec::new(),
            unused: Vec::new(),
        };
    };
    let found = extract_jsx_ui_strings(js);
    let found_set: std::collections::BTreeSet<&str> = found.iter().map(String::as_str).collect();
    let mut missing: Vec<String> = found
        .iter()
        .filter(|key| !map.contains_key(*key))
        .cloned()
        .collect();
    let mut unused: Vec<String> = map
        .keys()
        .filter(|key| !found_set.contains(key.as_str()))
        .cloned()
        .collect();
    missing.sort();
    unused.sort();
    Coverage { missing, unused }
}

pub fn missing_from_bundle(js: &str, catalog_json: &str) -> Vec<String> {
    coverage(js, catalog_json).missing
}

fn is_jsx_ui_context(js: &str, quote_start: usize, raw: &str) -> bool {
    let from = quote_start.saturating_sub(32);
    let prefix = &js[from..quote_start];
    if JSX_KEYS.iter().any(|key| {
        prefix.ends_with(&format!("{key}:"))
            || prefix.ends_with(&format!("{key}:["))
            || prefix.ends_with(&format!("\"{key}\":"))
    }) {
        return true;
    }
    if prefix.ends_with(',') && (raw.starts_with(' ') || raw.starts_with('•')) {
        return true;
    }
    prefix.ends_with('=')
        && raw.len() > 40
        && raw.chars().next().is_some_and(|c| c.is_ascii_uppercase())
}

fn is_catalog_candidate(s: &str) -> bool {
    if is_ui_string(s) {
        return true;
    }
    let t = s.trim();
    if t.len() < 3 || t.len() > 200 {
        return false;
    }
    if t.starts_with("http://") || t.starts_with("https://") {
        return false;
    }
    if s.starts_with(' ') || s.starts_with('•') {
        return t.chars().any(|c| c.is_ascii_alphabetic());
    }
    let first = t.chars().next().unwrap();
    first.is_ascii_uppercase()
        && t.contains(' ')
        && t.chars().filter(|c| c.is_ascii_alphabetic()).count() >= 3
}

fn is_noise(s: &str) -> bool {
    let t = s.trim();
    t == "div"
        || t == "span"
        || t.contains("fui-")
        || t.contains("__")
        || t.contains("Sentry")
        || t.contains("React.")
        || t.contains("Main UI thread")
        || t.contains("ModuleSymb")
        || t.contains("props on")
}

fn quoted_strings(js: &str) -> Vec<String> {
    quoted_with_index(js).into_iter().map(|(_, s)| s).collect()
}

fn quoted_with_index(js: &str) -> Vec<(usize, String)> {
    let chars: Vec<(usize, char)> = js.char_indices().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let (quote_start, q) = chars[i];
        if q == '"' || q == '\'' {
            i += 1;
            let mut buf = String::new();
            let mut escaped = false;
            while i < chars.len() {
                let (_, c) = chars[i];
                i += 1;
                if escaped {
                    match c {
                        'n' => buf.push('\n'),
                        't' => buf.push('\t'),
                        'r' => buf.push('\r'),
                        other => buf.push(other),
                    }
                    escaped = false;
                    continue;
                }
                if c == '\\' {
                    escaped = true;
                    continue;
                }
                if c == q {
                    break;
                }
                if c == '\n' {
                    buf.clear();
                    break;
                }
                buf.push(c);
            }
            if !buf.is_empty() {
                out.push((quote_start, buf));
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
