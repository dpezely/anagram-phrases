use std::path::PathBuf;
use std::string::ToString;
use std::sync::LazyLock;

use crate::config::Config;
use crate::languages::Language;
use crate::search::Search;
use crate::words;

static EN_AU_DICT_FILES: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| vec![PathBuf::from("third-party/en_AU/SCOWL-wl/words.txt")]);

static EN_CA_DICT_FILES: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| vec![PathBuf::from("third-party/en_CA/SCOWL-wl/words.txt")]);

static EN_US_DICT_FILES: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| vec![PathBuf::from("third-party/en_US/SCOWL-wl/words.txt")]);

/// One of the inaugural states of Australia was New South Wales.
#[test]
fn en_au_new_south_wales_two_words() {
    let max_phrase_words = 2;
    let input_phrase = "New South Wales";
    let word_list_files = &EN_AU_DICT_FILES;
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["newel"], vec!["washout's", "washouts"]],
        vec![vec!["newel's", "newels"], vec!["washout"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, false);
}

// Running with --ignored or --include-ignored will run these tests.
//#[ignore = "takes 90+ seconds on AMD Ryzen 5 7535U w/64 GB RAM using single core"]
#[test]
fn en_au_new_south_wales_three_words() {
    let max_phrase_words = 3;
    let input_phrase = "New South Wales";
    let word_list_files = &EN_AU_DICT_FILES;
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["aeon"], vec!["lust's", "lusts", "slut's", "sluts"], vec!["whew"]],
        vec![vec!["aeon's", "aeons"], vec!["lust", "slut"], vec!["whew"]],
        vec![vec!["ah", "ha"], vec!["tuneless"], vec!["wow"]],
        vec![vec!["ale", "lea"], vec!["snout's", "snouts"], vec!["whew"]],
        vec![vec!["ale's", "ales", "lea's", "leas", "sale", "seal"], vec!["snout"], vec!["whew"]],
        vec![vec!["aloe"], vec!["stuns"], vec!["whew"]],
        vec![vec!["aloe's", "aloes"], vec!["nut's", "nuts", "stun", "tun's", "tuns"], vec!["whew"]],
        vec![vec!["alone"], vec!["thew", "whet"], vec!["wuss"]],
        vec![vec!["also"], vec!["tune's", "tunes", "unset"], vec!["whew"]],
        vec![vec!["alt", "lat"], vec!["onuses"], vec!["whew"]],
        vec![vec!["an"], vec!["lotuses", "solute's", "solutes", "tousles"], vec!["whew"]],
        vec![vec!["anew", "wane", "wean"], vec!["helot", "hotel", "thole"], vec!["wuss"]],
        vec![vec!["anew", "wane", "wean"], vec!["hew"], vec!["lotus's"]],
        vec![vec!["anew", "wane", "wean"], vec!["hews", "shew"], vec!["lotus", "lout's", "louts"]],
        vec![vec!["anew", "wane", "wean"], vec!["how", "who"], vec!["tussle"]],
        vec![vec!["anew", "wane", "wean"], vec!["how's", "hows", "show", "who's"], vec!["lute's", "lutes"]],
        vec![vec!["anew", "wane", "wean"], vec!["howl"], vec!["suet's"]],
        vec![vec!["anew", "wane", "wean"], vec!["howl's", "howls"], vec!["suet", "ute's", "utes"]],
        vec![vec!["anew", "wane", "wean"], vec!["hustle", "sleuth"], vec!["sow"]],
        vec![vec!["anew", "wane", "wean"], vec!["hustle's", "hustles", "lushest", "sleuth's", "sleuths"], vec!["ow"]],
        vec![vec!["anew", "wane", "wean"], vec!["lout"], vec!["shews"]],
        vec![vec!["anew", "wane", "wean"], vec!["low", "owl"], vec!["tushes"]],
        vec![vec!["anew", "wane", "wean"], vec!["lushes"], vec!["tow", "two", "wot"]],
        vec![vec!["anew", "wane", "wean"], vec!["lust", "slut"], vec!["whose"]],
        vec![vec!["anew", "wane", "wean"], vec!["lute"], vec!["show's", "shows"]],
        vec![vec!["anew", "wane", "wean"], vec!["oust", "out's", "outs"], vec!["welsh"]],
        vec![vec!["anew", "wane", "wean"], vec!["shout", "south", "thou's", "thous"], vec!["slew"]],
        vec![vec!["anew", "wane", "wean"], vec!["slew's", "slews"], vec!["thou"]],
        vec![vec!["anew", "wane", "wean"], vec!["slowest"], vec!["uh"]],
        vec![vec!["anew", "wane", "wean"], vec!["soul"], vec!["thew's", "thews", "whets"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, true);
}

/// One of the inaugural provinces of Canada was Nova Scotia.
#[test]
fn en_ca_nova_scotia_two_words() {
    let max_phrase_words = 2;
    let input_phrase = "Nova Scotia";
    let word_list_files = &EN_CA_DICT_FILES;
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["vocation's", "vocations"]],
        vec![vec!["ac", "ca"], vec!["ovation's", "ovations"]],
        vec![vec!["action's", "actions", "cation's", "cations"], vec!["ova"]],
        vec![vec!["as"], vec!["vocation"]],
        vec![vec!["avian"], vec!["coot's", "coots", "scoot"]],
        vec![vec!["ovation"], vec!["sac"]],
        vec![vec!["so"], vec!["vacation"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, false);
}

#[test]
fn en_ca_nova_scotia_three_words() {
    let max_phrase_words = 3;
    let input_phrase = "Nova Scotia";
    let word_list_files = &EN_CA_DICT_FILES;
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["I"], vec!["an"], vec!["octavo's", "octavos"]],
        vec![vec!["I"], vec!["ans"], vec!["octavo"]],
        vec![vec!["I"], vec!["ascot", "coast", "coat's", "coats", "taco's", "tacos"], vec!["nova"]],
        vec![vec!["I"], vec!["avast"], vec!["coon"]],
        vec![vec!["I"], vec!["canto's", "cantos"], vec!["ova"]],
        vec![vec!["I"], vec!["canvas"], vec!["too"]],
        vec![vec!["I"], vec!["coat", "taco"], vec!["nova's", "novas"]],
        vec![vec!["I"], vec!["coo"], vec!["savant"]],
        vec![vec!["a"], vec!["ascot", "coast", "coat's", "coats", "taco's", "tacos"], vec!["vino"]],
        vec![vec!["a"], vec!["coat", "taco"], vec!["vino's"]],
        vec![vec!["a"], vec!["coon"], vec!["vista", "vita's"]],
        vec![vec!["a"], vec!["coon's", "coons"], vec!["vita"]],
        vec![vec!["a"], vec!["coot's", "coots", "scoot"], vec!["vain"]],
        vec![vec!["a"], vec!["cs"], vec!["ovation"]],
        vec![vec!["a"], vec!["in"], vec!["octavo's", "octavos"]],
        vec![vec!["a"], vec!["in's", "ins", "sin"], vec!["octavo"]],
        vec![vec!["a"], vec!["nova"], vec!["stoic"]],
        vec![vec!["a"], vec!["ova"], vec!["tocsin", "tonic's", "tonics"]],
        vec![vec!["a"], vec!["vocation's", "vocations"]],
        vec![vec!["ac", "ca"], vec!["oat"], vec!["vino's"]],
        vec![vec!["ac", "ca"], vec!["oat's", "oats"], vec!["vino"]],
        vec![vec!["ac", "ca"], vec!["onto"], vec!["visa"]],
        vec![vec!["ac", "ca"], vec!["ovation's", "ovations"]],
        vec![vec!["ac", "ca"], vec!["snoot"], vec!["via"]],
        vec![vec!["ac", "ca"], vec!["soon"], vec!["vita"]],
        vec![vec!["ac", "ca"], vec!["soot"], vec!["vain"]],
        vec![vec!["act", "cat"], vec!["iOS"], vec!["nova"]],
        vec![vec!["act", "cat"], vec!["ion's", "ions"], vec!["ova"]],
        vec![vec!["act", "cat"], vec!["nova's", "novas"], vec!["oi"]],
        vec![vec!["act", "cat"], vec!["soon"], vec!["via"]],
        vec![vec!["act's", "acts", "cast", "cat's", "cats", "scat"], vec!["ion"], vec!["ova"]],
        vec![vec!["act's", "acts", "cast", "cat's", "cats", "scat"], vec!["nova"], vec!["oi"]],
        vec![vec!["action", "cation"], vec!["av"], vec!["so"]],
        vec![vec!["action's", "actions", "cation's", "cations"], vec!["ova"]],
        vec![vec!["ain't", "anti"], vec!["av"], vec!["coo's", "coos"]],
        vec![vec!["ain't", "anti"], vec!["cos", "soc"], vec!["ova"]],
        vec![vec!["an"], vec!["coo"], vec!["vista", "vita's"]],
        vec![vec!["an"], vec!["coo's", "coos"], vec!["vita"]],
        vec![vec!["an"], vec!["coot"], vec!["visa"]],
        vec![vec!["an"], vec!["coot's", "coots", "scoot"], vec!["via"]],
        vec![vec!["an"], vec!["is"], vec!["octavo"]],
        vec![vec!["an"], vec!["ova"], vec!["stoic"]],
        vec![vec!["ans"], vec!["coo"], vec!["vita"]],
        vec![vec!["ans"], vec!["coot"], vec!["via"]],
        vec![vec!["ant", "tan"], vec!["coo"], vec!["visa"]],
        vec![vec!["ant", "tan"], vec!["coo's", "coos"], vec!["via"]],
        vec![vec!["ant's", "ants", "tan's", "tans"], vec!["coo"], vec!["via"]],
        vec![vec!["anti's", "antis", "saint", "satin", "stain"], vec!["av"], vec!["coo"]],
        vec![vec!["anti's", "antis", "saint", "satin", "stain"], vec!["co"], vec!["ova"]],
        vec![vec!["antic"], vec!["ova"], vec!["so"]],
        vec![vec!["as"], vec!["coat", "taco"], vec!["vino"]],
        vec![vec!["as"], vec!["coon"], vec!["vita"]],
        vec![vec!["as"], vec!["coot"], vec!["vain"]],
        vec![vec!["as"], vec!["in"], vec!["octavo"]],
        vec![vec!["as"], vec!["ova"], vec!["tonic"]],
        vec![vec!["as"], vec!["vocation"]],
        vec![vec!["ascot", "coast", "coat's", "coats", "taco's", "tacos"], vec!["av"], vec!["ion"]],
        vec![vec!["ascot", "coast", "coat's", "coats", "taco's", "tacos"], vec!["in"], vec!["ova"]],
        vec![vec!["ascot", "coast", "coat's", "coats", "taco's", "tacos"], vec!["no", "on"], vec!["via"]],
        vec![vec!["ascot", "coast", "coat's", "coats", "taco's", "tacos"], vec!["oi"], vec!["van"]],
        vec![vec!["at", "ta"], vec!["coin's", "coins", "icon's", "icons", "scion", "sonic"], vec!["ova"]],
        vec![vec!["at", "ta"], vec!["coo's", "coos"], vec!["vain"]],
        vec![vec!["at", "ta"], vec!["coon"], vec!["visa"]],
        vec![vec!["at", "ta"], vec!["coon's", "coons"], vec!["via"]],
        vec![vec!["av"], vec!["canto"], vec!["iOS"]],
        vec![vec!["av"], vec!["canto's", "cantos"], vec!["oi"]],
        vec![vec!["av"], vec!["casino"], vec!["to"]],
        vec![vec!["av"], vec!["ciao"], vec!["snot", "ton's", "tons"]],
        vec![vec!["av"], vec!["ciaos"], vec!["not", "ton"]],
        vec![vec!["av"], vec!["coat", "taco"], vec!["ion's", "ions"]],
        vec![vec!["av"], vec!["coin", "icon"], vec!["oat's", "oats"]],
        vec![vec!["av"], vec!["coin's", "coins", "icon's", "icons", "scion", "sonic"], vec!["oat"]],
        vec![vec!["av"], vec!["con"], vec!["iota's", "iotas"]],
        vec![vec!["av"], vec!["con's", "cons"], vec!["iota"]],
        vec![vec!["avast"], vec!["co"], vec!["ion"]],
        vec![vec!["avast"], vec!["con"], vec!["oi"]],
        vec![vec!["avast"], vec!["coo"], vec!["in"]],
        vec![vec!["avian"], vec!["co"], vec!["sot"]],
        vec![vec!["avian"], vec!["coo"], vec!["st", "ts"]],
        vec![vec!["avian"], vec!["coot's", "coots", "scoot"]],
        vec![vec!["avian"], vec!["cos", "soc"], vec!["to"]],
        vec![vec!["avian"], vec!["cot"], vec!["so"]],
        vec![vec!["avian"], vec!["cs"], vec!["too"]],
        vec![vec!["can"], vec!["soot"], vec!["via"]],
        vec![vec!["can"], vec!["too"], vec!["visa"]],
        vec![vec!["can's", "cans", "scan"], vec!["too"], vec!["via"]],
        vec![vec!["can't", "cant"], vec!["iOS"], vec!["ova"]],
        vec![vec!["canst", "cant's", "cants", "scant"], vec!["oi"], vec!["ova"]],
        vec![vec!["canto"], vec!["is"], vec!["ova"]],
        vec![vec!["canto"], vec!["so"], vec!["via"]],
        vec![vec!["canvas"], vec!["oi"], vec!["to"]],
        vec![vec!["ciao"], vec!["no", "on"], vec!["vast", "vat's", "vats"]],
        vec![vec!["ciao"], vec!["no's", "nos", "son"], vec!["vat"]],
        vec![vec!["ciao"], vec!["nova"], vec!["st", "ts"]],
        vec![vec!["ciao"], vec!["sot"], vec!["van"]],
        vec![vec!["ciao"], vec!["to"], vec!["van's", "vans"]],
        vec![vec!["ciaos"], vec!["no", "on"], vec!["vat"]],
        vec![vec!["ciaos"], vec!["ova"], vec!["tn"]],
        vec![vec!["ciaos"], vec!["to"], vec!["van"]],
        vec![vec!["cis", "sci", "sic"], vec!["nova"], vec!["oat"]],
        vec![vec!["co"], vec!["iota"], vec!["van's", "vans"]],
        vec![vec!["co"], vec!["iota's", "iotas"], vec!["van"]],
        vec![vec!["co"], vec!["oat's", "oats"], vec!["vain"]],
        vec![vec!["co"], vec!["oi"], vec!["savant"]],
        vec![vec!["coat", "taco"], vec!["iOS"], vec!["van"]],
        vec![vec!["coat", "taco"], vec!["in's", "ins", "sin"], vec!["ova"]],
        vec![vec!["coat", "taco"], vec!["is"], vec!["nova"]],
        vec![vec!["coat", "taco"], vec!["no", "on"], vec!["visa"]],
        vec![vec!["coat", "taco"], vec!["no's", "nos", "son"], vec!["via"]],
        vec![vec!["coat", "taco"], vec!["oi"], vec!["van's", "vans"]],
        vec![vec!["coat", "taco"], vec!["so"], vec!["vain"]],
        vec![vec!["coin", "icon"], vec!["ova"], vec!["sat"]],
        vec![vec!["con"], vec!["oat"], vec!["visa"]],
        vec![vec!["con"], vec!["oat's", "oats"], vec!["via"]],
        vec![vec!["con's", "cons"], vec!["oat"], vec!["via"]],
        vec![vec!["coo"], vec!["sat"], vec!["vain"]],
        vec![vec!["coon"], vec!["sat"], vec!["via"]],
        vec![vec!["cos", "soc"], vec!["iota"], vec!["van"]],
        vec![vec!["cos", "soc"], vec!["oat"], vec!["vain"]],
        vec![vec!["cs"], vec!["iota"], vec!["nova"]],
        vec![vec!["inc"], vec!["oat's", "oats"], vec!["ova"]],
        vec![vec!["into"], vec!["ova"], vec!["sac"]],
        vec![vec!["ion"], vec!["oat"], vec!["vacs"]],
        vec![vec!["ion"], vec!["oat's", "oats"], vec!["vac"]],
        vec![vec!["ion's", "ions"], vec!["oat"], vec!["vac"]],
        vec![vec!["iota"], vec!["no", "on"], vec!["vacs"]],
        vec![vec!["iota"], vec!["no's", "nos", "son"], vec!["vac"]],
        vec![vec!["iota's", "iotas"], vec!["no", "on"], vec!["vac"]],
        vec![vec!["oat"], vec!["sac"], vec!["vino"]],
        vec![vec!["oi"], vec!["so"], vec!["vacant"]],
        vec![vec!["onto"], vec!["sac"], vec!["via"]],
        vec![vec!["ovation"], vec!["sac"]],
        vec![vec!["sac"], vec!["too"], vec!["vain"]],
        vec![vec!["so"], vec!["vacation"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, false);
}

/// The first state registered after founding USA was Delaware.
#[test]
fn en_us_delaware_two_words() {
    let max_phrase_words = 2;
    let input_phrase = "Delaware";
    let word_list_files = &EN_US_DICT_FILES;
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["leeward"]],
        vec![vec!["alder"], vec!["awe"]],
        vec![vec!["ale", "lea"], vec!["wader"]],
        vec![vec!["are", "ear", "era"], vec!["waled"]],
        vec![vec!["area"], vec!["lewd", "weld"]],
        vec![vec!["areal"], vec!["dew", "we'd", "wed"]],
        vec![vec!["aw"], vec!["dealer", "leader"]],
        vec![vec!["award"], vec!["eel", "lee"]],
        vec![vec!["aware"], vec!["led"]],
        vec![vec!["awed", "wade"], vec!["earl", "real"]],
        vec![vec!["awl", "law"], vec!["eared"]],
        vec![vec!["dale", "deal", "lade", "lead"], vec!["ware", "wear"]],
        vec![vec!["dare", "dear", "read"], vec!["wale", "weal"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, false);
}

#[test]
fn en_us_delaware_three_words() {
    let max_phrase_words = 3;
    let input_phrase = "Delaware";
    let word_list_files = &EN_US_DICT_FILES;
    // Keep sorted.  Sequence may differ from CLI output.
    // TODO migrate `expected` to .json file, and compare files.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["a"], vec!["lewder", "welder"]],
        vec![vec!["a"], vec!["alder"], vec!["we"]],
        vec![vec!["a"], vec!["ale", "lea"], vec!["drew"]],
        vec![vec!["a"], vec!["are", "ear", "era"], vec!["lewd", "weld"]],
        vec![vec!["a"], vec!["aw"], vec!["elder"]],
        vec![vec!["a"], vec!["awed", "wade"], vec!["rel"]],
        vec![vec!["a"], vec!["awl", "law"], vec!["deer", "reed"]],
        vec![vec!["a"], vec!["dew", "we'd", "wed"], vec!["earl", "real"]],
        vec![vec!["a"], vec!["draw", "ward"], vec!["eel", "lee"]],
        vec![vec!["a"], vec!["er", "re"], vec!["waled"]],
        vec![vec!["a"], vec!["ewe", "wee"], vec!["lard"]],
        vec![vec!["a"], vec!["ewer", "we're", "weer", "were"], vec!["lad"]],
        vec![vec!["a"], vec!["la"], vec!["rewed"]],
        vec![vec!["a"], vec!["led"], vec!["ware", "wear"]],
        vec![vec!["a"], vec!["leer", "reel"], vec!["wad"]],
        vec![vec!["a"], vec!["leeward"]],
        vec![vec!["a"], vec!["red"], vec!["wale", "weal"]],
        vec![vec!["ad"], vec!["aw"], vec!["leer", "reel"]],
        vec![vec!["ad"], vec!["awe"], vec!["rel"]],
        vec![vec!["ad"], vec!["awl", "law"], vec!["e'er", "ere"]],
        vec![vec!["ad"], vec!["earl", "real"], vec!["we"]],
        vec![vec!["ad"], vec!["eel", "lee"], vec!["raw", "war"]],
        vec![vec!["ad"], vec!["er", "re"], vec!["wale", "weal"]],
        vec![vec!["ad"], vec!["ewer", "we're", "weer", "were"], vec!["la"]],
        vec![vec!["alder"], vec!["awe"]],
        vec![vec!["ale", "lea"], vec!["aw"], vec!["red"]],
        vec![vec!["ale", "lea"], vec!["awe"], vec!["rd"]],
        vec![vec!["ale", "lea"], vec!["ed"], vec!["raw", "war"]],
        vec![vec!["ale", "lea"], vec!["er", "re"], vec!["wad"]],
        vec![vec!["ale", "lea"], vec!["rad"], vec!["we"]],
        vec![vec!["ale", "lea"], vec!["wader"]],
        vec![vec!["are", "ear", "era"], vec!["aw"], vec!["led"]],
        vec![vec!["are", "ear", "era"], vec!["awl", "law"], vec!["ed"]],
        vec![vec!["are", "ear", "era"], vec!["dew", "we'd", "wed"], vec!["la"]],
        vec![vec!["are", "ear", "era"], vec!["lad"], vec!["we"]],
        vec![vec!["are", "ear", "era"], vec!["waled"]],
        vec![vec!["area"], vec!["lewd", "weld"]],
        vec![vec!["areal"], vec!["dew", "we'd", "wed"]],
        vec![vec!["aw"], vec!["dale", "deal", "lade", "lead"], vec!["er", "re"]],
        vec![vec!["aw"], vec!["dealer", "leader"]],
        vec![vec!["aw"], vec!["deer", "reed"], vec!["la"]],
        vec![vec!["aw"], vec!["e'er", "ere"], vec!["lad"]],
        vec![vec!["aw"], vec!["earl", "real"], vec!["ed"]],
        vec![vec!["aw"], vec!["eel", "lee"], vec!["rad"]],
        vec![vec!["award"], vec!["eel", "lee"]],
        vec![vec!["aware"], vec!["led"]],
        vec![vec!["awe"], vec!["er", "re"], vec!["lad"]],
        vec![vec!["awe"], vec!["la"], vec!["red"]],
        vec![vec!["awed", "wade"], vec!["earl", "real"]],
        vec![vec!["awed", "wade"], vec!["er", "re"], vec!["la"]],
        vec![vec!["awl", "law"], vec!["ea"], vec!["red"]],
        vec![vec!["awl", "law"], vec!["eared"]],
        vec![vec!["dale", "deal", "lade", "lead"], vec!["ware", "wear"]],
        vec![vec!["dare", "dear", "read"], vec!["la"], vec!["we"]],
        vec![vec!["dare", "dear", "read"], vec!["wale", "weal"]],
        vec![vec!["drew"], vec!["ea"], vec!["la"]],
        vec![vec!["e'er", "ere"], vec!["la"], vec!["wad"]],
        vec![vec!["ea"], vec!["lard"], vec!["we"]],
        vec![vec!["ea"], vec!["led"], vec!["raw", "war"]],
        vec![vec!["ea"], vec!["rd"], vec!["wale", "weal"]],
        vec![vec!["ea"], vec!["rel"], vec!["wad"]],
        vec![vec!["ed"], vec!["la"], vec!["ware", "wear"]],
        vec![vec!["ewe", "wee"], vec!["la"], vec!["rad"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, false);
}

// Running with --ignored or --include-ignored will run these tests.
//#[ignore = "takes 30+ seconds on AMD Ryzen 5 7535U w/64 GB RAM using single core"]
#[test]
fn canary_three_words() {
    let max_phrase_words = 3;
    let input_phrase = "canary in a coalmine";
    let word_list_files = &EN_US_DICT_FILES;
    // TODO replace use of vec! macro with .json file, and compare files.
    // Keep sorted.  Sequence may differ from CLI output.
    #[rustfmt::skip]
    let expected = vec![
        vec![vec!["a"], vec!["carcinoma"], vec!["inanely"]],
        vec![vec!["ac", "ca"], vec!["inanely"], vec!["macaroni"]],
        vec![vec!["acacia"], vec!["airmen", "marine", "remain"], vec!["nylon"]],
        vec![vec!["acacia"], vec!["inanely"], vec!["morn", "norm"]],
        vec![vec!["acacia"], vec!["ion"], vec!["mannerly"]],
        vec![vec!["acacia"], vec!["loamy"], vec!["rennin"]],
        vec![vec!["acacia"], vec!["morale"], vec!["ninny"]],
        vec![vec!["acacia"], vec!["my"], vec!["nonlinear"]],
        vec![vec!["acclaim"], vec!["anion"], vec!["yearn"]],
        vec![vec!["acclaim"], vec!["anyone"], vec!["rain"]],
        vec![vec!["acclaim"], vec!["inane"], vec!["rayon"]],
        vec![vec!["acne", "cane"], vec!["acrimony"], vec!["lanai"]],
        vec![vec!["acne", "cane"], vec!["airmail"], vec!["canyon"]],
        vec![vec!["acne", "cane"], vec!["inlay"], vec!["macaroni"]],
        vec![vec!["acne", "cane"], vec!["irony"], vec!["maniacal"]],
        vec![vec!["acne", "cane"], vec!["mainly"], vec!["ocarina"]],
        vec![vec!["acne", "cane"], vec!["mayoral"], vec!["niacin"]],
        vec![vec!["acre", "care", "race"], vec!["anionic"], vec!["layman"]],
        vec![vec!["acre", "care", "race"], vec!["anomaly"], vec!["niacin"]],
        vec![vec!["acrylic"], vec!["anemia"], vec!["anon"]],
        vec![vec!["acyl", "clay", "lacy"], vec!["inane"], vec!["macaroni"]],
        vec![vec!["aerial"], vec!["canny"], vec!["manioc"]],
        vec![vec!["aerial"], vec!["canyon"], vec!["manic"]],
        vec![vec!["aerial"], vec!["cay"], vec!["cinnamon"]],
        vec![vec!["aery", "year"], vec!["canonical"], vec!["main"]],
        vec![vec!["aileron"], vec!["caiman", "maniac"], vec!["cyan"]],
        vec![vec!["aim"], vec!["canonical"], vec!["yearn"]],
        vec![vec!["air"], vec!["annoyance"], vec!["claim"]],
        vec![vec!["air"], vec!["calamine"], vec!["canyon"]],
        vec![vec!["air"], vec!["canonical"], vec!["meany"]],
        vec![vec!["airline"], vec!["cancan"], vec!["mayo"]],
        vec![vec!["airmail"], vec!["canny"], vec!["canoe", "ocean"]],
        vec![vec!["airman", "marina"], vec!["canonical"], vec!["ye"]],
        vec![vec!["airman", "marina"], vec!["coca"], vec!["inanely"]],
        vec![vec!["airmen", "marine", "remain"], vec!["canonical"], vec!["ya"]],
        vec![vec!["airy"], vec!["amen", "mane", "mean", "name"], vec!["canonical"]],
        vec![vec!["airy"], vec!["calamine"], vec!["canon"]],
        vec![vec!["airy"], vec!["maniacal"], vec!["nonce"]],
        vec![vec!["alien"], vec!["any", "nay"], vec!["carcinoma"]],
        vec![vec!["alien"], vec!["caiman", "maniac"], vec!["crayon"]],
        vec![vec!["alien"], vec!["canary"], vec!["manioc"]],
        vec![vec!["alien"], vec!["cyan"], vec!["macaroni"]],
        vec![vec!["almanac"], vec!["anionic"], vec!["rye", "yer"]],
        vec![vec!["almanac"], vec!["annoy"], vec!["icier"]],
        vec![vec!["almanac"], vec!["ionic"], vec!["yearn"]],
        vec![vec!["almanac"], vec!["niacin"], vec!["yore"]],
        vec![vec!["amine", "anime"], vec!["canary"], vec!["oilcan"]],
        vec![vec!["amine", "anime"], vec!["canonical"], vec!["ray"]],
        vec![vec!["amine", "anime"], vec!["canyon"], vec!["racial"]],
        vec![vec!["amine", "anime"], vec!["cay"], vec!["nonracial"]],
        vec![vec!["amino"], vec!["arena"], vec!["cynical"]],
        vec![vec!["amino"], vec!["carnelian"], vec!["cay"]],
        vec![vec!["amnion"], vec!["area"], vec!["cynical"]],
        vec![vec!["anal"], vec!["anionic"], vec!["creamy"]],
        vec![vec!["anemia"], vec!["canon"], vec!["racily"]],
        vec![vec!["anemia"], vec!["canonical"], vec!["yr"]],
        vec![vec!["anemia"], vec!["carny"], vec!["oilcan"]],
        vec![vec!["anemia"], vec!["clarion"], vec!["cyan"]],
        vec![vec!["anemia"], vec!["conical", "laconic"], vec!["nary", "yarn"]],
        vec![vec!["anemia"], vec!["cony"], vec!["cranial"]],
        vec![vec!["anemia"], vec!["cynical"], vec!["roan"]],
        vec![vec!["anemic", "cinema", "iceman"], vec!["annoy"], vec!["racial"]],
        vec![vec!["anemic", "cinema", "iceman"], vec!["canola"], vec!["rainy"]],
        vec![vec!["anemic", "cinema", "iceman"], vec!["crayon"], vec!["lanai"]],
        vec![vec!["anemic", "cinema", "iceman"], vec!["nonracial"], vec!["ya"]],
        vec![vec!["aniline"], vec!["canary"], vec!["coma"]],
        vec![vec!["aniline"], vec!["cony"], vec!["maraca"]],
        vec![vec!["animal", "lamina", "manila"], vec!["anyone"], vec!["circa"]],
        vec![vec!["animal", "lamina", "manila"], vec!["cannery"], vec!["ciao"]],
        vec![vec!["animal", "lamina", "manila"], vec!["cocaine", "oceanic"], vec!["nary", "yarn"]],
        vec![vec!["animal", "lamina", "manila"], vec!["ency"], vec!["ocarina"]],
        vec![vec!["anion"], vec!["aria"], vec!["cyclamen"]],
        vec![vec!["anion"], vec!["calamari"], vec!["ency"]],
        vec![vec!["anion"], vec!["calamine"], vec!["racy"]],
        vec![vec!["anion"], vec!["canary"], vec!["malice"]],
        vec![vec!["anion"], vec!["maraca"], vec!["nicely"]],
        vec![vec!["anionic"], vec!["any", "nay"], vec!["caramel"]],
        vec![vec!["anionic"], vec!["canary"], vec!["lame", "male", "meal"]],
        vec![vec!["anionic"], vec!["manacle"], vec!["ray"]],
        vec![vec!["anneal"], vec!["icy"], vec!["macaroni"]],
        vec![vec!["annoy"], vec!["caiman", "maniac"], vec!["eclair", "lacier"]],
        vec![vec!["annoy"], vec!["calamari"], vec!["cine", "nice"]],
        vec![vec!["annoy"], vec!["calcine"], vec!["maria"]],
        vec![vec!["annoy"], vec!["ceramic"], vec!["lanai"]],
        vec![vec!["annoy"], vec!["circa"], vec!["laminae"]],
        vec![vec!["annoy"], vec!["maniacal"], vec!["rice"]],
        vec![vec!["annoyance"], vec!["arm", "mar", "ram"], vec!["cilia"]],
        vec![vec!["annoyance"], vec!["cram"], vec!["ilia"]],
        vec![vec!["annoyance"], vec!["lair", "liar", "lira", "rail", "rial"], vec!["mica"]],
        vec![vec!["annoyance"], vec!["mi"], vec!["racial"]],
        vec![vec!["anomaly"], vec!["circa"], vec!["inane"]],
        vec![vec!["anon"], vec!["caiman", "maniac"], vec!["clayier"]],
        vec![vec!["any", "nay"], vec!["canonical"], vec!["ramie"]],
        vec![vec!["any", "nay"], vec!["cocaine", "oceanic"], vec!["laminar"]],
        vec![vec!["any", "nay"], vec!["coiner"], vec!["maniacal"]],
        vec![vec!["any", "nay"], vec!["licorice"], vec!["manana"]],
        vec![vec!["anymore"], vec!["cancan"], vec!["ilia"]],
        vec![vec!["anyone"], vec!["calamari"], vec!["inc"]],
        vec![vec!["anyone"], vec!["cir"], vec!["maniacal"]],
        vec![vec!["anyone"], vec!["cranial"], vec!["mica"]],
        vec![vec!["anyone"], vec!["manic"], vec!["racial"]],
        vec![vec!["arcane"], vec!["inlay"], vec!["manioc"]],
        vec![vec!["arcane"], vec!["ionic"], vec!["layman"]],
        vec![vec!["arcane"], vec!["loamy"], vec!["niacin"]],
        vec![vec!["area"], vec!["cannily"], vec!["manioc"]],
        vec![vec!["arena"], vec!["iconic"], vec!["layman"]],
        vec![vec!["aria"], vec!["canny"], vec!["coalmine"]],
        vec![vec!["aroma"], vec!["cynical"], vec!["inane"]],
        vec![vec!["aye", "yea"], vec!["cinnamon"], vec!["racial"]],
        vec![vec!["aye", "yea"], vec!["manic"], vec!["nonracial"]],
        vec![vec!["cacao"], vec!["inaner"], vec!["mainly"]],
        vec![vec!["cacao"], vec!["lineman", "melanin"], vec!["rainy"]],
        vec![vec!["cacao"], vec!["mainline"], vec!["nary", "yarn"]],
        vec![vec!["caiman", "maniac"], vec!["canine"], vec!["royal"]],
        vec![vec!["caiman", "maniac"], vec!["carnelian"], vec!["yo"]],
        vec![vec!["caiman", "maniac"], vec!["carny"], vec!["eolian"]],
        vec![vec!["caiman", "maniac"], vec!["cornea"], vec!["inlay"]],
        vec![vec!["caiman", "maniac"], vec!["nonracial"], vec!["ye"]],
        vec![vec!["caiman", "maniac"], vec!["oilcan"], vec!["yearn"]],
        vec![vec!["calamari"], vec!["canine"], vec!["yon"]],
        vec![vec!["calamari"], vec!["cony"], vec!["inane"]],
        vec![vec!["calamine"], vec!["canary"], vec!["ion"]],
        vec![vec!["calcine"], vec!["mania"], vec!["rayon"]],
        vec![vec!["calorie"], vec!["canny"], vec!["mania"]],
        vec![vec!["cam", "mac"], vec!["inanely"], vec!["ocarina"]],
        vec![vec!["canary"], vec!["ciao"], vec!["lineman", "melanin"]],
        vec![vec!["canary"], vec!["coin", "icon"], vec!["laminae"]],
        vec![vec!["canary"], vec!["eolian"], vec!["manic"]],
        vec![vec!["canary"], vec!["income"], vec!["lanai"]],
        vec![vec!["canine"], vec!["cay"], vec!["manorial"]],
        vec![vec!["canine"], vec!["cony"], vec!["malaria"]],
        vec![vec!["canine"], vec!["cranial"], vec!["mayo"]],
        vec![vec!["canine"], vec!["lay"], vec!["macaroni"]],
        vec![vec!["cannery"], vec!["maniacal"], vec!["oi"]],
        vec![vec!["cannier"], vec!["ciao"], vec!["layman"]],
        vec![vec!["cannier"], vec!["maniacal"], vec!["yo"]],
        vec![vec!["cannily"], vec!["canoe", "ocean"], vec!["maria"]],
        vec![vec!["cannily"], vec!["ea"], vec!["macaroni"]],
        vec![vec!["canny"], vec!["email"], vec!["ocarina"]],
        vec![vec!["canny"], vec!["ilea"], vec!["macaroni"]],
        vec![vec!["canon"], vec!["clayier"], vec!["mania"]],
        vec![vec!["canonical"], vec!["mania"], vec!["rye", "yer"]],
        vec![vec!["canonical"], vec!["maria"], vec!["yen"]],
        vec![vec!["canyon"], vec!["cine", "nice"], vec!["malaria"]],
        vec![vec!["canyon"], vec!["eclair", "lacier"], vec!["mania"]],
        vec![vec!["canyon"], vec!["ire"], vec!["maniacal"]],
        vec![vec!["carcinoma"], vec!["inane"], vec!["lay"]],
        vec![vec!["carcinoma"], vec!["inlay"], vec!["nae"]],
        vec![vec!["carcinoma"], vec!["lanai"], vec!["yen"]],
        vec![vec!["carnelian"], vec!["ciao"], vec!["many", "myna"]],
        vec![vec!["carnelian"], vec!["coy"], vec!["mania"]],
        vec![vec!["carnelian"], vec!["manioc"], vec!["ya"]],
        vec![vec!["cay"], vec!["lineman", "melanin"], vec!["ocarina"]],
        vec![vec!["ciao"], vec!["cranny"], vec!["laminae"]],
        vec![vec!["ciceroni"], vec!["lay"], vec!["manana"]],
        vec![vec!["cine", "nice"], vec!["layman"], vec!["ocarina"]],
        vec![vec!["cine", "nice"], vec!["maniacal"], vec!["rayon"]],
        vec![vec!["clayier"], vec!["coin", "icon"], vec!["manana"]],
        vec![vec!["clayier"], vec!["manioc"], vec!["naan"]],
        vec![vec!["cloacae"], vec!["maria"], vec!["ninny"]],
        vec![vec!["cocaine", "oceanic"], vec!["layman"], vec!["rain"]],
        vec![vec!["cocaine", "oceanic"], vec!["manna"], vec!["riyal"]],
        vec![vec!["coin", "icon"], vec!["inanely"], vec!["maraca"]],
        vec![vec!["coin", "icon"], vec!["maniacal"], vec!["yearn"]],
        vec![vec!["conceal"], vec!["mania"], vec!["rainy"]],
        vec![vec!["cone", "econ", "once"], vec!["maniacal"], vec!["rainy"]],
        vec![vec!["conical", "laconic"], vec!["mania"], vec!["yearn"]],
        vec![vec!["cornea"], vec!["icily"], vec!["manana"]],
        vec![vec!["cornea"], vec!["maniacal"], vec!["yin"]],
        vec![vec!["coy"], vec!["inaner"], vec!["maniacal"]],
        vec![vec!["cyan"], vec!["menial"], vec!["ocarina"]],
        vec![vec!["early", "layer", "relay"], vec!["iconic"], vec!["manana"]],
        vec![vec!["ency"], vec!["lanai"], vec!["macaroni"]],
        vec![vec!["icicle"], vec!["manana"], vec!["rayon"]],
        vec![vec!["manacle"], vec!["ocarina"], vec!["yin"]],
    ];
    anagrams(max_phrase_words, input_phrase, word_list_files, expected, false);
}

fn anagrams(
    max_phrase_words: usize, input_phrase: &str, word_list_files: &[PathBuf],
    expected: Vec<Vec<Vec<&str>>>, elided: bool,
) {
    for f in word_list_files {
        assert!(std::fs::exists(f).expect("Word list file not found"));
    }

    let input_phrase: Vec<String> =
        input_phrase.split(' ').map(ToString::to_string).collect();

    let config = Config {
        lang: Language::EN,
        dict_file_paths: word_list_files.to_vec(),
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
    if elided {
        let limit = expected.len();
        assert_eq!(expected, anagrams[..limit], "expected (elided) vs actual (sliced)");
    } else {
        assert_eq!(expected, anagrams, "expected vs actual");
    }
}
