use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::{SchemaBaseClassInfoData, SchemaClassFieldData};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SchemaClassInfoData {
    pub pad_0: [u8; 0x8],                           // 0x0000
    pub name: Ptr<CStr>,                            // 0x0008
    pub pad_1: [u8; 0x8],                           // 0x0010
    pub size: u32,                                  // 0x0018
    pub num_fields: u16,                            // 0x001C
    pub pad_2: [u8; 0x4],                           // 0x001E
    pub alignment: u8,                              // 0x0022
    pub num_base_classes: u8,                       // 0x0023
    pub pad_3: [u8; 0x4],                           // 0x0024
    pub fields: Ptr<SchemaClassFieldData>,          // 0x0028
    pub pad_4: [u8; 0x8],                           // 0x0030
    pub base_classes: Ptr<SchemaBaseClassInfoData>, // 0x0038
    pub pad_5: [u8; 0x30],                          // 0x0040
}

unsafe impl Pod for SchemaClassInfoData {}
