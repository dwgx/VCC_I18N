use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

const INDEX: &str = "index.html";
const ORIG: &str = "index.html.vcc-i18n-orig";
const OURS: &str = "data-vcc-i18n";
const FOREIGN: &str = "vcc-auto-translate";
const TITLE_STYLE: &str = r#"<style data-vcc-i18n="1">.fui-Title1,.fui-Title2,.fui-Title3,.fui-Subtitle1,.fui-Subtitle2{white-space:nowrap}</style>"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistStatus {
    Missing,
    Clean,
    Ours,
    Foreign,
}

impl DistStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            DistStatus::Missing => "missing",
            DistStatus::Clean => "clean",
            DistStatus::Ours => "vcc-i18n installed",
            DistStatus::Foreign => "patched by another tool",
        }
    }
}

pub fn index_bundle_name(dist: &Path) -> Option<String> {
    let src = index_src_from_dist(dist).ok()?;
    Some(src.trim_start_matches('/').rsplit('/').next()?.to_string())
}

pub fn index_bundle_path(dist: &Path) -> Result<PathBuf> {
    let src = index_src_from_dist(dist)?;
    let rel = src
        .trim_start_matches('/')
        .replace('/', std::path::MAIN_SEPARATOR_STR);
    Ok(dist.join(rel))
}

fn index_src_from_dist(dist: &Path) -> Result<String> {
    let html_path = dist.join(INDEX);
    let html =
        fs::read_to_string(&html_path).with_context(|| format!("read {}", html_path.display()))?;
    if let Ok((_, _, src)) = find_index_script(&html) {
        return Ok(src);
    }
    if let Some(src) = meta_index_src(&html) {
        return Ok(src);
    }
    bail!("could not find /assets/index-*.js")
}

fn meta_index_src(html: &str) -> Option<String> {
    let key = "name=\"vcc-i18n-index\"";
    let i = html.find(key)?;
    let after = &html[i + key.len()..];
    let content_key = "content=\"";
    let j = after.find(content_key)?;
    let rest = &after[j + content_key.len()..];
    let k = rest.find('"')?;
    Some(rest[..k].to_string())
}

pub fn inspect(dist: &Path) -> Result<DistStatus> {
    let index = dist.join(INDEX);
    if !index.exists() {
        return Ok(DistStatus::Missing);
    }
    let html = fs::read_to_string(&index).with_context(|| format!("read {}", index.display()))?;
    if html.contains(OURS) {
        return Ok(DistStatus::Ours);
    }
    if html.contains(FOREIGN) {
        return Ok(DistStatus::Foreign);
    }
    Ok(DistStatus::Clean)
}

pub fn inject(dist: &Path, script_body: &str) -> Result<()> {
    let index = dist.join(INDEX);
    let orig = dist.join(ORIG);
    if !index.exists() {
        bail!("index.html is missing in {}", dist.display());
    }

    let source = match inspect(dist)? {
        DistStatus::Ours => {
            if !orig.exists() {
                bail!("patched index.html without {ORIG}");
            }
            fs::read_to_string(&orig).with_context(|| format!("read {}", orig.display()))?
        }
        DistStatus::Clean => {
            let current =
                fs::read_to_string(&index).with_context(|| format!("read {}", index.display()))?;
            // A VCC update replaces index.html but leaves our backup behind. Record the
            // current bytes so the stale backup cannot resurrect the previous bundle.
            fs::write(&orig, &current).with_context(|| format!("write {}", orig.display()))?;
            current
        }
        DistStatus::Foreign => bail!("Dist is patched by another tool; restore that first"),
        DistStatus::Missing => bail!("index.html is missing"),
    };

    let patched = patch_html(&source, script_body)?;
    fs::write(&index, patched).with_context(|| format!("write {}", index.display()))?;
    Ok(())
}

pub fn restore(dist: &Path) -> Result<()> {
    let index = dist.join(INDEX);
    let orig = dist.join(ORIG);
    if !orig.exists() {
        bail!("no {} to restore in {}", ORIG, dist.display());
    }
    let source = fs::read_to_string(&orig).with_context(|| format!("read {}", orig.display()))?;
    fs::write(&index, source).with_context(|| format!("write {}", index.display()))?;
    fs::remove_file(&orig).with_context(|| format!("remove {}", orig.display()))?;
    Ok(())
}

fn patch_html(html: &str, script_body: &str) -> Result<String> {
    let (tag_start, tag_end, src) = find_index_script(html)?;
    let meta = format!("<meta name=\"vcc-i18n-index\" content=\"{}\">", src);
    let mut out = String::with_capacity(html.len() + script_body.len() + 128);
    out.push_str(&html[..tag_start]);
    out.push_str(&meta);
    out.push_str(&html[tag_end..]);

    let body = script_body.replace("</script>", "<\\/script>");
    let injected = format!("\n<script type=\"module\" data-vcc-i18n=\"1\">\n{body}\n</script>\n");
    if let Some(idx) = out.to_ascii_lowercase().rfind("</body>") {
        out.insert_str(idx, &injected);
    } else {
        out.push_str(&injected);
    }

    // CJK has a break opportunity between every glyph, so a flex item such as
    // .fui-Title1 shrinks to min-content (one glyph per line). Keep titles on one line.
    if let Some(idx) = out.to_ascii_lowercase().rfind("</head>") {
        out.insert_str(idx, TITLE_STYLE);
    } else {
        out.insert_str(0, TITLE_STYLE);
    }
    Ok(out)
}

fn find_index_script(html: &str) -> Result<(usize, usize, String)> {
    let mut search_from = 0;
    while let Some(rel) = html[search_from..].find("<script") {
        let start = search_from + rel;
        let after = &html[start..];
        let Some(end_rel) = after.find("</script>") else {
            break;
        };
        let end = start + end_rel + "</script>".len();
        let tag = &html[start..end];
        if let Some(src) = script_src(tag) {
            if src.starts_with("/assets/index-") && src.ends_with(".js") {
                return Ok((start, end, src));
            }
        }
        search_from = end;
    }
    bail!("could not find /assets/index-*.js module script")
}

fn script_src(tag: &str) -> Option<String> {
    let key = "src=\"";
    let i = tag.find(key)?;
    let rest = &tag[i + key.len()..];
    let j = rest.find('"')?;
    Some(rest[..j].to_string())
}
