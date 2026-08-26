#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Catalog {
    ZhCn,
    ZhTw,
}

impl Catalog {
    pub fn id(self) -> &'static str {
        match self {
            Catalog::ZhCn => "zh-CN",
            Catalog::ZhTw => "zh-TW",
        }
    }

    pub fn english_name(self) -> &'static str {
        match self {
            Catalog::ZhCn => "Simplified Chinese",
            Catalog::ZhTw => "Traditional Chinese",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub catalog: Catalog,
    pub matched_tag: Option<String>,
}

pub fn propose_catalog(tags: &[&str]) -> Proposal {
    for tag in tags {
        if let Some(catalog) = classify(tag) {
            return Proposal {
                catalog,
                matched_tag: Some((*tag).to_string()),
            };
        }
    }
    Proposal {
        catalog: Catalog::ZhCn,
        matched_tag: None,
    }
}

fn classify(tag: &str) -> Option<Catalog> {
    let t = tag.trim().to_ascii_lowercase().replace('_', "-");
    if t.is_empty() {
        return None;
    }
    if t.starts_with("zh-hant")
        || t == "zh-tw"
        || t.starts_with("zh-tw-")
        || t == "zh-hk"
        || t.starts_with("zh-hk-")
        || t == "zh-mo"
        || t.starts_with("zh-mo-")
    {
        return Some(Catalog::ZhTw);
    }
    if t == "zh" || t.starts_with("zh-") {
        return Some(Catalog::ZhCn);
    }
    None
}
