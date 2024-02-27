use std::collections::BTreeMap;

use pelite::pattern;
use pelite::pattern::{save_len, Atom};
use pelite::pe64::{Pe, PeFile, Rva};

use phf::phf_map;

static PATTERNS: phf::Map<&'static str, &'static [Atom]> = phf_map! {
    "dwCSGOInput" => pattern!("4c8b0d${*{'}} 488d045b"),
    "dwEntityList" => pattern!("488935${'} 4885f6"),
    "dwGameEntitySystem" => pattern!("488b1d${'} 48891d"),
    "dwGameEntitySystem_getHighestEntityIndex" => pattern!("8b81u2?? 8902 488bc2 c3 cccccccc 48895C24? 48896c24"),
    "dwGameRules" => pattern!("48890d${'} 8b0d${} ff15"),
    "dwGlobalVars" => pattern!("48890d${'} 488941"),
    "dwGlowManager" => pattern!("488b05${'} c3 cccccccccccccccc 8b41"),
    "dwLocalPlayerController" => pattern!("488b05${'} 4885c0 74? 8b88"),
    "dwPlantedC4" => pattern!("488b15${'} ffc0 488d4c24"),
    "dwPrediction" => pattern!("488d05${'} c3 cccccccccccccccc 4883ec? 8b0d"),
    "dwSensitivity" => pattern!("488b05${'} 488b40? f3410f59f4"),
    "dwSensitivity_sensitivity" => pattern!("ff50u1 4c8bc6 488d55? 488bcf e8${} 84c0 0f85${} 4c8d45? 8bd3 488bcf e8${} e9${} f30f1006"),
    "dwViewMatrix" => pattern!("488d0d${'} 48c1e006"),
    "dwViewRender" => pattern!("488905${'} 488bc8 4885c0"),
};

pub fn offsets(file: PeFile<'_>) -> BTreeMap<&'static str, Rva> {
    let mut map = BTreeMap::new();

    for (name, pat) in &PATTERNS {
        let mut save = vec![0; save_len(&pat)];

        if !file.scanner().finds_code(pat, &mut save) {
            continue;
        }

        if *name == "dwCSGOInput" {
            let instance = save[1];

            let mut save = [0; 2];

            if file
                .scanner()
                .finds_code(pattern!("498d81u4 4803c7"), &mut save)
            {
                map.insert("dwViewAngles", instance + save[1]);
            }
        }

        if *name == "dwPrediction" {
            map.insert("dwLocalPlayerPawn", save[1] + 0x138);
        }

        map.insert(*name, save[1]);
    }

    map
}
