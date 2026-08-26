use vcc_i18n::locale::{propose_catalog, Catalog};

#[test]
fn hans_in_list_proposes_zh_cn_even_if_not_first() {
    let p = propose_catalog(&["en-US", "zh-Hans-CN", "ja"]);
    assert_eq!(p.catalog, Catalog::ZhCn);
    assert_eq!(p.matched_tag.as_deref(), Some("zh-Hans-CN"));
}

#[test]
fn hant_proposes_zh_tw() {
    let p = propose_catalog(&["zh-Hant-TW"]);
    assert_eq!(p.catalog, Catalog::ZhTw);
    assert_eq!(p.matched_tag.as_deref(), Some("zh-Hant-TW"));
}

#[test]
fn zh_tw_tag_proposes_zh_tw() {
    let p = propose_catalog(&["zh-TW"]);
    assert_eq!(p.catalog, Catalog::ZhTw);
}

#[test]
fn zh_hk_proposes_zh_tw() {
    let p = propose_catalog(&["zh-HK"]);
    assert_eq!(p.catalog, Catalog::ZhTw);
}

#[test]
fn bare_zh_proposes_zh_cn() {
    let p = propose_catalog(&["zh"]);
    assert_eq!(p.catalog, Catalog::ZhCn);
}

#[test]
fn no_chinese_still_proposes_zh_cn() {
    let p = propose_catalog(&["en-US", "ja"]);
    assert_eq!(p.catalog, Catalog::ZhCn);
    assert_eq!(p.matched_tag, None);
}

#[test]
fn first_chinese_in_list_wins() {
    let p = propose_catalog(&["zh-Hant", "zh-Hans"]);
    assert_eq!(p.catalog, Catalog::ZhTw);
}
