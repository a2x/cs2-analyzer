use std::collections::BTreeMap;

use pelite::pattern;
use pelite::pattern::{save_len, Atom};
use pelite::pe64::{Pe, PeFile, Rva};

use phf::phf_map;

static PATTERNS: phf::Map<&'static str, &'static [Atom]> = phf_map! {
    "dwCSGOInput" => pattern!("488d0d${'} e8${} 488d05${} 48c705[8] 488905${} 488d0d${} 488d05"),
    "dwEntityList" => pattern!("488935${'} 4885f6"),
    "dwGameEntitySystem" => pattern!("488b1d${'} 48891d"),
    "dwGameEntitySystem_highestEntityIndex" => pattern!("8b81u2?? 8902 488bc2 c3 cccccccc 48895c24? 48896c24"),
    "dwGameRules" => pattern!("48891d${'} ff15${} 84c0"),
    "dwGlobalVars" => pattern!("48890d${'} 488941"),
    "dwGlowManager" => pattern!("488b05${'} c3 cccccccccccccccc 8b41"),
    "dwLocalPlayerController" => pattern!("488905${'} 8b9e"),
    "dwPlantedC4" => pattern!("488b15${'} 41ffc0"),
    "dwPrediction" => pattern!("488d05${'} c3 cccccccccccccccc 4883ec? 8b0d"),
    "dwSensitivity" => pattern!("488b05${'} 488b40? f3410f59f4"),
    "dwSensitivity_sensitivity" => pattern!("ff50u1 4c8bc6 488d55? 488bcf e8${} 84c0 0f85${} 4c8d45? 8bd3 488bcf e8${} e9${} f30f1006"),
    "dwViewMatrix" => pattern!("488d0d${'} 48c1e006"),
    "dwViewRender" => pattern!("488905${'} 488bc8 4885c0"),
    "dwWeaponC4" => pattern!("488b15${'} ffc0 8905${} 488bc6 488934ea 488b6c24? c686[5] 80be"),
};

pub fn offsets(file: PeFile<'_>) -> BTreeMap<&'static str, Rva> {
    let mut map = BTreeMap::new();

    for (name, pat) in &PATTERNS {
        let mut save = vec![0; save_len(&pat)];

        if !file.scanner().finds_code(pat, &mut save) {
            continue;
        }

        let rva = save[1];

        match *name {
            "dwCSGOInput" => {
                let mut save = [0; 2];

                if file
                    .scanner()
                    .finds_code(pattern!("f2410f108430u4"), &mut save)
                {
                    map.insert("dwViewAngles", rva + save[1]);
                }
            }
            "dwPrediction" => {
                map.insert("dwLocalPlayerPawn", rva + 0x148);
            }
            _ => {}
        }

        map.insert(*name, rva);
    }

    map
}
