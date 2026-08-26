use std::fs;

use vcc_i18n::dist::{inject, inspect, restore, DistStatus};

fn sample_index() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <script type="module" crossorigin src="/assets/index-066158fc.js"></script>
    <link rel="stylesheet" href="/assets/index-ece89820.css">
  </head>
  <body>
    <div id="root"></div>
  </body>
</html>
"#
}

#[test]
fn inspect_clean_index() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("index.html"), sample_index()).unwrap();
    assert_eq!(inspect(dir.path()).unwrap(), DistStatus::Clean);
}

#[test]
fn inject_replaces_module_script_and_writes_orig() {
    let dir = tempfile::tempdir().unwrap();
    let dist = dir.path();
    fs::write(dist.join("index.html"), sample_index()).unwrap();

    inject(dist, "console.log('vcc-i18n')").unwrap();

    let html = fs::read_to_string(dist.join("index.html")).unwrap();
    assert!(html.contains("data-vcc-i18n=\"1\""));
    assert!(html.contains("name=\"vcc-i18n-index\""));
    assert!(html.contains("/assets/index-066158fc.js"));
    assert!(!html.contains("src=\"/assets/index-066158fc.js\""));
    assert!(html.contains("console.log('vcc-i18n')"));
    assert_eq!(
        fs::read_to_string(dist.join("index.html.vcc-i18n-orig")).unwrap(),
        sample_index()
    );
    assert_eq!(inspect(dist).unwrap(), DistStatus::Ours);
}

#[test]
fn restore_returns_original_bytes() {
    let dir = tempfile::tempdir().unwrap();
    let dist = dir.path();
    fs::write(dist.join("index.html"), sample_index()).unwrap();
    inject(dist, "console.log('vcc-i18n')").unwrap();
    restore(dist).unwrap();
    assert_eq!(
        fs::read_to_string(dist.join("index.html")).unwrap(),
        sample_index()
    );
    assert!(!dist.join("index.html.vcc-i18n-orig").exists());
    assert_eq!(inspect(dist).unwrap(), DistStatus::Clean);
}

#[test]
fn reinject_uses_orig_not_already_patched_html() {
    let dir = tempfile::tempdir().unwrap();
    let dist = dir.path();
    fs::write(dist.join("index.html"), sample_index()).unwrap();
    inject(dist, "first").unwrap();
    inject(dist, "second").unwrap();
    let html = fs::read_to_string(dist.join("index.html")).unwrap();
    assert!(html.contains("second"));
    assert!(!html.contains("first"));
    assert_eq!(html.matches("data-vcc-i18n").count(), 1);
}

#[test]
fn index_bundle_name_reads_meta_after_inject() {
    let dir = tempfile::tempdir().unwrap();
    let dist = dir.path();
    fs::write(dist.join("index.html"), sample_index()).unwrap();
    inject(dist, "console.log('vcc-i18n')").unwrap();
    assert_eq!(
        vcc_i18n::dist::index_bundle_name(dist).as_deref(),
        Some("index-066158fc.js")
    );
}

#[test]
fn inspect_foreign_marker() {
    let dir = tempfile::tempdir().unwrap();
    let html = sample_index().replace(
        "</body>",
        "<script name=\"vcc-auto-translate\"></script></body>",
    );
    fs::write(dir.path().join("index.html"), html).unwrap();
    assert_eq!(inspect(dir.path()).unwrap(), DistStatus::Foreign);
}
