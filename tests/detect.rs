use std::fs;

use vcc_i18n::detect::{detect, discover_installs, looks_like_vcc};
use vcc_i18n::dist::DistStatus;

fn write_vcc_layout(root: &std::path::Path, build_info: &str, index_html: &str, beta: bool) {
    let dist = root.join("WebApp").join("Dist").join("assets");
    fs::create_dir_all(&dist).unwrap();
    fs::write(
        root.join("WebApp").join("Dist").join("buildInfo.json"),
        build_info,
    )
    .unwrap();
    fs::write(
        root.join("WebApp").join("Dist").join("index.html"),
        index_html,
    )
    .unwrap();
    let exe = if beta {
        "CreatorCompanionBeta.exe"
    } else {
        "CreatorCompanion.exe"
    };
    fs::write(root.join(exe), b"").unwrap();
}

#[test]
fn reads_build_info_from_vcc_dir() {
    let dir = tempfile::tempdir().unwrap();
    write_vcc_layout(
        dir.path(),
        r#"{"version":{"Major":2,"Minor":4,"Patch":5},"branch":"release/2.4.5"}"#,
        r#"<script type="module" src="/assets/index-abc.js"></script>"#,
        false,
    );

    let detection = detect(Some(dir.path())).unwrap();
    let vcc = detection.vcc.expect("vcc");
    assert_eq!(vcc.version, "2.4.5");
    assert_eq!(vcc.branch, "release/2.4.5");
    assert!(!vcc.portable);
    assert_eq!(vcc.dist_status, DistStatus::Clean);
    assert_eq!(vcc.dist_bundle.as_deref(), Some("index-abc.js"));
}

#[test]
fn reads_prerelease_portable_build_info() {
    let dir = tempfile::tempdir().unwrap();
    write_vcc_layout(
        dir.path(),
        r#"{"version":{"Major":2,"Minor":5,"Patch":0,"PreRelease":"beta.2","IsPreRelease":true},"branch":"release/2.5.0","portable":true}"#,
        r#"<script type="module" crossorigin src="/assets/index-2254061d.js"></script>"#,
        true,
    );

    let detection = detect(Some(dir.path())).unwrap();
    let vcc = detection.vcc.expect("vcc");
    assert_eq!(vcc.version, "2.5.0-beta.2");
    assert_eq!(vcc.branch, "release/2.5.0");
    assert!(vcc.portable);
    assert_eq!(vcc.dist_bundle.as_deref(), Some("index-2254061d.js"));
}

#[test]
fn discovers_portable_vcc_under_a_root() {
    let root = tempfile::tempdir().unwrap();
    let portable = root.path().join("web_vcc_2.5.0-beta.2_Release_Portable");
    write_vcc_layout(
        &portable,
        r#"{"version":{"Major":2,"Minor":5,"Patch":0,"PreRelease":"beta.2"},"portable":true}"#,
        r#"<script type="module" src="/assets/index-2254061d.js"></script>"#,
        true,
    );
    fs::create_dir_all(root.path().join("Bandizip")).unwrap();

    assert!(looks_like_vcc(&portable));
    let found = discover_installs(&[root.path().to_path_buf()]);
    assert_eq!(found, vec![portable]);
}
