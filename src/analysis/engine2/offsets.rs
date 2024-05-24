use std::collections::BTreeMap;

use pelite::pattern;
use pelite::pattern::{save_len, Atom};
use pelite::pe64::{Pe, PeFile, Rva};

use phf::phf_map;

static PATTERNS: phf::Map<&'static str, &'static [Atom]> = phf_map! {
    "dwBuildNumber" => pattern!("8905${'} 488d0d${} ff15${}"),
    "dwNetworkGameClient" => pattern!("48893d${'} 488d15"),
    "dwNetworkGameClient_clientTickCount" => pattern!("8b81u4 c3 cccccccccccccccccc 8b81${} c3 cccccccccccccccccc 83b9"),
    "dwNetworkGameClient_deltaTick" => pattern!("8983u4 40b7"),
    "dwNetworkGameClient_isBackgroundMap" => pattern!("0fb681u4 c3 cccccccccccccccc 0fb681${} c3 cccccccccccccccc 48895c24"),
    "dwNetworkGameClient_localPlayer" => pattern!("4883c0u1 488d0440 8b0cc1"),
    "dwNetworkGameClient_maxClients" => pattern!("8b81u4 c3cccccccccccccccccc 8b81${} ffc0"),
    "dwNetworkGameClient_serverTickCount" => pattern!("8b81u4 c3 cccccccccccccccccc 83b9"),
    "dwNetworkGameClient_signOnState" => pattern!("448b81u4 488d0d"),
    "dwSoundService" => pattern!("488905${'} 4c8d4424? 488d05"),
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

        let mut rva = save[1];

        match *name {
            "dwNetworkGameClient_localPlayer" => {
                // .text 48 83 C0 0A | add rax, 0Ah
                // .text 48 8D 04 40 | lea rax, [rax + rax * 2]
                // .text 8B 0C C1    | mov ecx, [rcx + rax * 8]
                rva = (rva + (rva * 2)) * 8;
            }
            "dwSoundService" => {
                map.insert("dwEngineViewData", rva + 0x9C);
            }
            _ => {}
        }

        map.insert(*name, rva);
    }

    map
}
