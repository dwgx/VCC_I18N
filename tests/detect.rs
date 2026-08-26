use std::fs;

use vcc_i18n::detect::detect;
use vcc_i18n::dist::DistStatus;

#[test]
fn reads_build_info_from_vcc_dir() {
    let dir = tempfile::tempdir().unwrap();
    let dist = dir.path().join("WebApp").join("Dist");
    fs::create_dir_all(&dist).unwrap();
    fs::write(
        dist.join("buildInfo.json"),
        r#"{"version":{"Major":2,"Minor":4,"Patch":5},"branch":"release/2.4.5"}"#,
    )
    .unwrap();
    fs::write(
        dist.join("index.html"),
        r#"<script type="module" src="/assets/index-abc.js"></script>"#,
    )
    .unwrap();

    let detection = detect(Some(dir.path())).unwrap();
    let vcc = detection.vcc.expect("vcc");
    assert_eq!(vcc.version, "2.4.5");
    assert_eq!(vcc.branch, "release/2.4.5");
    assert_eq!(vcc.dist_status, DistStatus::Clean);
}
