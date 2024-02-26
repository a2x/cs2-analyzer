use log::{info, warn};

use pelite::pattern;
use pelite::pe64::{Pe, PeFile, Rva};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use crate::error::Result;

/// Represents an interface exported by `CreateInterface`.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Interface<'a> {
    /// The name of the interface, including the version number.
    ///
    /// E.g. `Source2Client002`.
    pub name: &'a str,

    /// The RVA of the interface instance.
    pub value: Rva,
}

/// Scans the PE file for interfaces exported by `CreateInterface`.
pub fn interfaces(file: PeFile<'_>) -> Vec<Interface<'_>> {
    // Ensure the PE file exports "CreateInterface".
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
        "cc 4c8d05${'} 488d15${'} 488d0d${} e9${4c894108} cc"
    ));

    let mut save = [0; 3];

    let mut list = Vec::new();

    while matches.next(&mut save) {
        let _ = read(file, &save, &mut list);
    }

    if list.is_empty() {
        warn!("unable to find any interfaces");
    }

    list.sort_unstable_by_key(|k| k.name);

    list
}

fn read<'a>(file: PeFile<'a>, save: &[Rva], list: &mut Vec<Interface<'a>>) -> Result<()> {
    let name = file.derva_c_str(save[1])?.to_str()?;
    let value = save[2];

    info!("found interface: {} @ {:#X}", name, value);

    list.push(Interface { name, value });

    Ok(())
}
