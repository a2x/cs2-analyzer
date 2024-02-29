use std::mem;

use log::{info, warn};

use pelite::pattern::{save_len, Atom};
use pelite::pe64::{Pe, PeFile, Ptr, Rva, Va};
use pelite::{pattern, Pod};

use rayon::prelude::*;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::globals;

use crate::error::{Error, Result};

use crate::sdk::{
    SchemaClassFieldData, SchemaClassInfoData, SchemaEnumInfoData, SchemaEnumeratorInfoData,
    SchemaFieldType, SchemaMetadataEntryData,
};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Class<'a> {
    pub name: &'a str,
    pub parent: Option<Box<Class<'a>>>,
    pub fields: Vec<ClassField<'a>>,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ClassField<'a> {
    pub name: &'a str,
    pub r#type: SchemaFieldType,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct ClassMetadata<'a> {
    pub name: &'a str,
    pub function: Rva,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct Enum<'a> {
    pub name: &'a str,
    pub type_name: &'a str,
    pub alignment: u8,
    pub size: u16,
    pub members: Vec<EnumMember<'a>>,
}

impl<'a> Enum<'a> {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.size > 0 && self.alignment >= 1 && self.alignment <= 8
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde_support", derive(Deserialize, Serialize))]
pub struct EnumMember<'a> {
    pub name: &'a str,
    pub value: i64,
}

struct SchemaRegistration<'a> {
    #[allow(dead_code)]
    type_name: &'a str,

    constructor: Rva,
}

pub fn schemas(file: PeFile<'_>) -> (Vec<Class<'_>>, Vec<Enum<'_>>) {
    // Ensure the PE file exports "InstallSchemaBindings".
    if file
        .exports()
        .unwrap()
        .by()
        .unwrap()
        .name("InstallSchemaBindings")
        .is_err()
    {
        return (Vec::new(), Vec::new());
    }

    let registrations = schema_registrations(file);

    let mut classes: Vec<_> = registrations
        .par_iter()
        .flat_map(|registration| process_entries::<SchemaClassInfoData, _, _>(
            file,
            registration,
            pattern!("(4183f803 75? ??? ??? ???${'} ff9018 010000 | 83?03 75? ??? ???${'} ??? ff9018 010000)"),
            read_class,
        ))
        .collect();

    let mut enums: Vec<_> = registrations
        .par_iter()
        .flat_map(|registration| {
            process_entries::<SchemaEnumInfoData, _, _>(
                file,
                registration,
                pattern!("488b? 488d?${'} 4889?2428 4c8d0d${}"),
                read_enum,
            )
        })
        .collect();

    if classes.is_empty() {
        warn!("unable to find any classes");
    }

    if enums.is_empty() {
        warn!("unable to find any enums");
    }

    classes.sort_unstable_by_key(|k| k.name);
    enums.sort_unstable_by_key(|k| k.name);

    (classes, enums)
}

fn process_entries<'a, T, F, E>(
    file: PeFile<'a>,
    registration: &SchemaRegistration<'a>,
    pat: &[Atom],
    process_entry: F,
) -> Vec<E>
where
    T: Pod,
    F: Fn(PeFile<'a>, Ptr<T>) -> Result<E>,
{
    let mut matches = file.scanner().matches_code(pat);

    let mut save = vec![0; save_len(pat)];

    let start_addr = registration.constructor;
    let end_addr = start_addr + 0x1000;

    let mut list = Vec::new();

    while matches.next(&mut save) {
        if start_addr < save[0] && save[0] < end_addr {
            if let Ok(entries) = table_entries::<T>(file, save[1]) {
                for entry in entries {
                    if let Ok(result) = process_entry(file, entry) {
                        list.push(result);
                    }
                }
            }
        }
    }

    list
}

fn read_class(file: PeFile<'_>, ptr: Ptr<SchemaClassInfoData>) -> Result<Class<'_>> {
    let data = file.deref(ptr)?;
    let name = file.deref_c_str(data.name)?.to_str()?;

    let fields = read_class_fields(file, &data)?;
    let metadata = read_class_metadata(file, data.metadata)?;

    let parent = if !data.base_classes.is_null() {
        let base_class = file.deref(data.base_classes)?;

        Some(Box::new(read_class(file, base_class.prev)?))
    } else {
        None
    };

    info!(
        "found class: {} (parent: {:?}) (fields: {}) (metadata: {:?})",
        name,
        parent.as_ref().map(|p| p.name),
        fields.len(),
        metadata
    );

    Ok(Class {
        name,
        parent,
        fields,
    })
}

fn read_class_fields<'a>(
    file: PeFile<'a>,
    data: &SchemaClassInfoData,
) -> Result<Vec<ClassField<'a>>> {
    (0..data.num_fields)
        .into_par_iter()
        .map(|i| {
            let ptr: Ptr<SchemaClassFieldData> = data
                .fields
                .offset((i * mem::size_of::<SchemaClassFieldData>() as u16).into());

            let data = file.deref(ptr)?;
            let name = file.deref_c_str(data.name)?.to_str()?;

            Ok(ClassField {
                name,
                r#type: data.r#type(),
                offset: data.offset,
            })
        })
        .collect()
}

fn read_class_metadata(
    file: PeFile<'_>,
    ptr: Ptr<SchemaMetadataEntryData>,
) -> Result<ClassMetadata<'_>> {
    let data = file.deref(ptr)?;
    let name = file.deref_c_str(data.name)?.to_str()?;
    let function_va = *file.deref(data.function)?;
    let function_rva = file.va_to_rva(function_va)?;

    Ok(ClassMetadata {
        name,
        function: function_rva,
    })
}

fn read_enum(file: PeFile<'_>, ptr: Ptr<SchemaEnumInfoData>) -> Result<Enum<'_>> {
    let data = file.deref(ptr)?;
    let name = file.deref_c_str(data.name)?.to_str()?;

    let members = read_enum_members(file, &data)?;

    let e = Enum {
        name,
        type_name: data.type_name(),
        alignment: data.alignment,
        size: data.size,
        members,
    };

    if !e.is_valid() {
        return Err(Error::Other("Invalid enum"));
    }

    info!(
        "found enum: {} (type: {}) (alignment: {}) (size: {}) (members: {})",
        e.name,
        e.type_name,
        e.alignment,
        e.size,
        e.members.len()
    );

    Ok(e)
}

fn read_enum_members<'a>(
    file: PeFile<'a>,
    data: &SchemaEnumInfoData,
) -> Result<Vec<EnumMember<'a>>> {
    (0..data.size)
        .into_par_iter()
        .map(|i| {
            let ptr: Ptr<SchemaEnumeratorInfoData> = data
                .enum_info
                .offset((i * mem::size_of::<SchemaEnumeratorInfoData>() as u16).into());

            let data = file.deref(ptr)?;
            let name = file.deref_c_str(data.name)?.to_str()?;

            let value = {
                let value = unsafe { data.u.ulong } as i64;

                if value == i64::MAX {
                    -1
                } else {
                    value
                }
            };

            Ok(EnumMember { name, value })
        })
        .collect()
}

fn schema_registrations(file: PeFile<'_>) -> Vec<SchemaRegistration<'_>> {
    globals::globals(file)
        .par_iter()
        .filter(|instance| instance.type_name.contains("CSchemaRegistration_"))
        .filter_map(|global| {
            let result = file
                .derva(global.instance)
                .and_then(|va| file.va_to_rva(*va))
                .map(|constructor| SchemaRegistration {
                    type_name: global.type_name,
                    constructor,
                });

            if let Ok(registration) = &result {
                info!(
                    "found schema registration: {} @ {:#X} (constructor @ {:#X})",
                    global.type_name, global.instance, registration.constructor
                );
            }

            result.ok()
        })
        .collect::<Vec<_>>()
}

fn table_entries<T: Pod>(file: PeFile<'_>, table: Rva) -> Result<Vec<Ptr<T>>> {
    let mut cur_entry = file.rva_to_va(table)?;

    let mut entries = Vec::new();

    while let Ok(entry) = file.deref_copy::<Ptr<T>>(cur_entry.into()) {
        if entry.is_null() {
            break;
        }

        entries.push(entry);

        cur_entry += mem::size_of::<usize>() as Va;
    }

    Ok(entries)
}
