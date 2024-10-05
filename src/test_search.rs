use std::path::PathBuf;
use std::string::ToString;
use std::sync::LazyLock;

use crate::config::Config;
use crate::languages::Language;
use crate::search::Search;
use crate::words;

/// Tests were written specifically against the American flavour of English.
static DEFAULT_DICT_FILES: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| vec![PathBuf::from("/usr/share/dict/american-english")]);

#[test]
fn delaware_two_words() {
    assert!(std::fs::exists(&DEFAULT_DICT_FILES[0]).expect("Word list file not found"));

    let max_phrase_words = 2;
    let input_phrase = &["delaware".to_string()];

    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["leeward"]],
        vec![vec!["alder"], vec!["awe"]],
        vec![vec!["ale", "lea"], vec!["wader"]],
        vec![vec!["are", "ear", "era"], vec!["waled"]],
        vec![vec!["area"], vec!["lewd", "weld"]],
        vec![vec!["aw"], vec!["dealer", "leader"]],
        vec![vec!["award"], vec!["eel", "lee"]],
        vec![vec!["aware"], vec!["led"]],
        vec![vec!["awed", "wade"], vec!["earl", "real"]],
        vec![vec!["dale", "deal", "lade", "lead"], vec!["ware", "wear"]],
        vec![vec!["dare", "dear", "read"], vec!["wale", "weal"]],
    ];
    let config = Config {
        lang: Language::EN,
        dict_file_paths: DEFAULT_DICT_FILES.to_vec(),
        max_phrase_words,
        include_short: false,
        ..Config::default()
    };
    let search = Search::query(input_phrase, &[], &config).unwrap();
    let (dict, _singles) = words::load_and_select(
        &config,
        &search.pattern,
        &search.essential,
        &search.primes_product,
        &[],
    )
    .unwrap();
    let cache = words::Cache::init(&dict);
    let mut builder = search.add_cache(&cache);
    let mut anagrams = builder.brute_force();
    anagrams.sort_unstable_by(
        |a, b| {
            if a[0] == b[0] {
                a[1].cmp(b[1])
            } else {
                a[0].cmp(b[0])
            }
        },
    );
    dbg!(&[expected.len(), anagrams.len()]);
    assert_eq!(expected, anagrams, "expected vs actual");
}

#[test]
fn delaware_three_words() {
    assert!(std::fs::exists(&DEFAULT_DICT_FILES[0]).expect("Word list file not found"));

    let max_phrase_words = 3;
    let input_phrase: Vec<String> =
        "delaware".split(' ').map(ToString::to_string).collect();

    // Keep sorted.  Sequence may differ from CLI output.
    // TODO migrate `expected` to .json file, and compare files.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["a"], vec!["lewder", "welder"]],
        vec![vec!["a"], vec!["alder"], vec!["we"]],
        vec![vec!["a"], vec!["ale", "lea"], vec!["drew"]],
        vec![vec!["a"], vec!["are", "ear", "era"], vec!["lewd", "weld"]],
        vec![vec!["a"], vec!["aw"], vec!["elder"]],
        vec![vec!["a"], vec!["awl", "law"], vec!["deer", "reed"]],
        vec![vec!["a"], vec!["dew", "we'd", "wed"], vec!["earl", "real"]],
        vec![vec!["a"], vec!["draw", "ward"], vec!["eel", "lee"]],
        vec![vec!["a"], vec!["ewe", "wee"], vec!["lard"]],
        vec![vec!["a"], vec!["ewer", "we're", "weer", "were"], vec!["lad"]],
        vec![vec!["a"], vec!["led"], vec!["ware", "wear"]],
        vec![vec!["a"], vec!["leer", "reel"], vec!["wad"]],
        vec![vec!["a"], vec!["leeward"]],
        vec![vec!["a"], vec!["re"], vec!["waled"]],
        vec![vec!["a"], vec!["red"], vec!["wale", "weal"]],
        vec![vec!["ad"], vec!["aw"], vec!["leer", "reel"]],
        vec![vec!["ad"], vec!["awl", "law"], vec!["e'er", "ere"]],
        vec![vec!["ad"], vec!["earl", "real"], vec!["we"]],
        vec![vec!["ad"], vec!["eel", "lee"], vec!["raw", "war"]],
        vec![vec!["ad"], vec!["ewer", "we're", "weer", "were"], vec!["la"]],
        vec![vec!["ad"], vec!["re"], vec!["wale", "weal"]],
        vec![vec!["alder"], vec!["awe"]],
        vec![vec!["ale", "lea"], vec!["aw"], vec!["red"]],
        vec![vec!["ale", "lea"], vec!["ed"], vec!["raw", "war"]],
        vec![vec!["ale", "lea"], vec!["re"], vec!["wad"]],
        vec![vec!["ale", "lea"], vec!["wader"]],
        vec![vec!["are", "ear", "era"], vec!["aw"], vec!["led"]],
        vec![vec!["are", "ear", "era"], vec!["awl", "law"], vec!["ed"]],
        vec![vec!["are", "ear", "era"], vec!["dew", "we'd", "wed"], vec!["la"]],
        vec![vec!["are", "ear", "era"], vec!["lad"], vec!["we"]],
        vec![vec!["are", "ear", "era"], vec!["waled"]],
        vec![vec!["area"], vec!["lewd", "weld"]],
        vec![vec!["aw"], vec!["dale", "deal", "lade", "lead"], vec!["re"]],
        vec![vec!["aw"], vec!["dealer", "leader"]],
        vec![vec!["aw"], vec!["deer", "reed"], vec!["la"]],
        vec![vec!["aw"], vec!["e'er", "ere"], vec!["lad"]],
        vec![vec!["aw"], vec!["earl", "real"], vec!["ed"]],
        vec![vec!["award"], vec!["eel", "lee"]],
        vec![vec!["aware"], vec!["led"]],
        vec![vec!["awe"], vec!["la"], vec!["red"]],
        vec![vec!["awe"], vec!["lad"], vec!["re"]],
        vec![vec!["awed", "wade"], vec!["earl", "real"]],
        vec![vec!["awed", "wade"], vec!["la"], vec!["re"]],
        vec![vec!["awl", "law"], vec!["ea"], vec!["red"]],
        vec![vec!["dale", "deal", "lade", "lead"], vec!["ware", "wear"]],
        vec![vec!["dare", "dear", "read"], vec!["la"], vec!["we"]],
        vec![vec!["dare", "dear", "read"], vec!["wale", "weal"]],
        vec![vec!["drew"], vec!["ea"], vec!["la"]],
        vec![vec!["e'er", "ere"], vec!["la"], vec!["wad"]],
        vec![vec!["ea"], vec!["lard"], vec!["we"]],
        vec![vec!["ea"], vec!["led"], vec!["raw", "war"]],
        vec![vec!["ed"], vec!["la"], vec!["ware", "wear"]],
    ];
    let config = Config {
        lang: Language::EN,
        dict_file_paths: DEFAULT_DICT_FILES.to_vec(),
        max_phrase_words,
        include_short: false,
        ..Config::default()
    };
    let search = Search::query(&input_phrase, &[], &config).unwrap();
    let (dict, _singles) = words::load_and_select(
        &config,
        &search.pattern,
        &search.essential,
        &search.primes_product,
        &[],
    )
    .unwrap();
    let cache = words::Cache::init(&dict);
    let mut builder = search.add_cache(&cache);
    let mut anagrams = builder.brute_force();
    anagrams.sort_unstable_by(
        |a, b| {
            if a[0] == b[0] {
                a[1].cmp(b[1])
            } else {
                a[0].cmp(b[0])
            }
        },
    );
    dbg!(&[expected.len(), anagrams.len()]);
    assert_eq!(expected, anagrams, "expected vs actual");
}

// Running with --ignored or --include-ignored will run these tests.
#[ignore = "takes 33 seconds on AMD Ryzen 5 7535U w/64 GB RAM using single core"]
#[test]
fn canary_three_words() {
    assert!(std::fs::exists(&DEFAULT_DICT_FILES[0]).expect("Word list file not found"));

    let max_phrase_words = 3;
    let input_phrase: Vec<String> =
        "canary in a coalmine".split(' ').map(ToString::to_string).collect();

    // TODO replace use of vec! macro with .json file, and compare files.
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["carcinoma"], vec!["inanely"]],
        vec![vec!["acacia"], vec!["airmen", "marine", "remain"], vec!["nylon"]],
        vec![vec!["acacia"], vec!["inanely"], vec!["morn", "norm"]],
        vec![vec!["acacia"], vec!["ion"], vec!["mannerly"]],
        vec![vec!["acacia"], vec!["morale"], vec!["ninny"]],
        vec![vec!["acacia"], vec!["my"], vec!["nonlinear"]],
        vec![vec!["acclaim"], vec!["anion"], vec!["yearn"]],
        vec![vec!["acclaim"], vec!["anyone"], vec!["rain"]],
        vec![vec!["acclaim"], vec!["inane"], vec!["rayon"]],
        vec![vec!["acne", "cane"], vec!["airmail"], vec!["canyon"]],
        vec![vec!["acne", "cane"], vec!["alimony"], vec!["crania"]],
        vec![vec!["acne", "cane"], vec!["inlay"], vec!["macaroni"]],
        vec![vec!["acne", "cane"], vec!["irony"], vec!["maniacal"]],
        vec![vec!["acne", "cane"], vec!["mainly"], vec!["ocarina"]],
        vec![vec!["acne", "cane"], vec!["mayoral"], vec!["niacin"]],
        vec![vec!["acorn"], vec!["anaemic"], vec!["inlay"]],
        vec![vec!["acre", "care", "race"], vec!["anomaly"], vec!["niacin"]],
        vec![vec!["acrylic"], vec!["anaemia"], vec!["non"]],
        vec![vec!["acrylic"], vec!["anemia"], vec!["anon"]],
        vec![vec!["aeon"], vec!["airman", "marina"], vec!["cynical"]],
        vec![vec!["aerial"], vec!["canyon"], vec!["manic"]],
        vec![vec!["aery", "year"], vec!["canonical"], vec!["main"]],
        vec![vec!["ail"], vec!["annoyance"], vec!["micra"]],
        vec![vec!["aim"], vec!["canonical"], vec!["yearn"]],
        vec![vec!["air"], vec!["annoyance"], vec!["claim"]],
        vec![vec!["air"], vec!["calamine"], vec!["canyon"]],
        vec![vec!["airline"], vec!["cancan"], vec!["mayo"]],
        vec![vec!["airmail"], vec!["canny"], vec!["canoe", "ocean"]],
        vec![vec!["airman", "marina"], vec!["canonical"], vec!["ye"]],
        vec![vec!["airmen", "marine", "remain"], vec!["ay"], vec!["canonical"]],
        vec![vec!["airy"], vec!["amen", "mane", "mean", "name"], vec!["canonical"]],
        vec![vec!["airy"], vec!["calamine"], vec!["canon"]],
        vec![vec!["airy"], vec!["maniacal"], vec!["nonce"]],
        vec![vec!["alien", "aline"], vec!["any", "nay"], vec!["carcinoma"]],
        vec![vec!["alien", "aline"], vec!["crayon"], vec!["maniac"]],
        vec![vec!["almanac"], vec!["annoy"], vec!["icier"]],
        vec![vec!["almanac"], vec!["niacin"], vec!["yore"]],
        vec![vec!["amen", "mane", "mean", "name"], vec!["crayola"], vec!["niacin"]],
        vec![vec!["amino"], vec!["arena"], vec!["cynical"]],
        vec![vec!["anaemia"], vec!["cannily"], vec!["orc"]],
        vec![vec!["anaemia"], vec!["canon"], vec!["lyric"]],
        vec![vec!["anaemia"], vec!["circa"], vec!["nylon"]],
        vec![vec!["anaemia"], vec!["coil", "loci"], vec!["cranny"]],
        vec![vec!["anaemia"], vec!["cynical"], vec!["nor"]],
        vec![vec!["anaemic"], vec!["anon"], vec!["racily"]],
        vec![vec!["anaemic"], vec!["any", "nay"], vec!["clarion"]],
        vec![vec!["anaemic"], vec!["canal"], vec!["irony"]],
        vec![vec!["anaemic"], vec!["canary"], vec!["lion", "loin"]],
        vec![vec!["anaemic"], vec!["cannily"], vec!["oar"]],
        vec![vec!["anaemic"], vec!["canyon"], vec!["lair", "liar", "lira", "rail"]],
        vec![vec!["anaemic"], vec!["crania"], vec!["only"]],
        vec![vec!["anaemic"], vec!["cranial"], vec!["yon"]],
        vec![vec!["anaemic"], vec!["crayola"], vec!["inn"]],
        vec![vec!["anaemic"], vec!["crayon"], vec!["lain", "nail"]],
        vec![vec!["anemia"], vec!["canon"], vec!["racily"]],
        vec![vec!["anemia"], vec!["canonical"], vec!["yr"]],
        vec![vec!["anemia"], vec!["conical", "laconic"], vec!["nary", "yarn"]],
        vec![vec!["anemia"], vec!["cynical"], vec!["roan"]],
        vec![vec!["anemic", "cinema"], vec!["annoy"], vec!["racial"]],
        vec![vec!["ani"], vec!["calamine"], vec!["crayon"]],
        vec![vec!["animal"], vec!["anyone"], vec!["circa"]],
        vec![vec!["animal"], vec!["cocaine", "oceanic"], vec!["nary", "yarn"]],
        vec![vec!["anime"], vec!["canonical"], vec!["ray"]],
        vec![vec!["anime"], vec!["canyon"], vec!["racial"]],
        vec![vec!["anion"], vec!["aria"], vec!["cyclamen"]],
        vec![vec!["anion"], vec!["calamine"], vec!["cray", "racy"]],
        vec![vec!["anion"], vec!["canary"], vec!["malice"]],
        vec![vec!["anion"], vec!["maraca"], vec!["nicely"]],
        vec![vec!["anneal"], vec!["icy"], vec!["macaroni"]],
        vec![vec!["annoy"], vec!["calcine"], vec!["maria"]],
        vec![vec!["annoy"], vec!["crania"], vec!["malice"]],
        vec![vec!["annoy"], vec!["lacier"], vec!["maniac"]],
        vec![vec!["annoy"], vec!["maniacal"], vec!["rice"]],
        vec![vec!["annoyance"], vec!["arm", "mar", "ram"], vec!["cilia"]],
        vec![vec!["annoyance"], vec!["lair", "liar", "lira", "rail"], vec!["mica"]],
        vec![vec!["annoyance"], vec!["mi"], vec!["racial"]],
        vec![vec!["anomaly"], vec!["circa"], vec!["inane"]],
        vec![vec!["anomaly"], vec!["crania"], vec!["nice"]],
        vec![vec!["anon"], vec!["clayier"], vec!["maniac"]],
        vec![vec!["anyone"], vec!["claim"], vec!["crania"]],
        vec![vec!["anyone"], vec!["cranial"], vec!["mica"]],
        vec![vec!["anyone"], vec!["manic"], vec!["racial"]],
        vec![vec!["arcane"], vec!["loamy"], vec!["niacin"]],
        vec![vec!["aroma"], vec!["cynical"], vec!["inane"]],
        vec![vec!["aye", "yea"], vec!["cinnamon"], vec!["racial"]],
        vec![vec!["ca"], vec!["inanely"], vec!["macaroni"]],
        vec![vec!["cacao"], vec!["inaner"], vec!["mainly"]],
        vec![vec!["cacao"], vec!["lineman", "melanin"], vec!["rainy"]],
        vec![vec!["cacao"], vec!["mainline"], vec!["nary", "yarn"]],
        vec![vec!["calamine"], vec!["canary"], vec!["ion"]],
        vec![vec!["calamine"], vec!["crania"], vec!["yon"]],
        vec![vec!["calcine"], vec!["mania"], vec!["rayon"]],
        vec![vec!["calorie"], vec!["canny"], vec!["mania"]],
        vec![vec!["cam"], vec!["inanely"], vec!["ocarina"]],
        vec![vec!["canine"], vec!["crania"], vec!["loamy"]],
        vec![vec!["canine"], vec!["cranial"], vec!["mayo"]],
        vec![vec!["canine"], vec!["crayola"], vec!["main"]],
        vec![vec!["canine"], vec!["lay"], vec!["macaroni"]],
        vec![vec!["canine"], vec!["maniac"], vec!["royal"]],
        vec![vec!["cannier"], vec!["maniacal"], vec!["yo"]],
        vec![vec!["cannily"], vec!["canoe", "ocean"], vec!["maria"]],
        vec![vec!["cannily"], vec!["ea"], vec!["macaroni"]],
        vec![vec!["canny"], vec!["email"], vec!["ocarina"]],
        vec![vec!["canoe", "ocean"], vec!["crania"], vec!["mainly"]],
        vec![vec!["canon"], vec!["clayier"], vec!["mania"]],
        vec![vec!["canonical"], vec!["mania"], vec!["rye"]],
        vec![vec!["canonical"], vec!["maria"], vec!["yen"]],
        vec![vec!["canyon"], vec!["crania"], vec!["email"]],
        vec![vec!["canyon"], vec!["ire"], vec!["maniacal"]],
        vec![vec!["canyon"], vec!["lacier"], vec!["mania"]],
        vec![vec!["canyon"], vec!["malaria"], vec!["nice"]],
        vec![vec!["carcinoma"], vec!["inane"], vec!["lay"]],
        vec![vec!["carnelian"], vec!["coy"], vec!["mania"]],
        vec![vec!["carnelian"], vec!["maniac"], vec!["yo"]],
        vec![vec!["clay", "lacy"], vec!["inane"], vec!["macaroni"]],
        vec![vec!["cocaine", "oceanic"], vec!["layman"], vec!["rain"]],
        vec![vec!["coin", "icon"], vec!["inanely"], vec!["maraca"]],
        vec![vec!["coin", "icon"], vec!["maniacal"], vec!["yearn"]],
        vec![vec!["coma"], vec!["crania"], vec!["inanely"]],
        vec![vec!["conceal"], vec!["mania"], vec!["rainy"]],
        vec![vec!["cone", "once"], vec!["maniacal"], vec!["rainy"]],
        vec![vec!["conical", "laconic"], vec!["mania"], vec!["yearn"]],
        vec![vec!["cornea"], vec!["inlay"], vec!["maniac"]],
        vec![vec!["coy"], vec!["inaner"], vec!["maniacal"]],
        vec![vec!["crayola"], vec!["inane"], vec!["manic"]],
        vec![vec!["crayola"], vec!["maniac"], vec!["nine"]],
        vec![vec!["layman"], vec!["nice"], vec!["ocarina"]],
        vec![vec!["maniacal"], vec!["nice"], vec!["rayon"]]
    ];
    let config = Config {
        lang: Language::EN,
        dict_file_paths: DEFAULT_DICT_FILES.to_vec(),
        max_phrase_words,
        include_short: false,
        ..Config::default()
    };
    let search = Search::query(&input_phrase, &[], &config).unwrap();
    let (dict, _singles) = words::load_and_select(
        &config,
        &search.pattern,
        &search.essential,
        &search.primes_product,
        &[],
    )
    .unwrap();
    let cache = words::Cache::init(&dict);
    let mut builder = search.add_cache(&cache);
    let mut anagrams = builder.brute_force();
    anagrams.sort_unstable_by(
        |a, b| {
            if a[0] == b[0] {
                a[1].cmp(b[1])
            } else {
                a[0].cmp(b[0])
            }
        },
    );
    dbg!(&[expected.len(), anagrams.len()]);
    assert_eq!(expected, anagrams, "expected vs actual");
}
