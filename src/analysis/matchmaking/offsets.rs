use std::collections::BTreeMap;

use pelite::pattern;
use pelite::pattern::{save_len, Atom};
use pelite::pe64::{Pe, PeFile, Rva};

use phf::phf_map;

static PATTERNS: phf::Map<&'static str, &'static [Atom]> = phf_map! {
    "dwGameTypes" => pattern!("488d0d${'} 33d2"),
    "dwGameTypes_mapName" => pattern!("488b81u4 4885c074? 4883c0"),
};

pub fn offsets(file: PeFile<'_>) -> BTreeMap<&'static str, Rva> {
    let mut map = BTreeMap::new();

    for (name, pat) in &PATTERNS {
        let mut save = vec![0; save_len(&pat)];

        if !file.scanner().finds_code(pat, &mut save) {
            continue;
        }

        map.insert(*name, save[1]);
    }

    map
}
