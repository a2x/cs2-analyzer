use log::{info, warn};

use pelite::pattern;
use pelite::pe64::{Pe, PeFile, Rva};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::convars::ConVarFlags;

use crate::error::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ConCommand<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub flags: ConVarFlags,
}

impl<'a> ConCommand<'a> {
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

pub fn concommands(file: PeFile<'_>) -> Vec<ConCommand<'_>> {
    // XREF: "RegisterConCommand: Unknown error registering con command \"%s\"!\n"
    let mut matches = file.scanner().matches_code(pattern!(
        "4c8d0d${'} [5-40] 488d15${'} (488d0d${} | 88442438 488d0d${}) (48c7442420u4 | [5-60] 48c7442420u4) [5-40] e8${48895c2408}"
    ));

    let mut save = [0; 4];

    let mut list = Vec::new();

    while matches.next(&mut save) {
        _ = read(file, &save, &mut list);
    }

    if list.is_empty() {
        warn!("unable to find any concommands");
    }

    list.dedup_by_key(|k| k.name);
    list.sort_unstable_by_key(|k| k.name);

    list
}

fn read<'a>(file: PeFile<'a>, save: &[Rva], list: &mut Vec<ConCommand<'a>>) -> Result<()> {
    let description = Some(file.derva_c_str(save[1])?.to_str()?).filter(|s| !s.is_empty());
    let name = file.derva_c_str(save[2])?.to_str()?;

    let flags = ConVarFlags::try_from(save[3]).unwrap_or(ConVarFlags::None);

    info!("found concommand: {}", name);

    list.push(ConCommand {
        name,
        description,
        flags,
    });

    Ok(())
}
