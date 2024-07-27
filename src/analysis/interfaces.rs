use log::{info, warn};

use pelite::pattern;
use pelite::pe64::{Pe, PeFile, Rva};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Interface<'a> {
    pub name: &'a str,
    pub rva: Rva,
}

pub fn interfaces(file: PeFile<'_>) -> Vec<Interface<'_>> {
    if file
        .exports()
        .unwrap()
        .by()
        .unwrap()
        .name("CreateInterface")
        .is_err()
    {
        return Vec::new();
    }

    let mut matches = file.scanner().matches_code(pattern!(
        "cc 4c8d05${'} 488d15${488d05${'}} 488d0d${} e9${4c894108} cc"
    ));

    let mut save = [0; 3];

    let mut list = Vec::new();

    while matches.next(&mut save) {
        _ = read(file, &save, &mut list);
    }

    if list.is_empty() {
        warn!("unable to find any interfaces");
    }

    list.sort_unstable_by_key(|k| k.name);

    list
}

fn read<'a>(file: PeFile<'a>, save: &[Rva], list: &mut Vec<Interface<'a>>) -> Result<()> {
    let name = file.derva_c_str(save[1])?.to_str()?;
    let rva = save[2];

    info!("found interface: {} at {:#X}", name, rva);

    list.push(Interface { name, rva });

    Ok(())
}
