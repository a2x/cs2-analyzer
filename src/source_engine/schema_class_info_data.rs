use pelite::pe64::{Ptr, Va};
use pelite::util::CStr;
use pelite::Pod;

use super::{SchemaBaseClassInfoData, SchemaClassFieldData, SchemaMetadataEntryData};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SchemaClassInfoData {
    pad_0000: [u8; 0x8],
    pub name: Ptr<CStr>,
    pad_0016: [u8; 0x8],
    pub size: u32,
    pub fields_count: u16,
    pad_0030: [u8; 0x4],
    pub align_of: u8,
    pad_0036: [u8; 0x5],
    pub fields: Ptr<SchemaClassFieldData>,
    pad_0048: [u8; 0x8],
    pub base_classes: Ptr<SchemaBaseClassInfoData>,
    pad_0064: [u8; 0x8],
    pub metadata: Ptr<SchemaMetadataEntryData>,
    pad_0080: [u8; 0x18],
    pub function: Ptr<Va>,
}

unsafe impl Pod for SchemaClassInfoData {}
