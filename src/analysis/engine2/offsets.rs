use std::collections::BTreeMap;

use pelite::pattern;
use pelite::pattern::{save_len, Atom};
use pelite::pe64::{Pe, PeFile, Rva};

use phf::phf_map;

static PATTERNS: phf::Map<&'static str, &'static [Atom]> = phf_map! {
    "dwBuildNumber" => pattern!("8905${'} 488d0d${} ff15${} e9"),
    "dwNetworkGameClient" => pattern!("48893d${'} 488d15"),
    "dwNetworkGameClient_deltaTick" => pattern!("8983u4 40b7"),
    "dwNetworkGameClient_getLocalPlayer" => pattern!("4883c0u1 488d0440 458b04c7"),
    "dwNetworkGameClient_getMaxClients" => pattern!("8b81u2?? c3cccccccccccccccccc 8b81${} ffc0"),
    "dwNetworkGameClient_signOnState" => pattern!("448b81u2?? 488d0d"),
    "dwWindowHeight" => pattern!("8b05${'} 8903"),
    "dwWindowWidth" => pattern!("8b05${'} 8907"),
};

pub fn offsets(file: PeFile<'_>) -> BTreeMap<&'static str, Rva> {
    let mut map = BTreeMap::new();

    for (name, pat) in &PATTERNS {
        let mut save = vec![0; save_len(&pat)];

        if !file.scanner().finds_code(pat, &mut save) {
            continue;
        }

        if *name == "dwNetworkGameClient_getLocalPlayer" {
            // .text 48 83 C0 0A   add rax, 0Ah
            // .text 48 8D 04 40   lea rax, [rax+rax*2]
            // .text 45 8B 04 C7   mov r8d, [r15+rax*8]

            let index = (save[1] + (save[1] * 2)) * 8;

            save[1] = index;
        }

        map.insert(*name, save[1]);
    }

    map
}
