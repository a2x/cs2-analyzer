use log::{info, warn};

use num_enum::TryFromPrimitive;

use pelite::pattern;
use pelite::pe64::{Pe, PeFile, Rva};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::error::Result;

// TODO: Add other flags.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
#[repr(u32)]
pub enum ConVarFlags {
    None = 0x0,
    Unregistered = 0x1,
    DevelopmentOnly = 0x2,
    GameDll = 0x4,
    ClientDll = 0x8,
    Hidden = 0x10,
    Protected = 0x20,
    SpOnly = 0x40,
    Archive = 0x80,
    Notify = 0x100,
    UserInfo = 0x200,
    Unlogged = 0x800,
    Replicated = 0x2000,
    Cheat = 0x4000,
    PerUser = 0x8000,
    Demo = 0x10000,
    DontRecord = 0x20000,
    NotConnected = 0x40000,
    VConsoleSetFocus = 0x8000000,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ConVar<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub flags: ConVarFlags,
    pub rva: Rva,
}

impl<'a> ConVar<'a> {
    #[inline]
    pub fn contains(&self, flag: ConVarFlags) -> bool {
        (self.flags as u32) & (flag as u32) != 0
    }

    #[inline]
    pub fn is_cheat(&self) -> bool {
        self.contains(ConVarFlags::Cheat)
    }

    #[inline]
    pub fn is_dev_only(&self) -> bool {
        self.contains(ConVarFlags::DevelopmentOnly)
    }

    #[inline]
    pub fn is_hidden(&self) -> bool {
        self.contains(ConVarFlags::Hidden)
    }

    #[inline]
    pub fn is_protected(&self) -> bool {
        self.contains(ConVarFlags::Protected)
    }

    #[inline]
    pub fn is_replicated(&self) -> bool {
        self.contains(ConVarFlags::Replicated)
    }
}

pub fn convars(file: PeFile<'_>) -> Vec<ConVar<'_>> {
    // XREF: "RegisterConVar: Unknown error registering convar \"%s\"!\n"
    let mut matches = file.scanner().matches_code(pattern!(
        "e8${48895c2408} 488d442430 c6442434? ('4533c9 | 4c8d0d${'}) 4889442420 41b8u4 (c6442437? | c7442437${}) 488d15${'} 488d0d${'}"
    ));

    let mut save = [0; 5];

    let mut list = Vec::new();

    while matches.next(&mut save) {
        _ = read(file, &save, &mut list);
    }

    if list.is_empty() {
        warn!("unable to find any convars");
    }

    list.dedup_by_key(|k| k.name);
    list.sort_unstable_by_key(|k| k.name);

    list
}

fn read<'a>(file: PeFile<'a>, save: &[Rva], list: &mut Vec<ConVar<'a>>) -> Result<()> {
    let description = file.derva_c_str(save[1])?.to_str().ok();
    let name = file.derva_c_str(save[3])?.to_str()?;

    let flags = ConVarFlags::try_from(save[2]).unwrap_or(ConVarFlags::None);

    info!("found convar: {}", name);

    list.push(ConVar {
        name,
        description,
        flags,
        rva: save[4] + 0x8,
    });

    Ok(())
}
