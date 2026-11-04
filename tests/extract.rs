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

#[test]
fn extracts_jsx_prop_strings_and_skips_class_names() {
    let js = r#"f(Xt,{children:"Open External Link",className:"fui-Dialog"});f(W,{title:"Open Changelog"});const Qpe="This project is using Git packages, which have been phased out as of today. Please migrate your project to use the new VCC package management.";"#;
    let got = vcc_i18n::catalog::extract_jsx_ui_strings(js);
    assert!(got.contains(&"Open External Link".to_string()));
    assert!(got.contains(&"Open Changelog".to_string()));
    assert!(got.iter().any(|s| s.contains("Git packages")));
    assert!(!got.iter().any(|s| s.contains("fui-")));
}

#[test]
fn missing_from_bundle_reports_untranslated_jsx_strings() {
    let js = r#"f(W,{children:"Open External Link"});f(W,{children:"Projects"});"#;
    let missing = vcc_i18n::catalog::missing_from_bundle(js, r#"{"Projects":"项目"}"#);
    assert_eq!(missing, vec!["Open External Link".to_string()]);
}

#[test]
fn coverage_reports_missing_and_unused_keys() {
    let js = r#"f(W,{children:"Open External Link"});f(W,{children:"Projects"});"#;
    let coverage = vcc_i18n::catalog::coverage(js, r#"{"Projects":"项目","Stale Key":"旧"}"#);
    assert_eq!(coverage.missing, vec!["Open External Link".to_string()]);
    assert_eq!(coverage.unused, vec!["Stale Key".to_string()]);
}

#[test]
fn extractor_reads_alt_and_aria_label_props() {
    let js = r#"f("img",{alt:"VCC Logo"});f("button",{"aria-label":"Resize column"});"#;
    let got = vcc_i18n::catalog::extract_jsx_ui_strings(js);
    assert!(got.contains(&"VCC Logo".to_string()));
    assert!(got.contains(&"Resize column".to_string()));
}

#[test]
fn patch_code_rewrites_dayjs_relative_time() {
    let src = injector_source(Catalog::ZhCn);
    assert!(src.contains(
        r#"{future:"in %s",past:"%s ago",s:"a few seconds",m:"a minute",mm:"%d minutes",h:"an hour",hh:"%d hours",d:"a day",dd:"%d days",M:"a month",MM:"%d months",y:"a year",yy:"%d years"}"#
    ));
    assert!(src.contains(
        r#"{future:"%s后",past:"%s前",s:"几秒",m:"1 分钟",mm:"%d 分钟",h:"1 小时",hh:"%d 小时",d:"1 天",dd:"%d 天",M:"1 个月",MM:"%d 个月",y:"1 年",yy:"%d 年"}"#
    ));
    assert!(src.contains(".split(DAYJS_EN_RELATIVE_TIME)"));
    assert!(src.contains(".join(DAYJS_ZH_RELATIVE_TIME)"));
}

#[test]
fn jsx_proxy_does_not_translate_key_argument() {
    let src = injector_source(Catalog::ZhCn);
    assert!(src.contains("function proxy(fn, translateChildren)"));
    assert!(src.contains("globalThis.__vcc_i18n_proxy__(${obj}.${prop},false)"));
    assert!(src.contains("globalThis.__vcc_i18n_proxy__(${obj}.createElement,true)"));
    let compact: String = src.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(compact.contains(
        r#""children","content","title","placeholder","label","text","header","description","alt","aria-label""#
    ));
}
