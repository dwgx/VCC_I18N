use std::fs;
use std::path::Path;

use anyhow::{bail, Context, Result};

const INDEX: &str = "index.html";
const ORIG: &str = "index.html.vcc-i18n-orig";
const OURS: &str = "data-vcc-i18n";
const FOREIGN: &str = "vcc-auto-translate";

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

    let source = if orig.exists() {
        fs::read_to_string(&orig).with_context(|| format!("read {}", orig.display()))?
    } else {
        let current =
            fs::read_to_string(&index).with_context(|| format!("read {}", index.display()))?;
        match inspect(dist)? {
            DistStatus::Foreign => {
                bail!("Dist is patched by another tool; restore that first")
            }
            DistStatus::Ours => bail!("patched index.html without {} ", ORIG),
            DistStatus::Missing => bail!("index.html is missing"),
            DistStatus::Clean => current,
        }
    };

    if !orig.exists() {
        fs::write(&orig, &source).with_context(|| format!("write {}", orig.display()))?;
    }

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
