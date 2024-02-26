use pelite::pe64::{msvc, Pe, PeFile, PeObject, Ptr, Rva};

use rayon::prelude::*;

use crate::error::Result;

const BLACKLIST: [&'static str; 2] = [".?AVexception@std@@", ".?AVtype_info@@"];

/// Represents a global vtable instance.
#[derive(Clone, Copy, Debug)]
pub struct Global<'a> {
    /// The mangled RTTI type name.
    pub type_name: &'a str,

    /// The RVA of the vtable instance.
    pub instance: Rva,
}

/// Scans the PE file for global vtable instances.
pub fn globals(file: PeFile<'_>) -> Vec<Global<'_>> {
    let image = file.image();

    let mut list: Vec<_> = (0..image.len() / 8)
        .into_par_iter()
        .filter_map(|i| {
            file.file_offset_to_rva(i * 8)
                .ok()
                .and_then(|rva| global(file, rva).ok())
                .filter(|instance| !BLACKLIST.contains(&instance.type_name))
        })
        .collect();

    list.dedup_by_key(|k| k.type_name);

    list
}

fn global(file: PeFile<'_>, rva: Rva) -> Result<Global<'_>> {
    let vtable_va = *file.derva::<u64>(rva)?;
    let vtable_rva = file.va_to_rva(vtable_va)?;

    let col_ptr = *file.deref::<Ptr<msvc::RTTICompleteObjectLocator>>((vtable_va - 0x8).into())?;
    let col = file.deref(col_ptr)?;

    let type_info = file.derva::<msvc::TypeDescriptor>(col.type_descriptor)?;

    if type_info.spare != Ptr::null() {
        return Err(pelite::Error::Null.into());
    }

    let type_name = file.derva_c_str(col.type_descriptor + 0x10)?.to_str()?;

    Ok(Global {
        type_name,
        instance: vtable_rva,
    })
}
