use pelite::pe64::{Ptr, Va};
use pelite::util::CStr;
use pelite::Pod;

use super::{SchemaBaseClassInfoData, SchemaClassFieldData, SchemaMetadataEntryData};

#[derive(Pod)]
#[repr(C)]
pub struct SchemaClassInfoData {
    pad_0000: [u8; 0x8],                               // 0x0000
    pub name: Ptr<CStr>,                               // 0x0008
    pad_0010: [u8; 0x8],                               // 0x0010
    pub size: i32,                                     // 0x0018
    pub field_count: u16,                              // 0x001C
    pad_001e: [u8; 0x4],                               // 0x001E
    pub alignment: u8,                                 // 0x0022
    pad_0023: [u8; 0x5],                               // 0x0023
    pub fields: Ptr<[SchemaClassFieldData]>,           // 0x0028
    pad_0030: [u8; 0x8],                               // 0x0030
    pub base_classes: Ptr<SchemaBaseClassInfoData>,    // 0x0038
    pad_0040: [u8; 0x8],                               // 0x0040
    pub static_metadata: Ptr<SchemaMetadataEntryData>, // 0x0048
    pad_0050: [u8; 0x18],                              // 0x0050
    pub function: Ptr<Va>,                             // 0x0068
}
