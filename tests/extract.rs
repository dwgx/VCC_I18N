use vcc_i18n::catalog::{extract_ui_strings, injector_source};
use vcc_i18n::locale::Catalog;

#[test]
fn zh_cn_catalog_is_json_object() {
    let value: serde_json::Value =
        serde_json::from_str(vcc_i18n::catalog::json_for(Catalog::ZhCn)).unwrap();
    assert!(value.as_object().unwrap().len() > 50);
    assert_eq!(value["Projects"], "项目");
}

#[test]
fn injector_embeds_catalog_json() {
    let src = injector_source(Catalog::ZhCn);
    assert!(src.contains("const CATALOG = {"));
    assert!(!src.contains("__CATALOG_JSON__"));
    assert!(src.contains("__vcc_i18n_proxy__"));
}

#[test]
fn extracts_capitalized_phrases() {
    let js = r#"n("Create New Project"); t('Open Project');"#;
    let got = extract_ui_strings(js);
    assert!(got.contains(&"Create New Project".to_string()));
    assert!(got.contains(&"Open Project".to_string()));
}

#[test]
fn skips_urls_and_short_noise() {
    let js = r#""https://assets.vrchat.com/x"; "OK"; "a"; "settings.json";"#;
    let got = extract_ui_strings(js);
    assert!(!got.iter().any(|s| s.starts_with("http")));
    assert!(!got
        .iter()
        .any(|s| s == "OK" || s == "a" || s == "settings.json"));
}
