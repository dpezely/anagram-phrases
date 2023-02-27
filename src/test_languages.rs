use crate::languages::{self, UPCASE, SHORT, Language, Region};

#[test]
fn filters() {
    let empty: Vec<&str> = vec![];
    assert!(languages::filter("a", &empty, &empty, true, true));
    assert!(languages::filter("I", &empty, &empty, true, true));
    assert!(languages::filter("Foo", &empty, &empty, true, true));
    assert!(!languages::filter("a", &empty, &empty, false, true));
    assert!(!languages::filter("a", &empty, &empty, false, false));
    assert!(!languages::filter("I", &empty, &empty, false, false));
    assert!(languages::filter("I", &empty, &empty, true, false));

    // EN == English:
    // Note: for English, "I" is in both lists:
    let short = SHORT.get(&Language::EN).unwrap();
    let upcase = UPCASE.get(&Language::EN).unwrap();
    assert!(languages::filter("n", short, upcase, false, false));
    assert!(languages::filter("Rust", short, upcase, false, false));
    assert!(!languages::filter("a", short, upcase, false, false));
    assert!(!languages::filter("I", short, upcase, false, false));

    // ES == Español, Spanish
    let short = SHORT.get(&Language::ES).unwrap();
    assert!(!languages::filter("y", short, upcase, false, false));
}

#[test]
fn lang() {
    // negative tests:
    assert_eq!(languages::parse_lang(" en"), (Language::Any, Region::Any));
    assert_eq!(languages::parse_lang("en "), (Language::Any, Region::Any));
    assert_eq!(languages::parse_lang(" EN"), (Language::Any, Region::Any));
    assert_eq!(languages::parse_lang("EN "), (Language::Any, Region::Any));
    assert_eq!(languages::parse_lang(" EN "), (Language::Any, Region::Any));
    // positive tests:
    assert_eq!(languages::parse_lang("en"), (Language::EN, Region::Any));
    assert_eq!(languages::parse_lang("EN"), (Language::EN, Region::Any));
    assert_eq!(languages::parse_lang("en-us"), (Language::EN, Region::US));
    assert_eq!(languages::parse_lang("en_US"), (Language::EN, Region::US));
    assert_eq!(languages::parse_lang("en_GB"), (Language::EN, Region::GB));
    assert_eq!(languages::parse_lang("en_UK"), (Language::EN, Region::GB));
    // HTTP header Accept-Language weighting gets silently ignored:
    assert_eq!(languages::parse_lang("en_CA;q=0.9"), (Language::EN, Region::CA));
    assert_eq!(languages::parse_lang("en-CA;q=0.7"), (Language::EN, Region::CA));
}
