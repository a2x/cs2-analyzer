use log::info;

use pelite::pattern;
use pelite::pe64::{Pe, PeFile, Rva};

use crate::error::Result;

/// Represents a key button.
///
/// In the Source Engine, this is referred to as `kbutton_t`.
#[derive(Clone, Copy, Debug)]
pub struct Button<'a> {
    /// The name of the key button.
    pub name: &'a str,

    /// The RVA of the key button state.
    pub value: Rva,
}

/// Scans the PE file for all key buttons.
pub fn buttons(file: PeFile<'_>) -> Vec<Button<'_>> {
    let mut matches = file.scanner().matches_code(pattern!(
        "4883ec28 4533c0 488d15${'} 488d0d${'} e8${48895c2408}"
    ));

    let mut save = [0; 3];

    let mut list = Vec::new();

    while matches.next(&mut save) {
        let _ = read(file, &save, &mut list);
    }

    list.dedup_by_key(|k| k.name);
    list.sort_unstable_by_key(|k| k.name);

    list
}

fn read<'a>(file: PeFile<'a>, save: &[Rva], list: &mut Vec<Button<'a>>) -> Result<()> {
    let name = file.derva_c_str(save[1])?.to_str()?;
    let value = save[2] + 0x30 - 0x8;

    info!("found button: {} @ {:#X}", name, value);

    list.push(Button { name, value });

    Ok(())
}