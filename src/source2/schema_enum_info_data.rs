use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaEnumeratorInfoData;

#[derive(Pod)]
#[repr(C)]
pub struct SchemaEnumInfoData {
    pad_0000: [u8; 0x8],                              // 0x0000
    pub name: Ptr<CStr>,                              // 0x0008
    pad_0010: [u8; 0x8],                              // 0x0010
    pub alignment: u8,                                // 0x0018
    pad_0019: [u8; 0x3],                              // 0x0019
    pub enumerator_count: u16,                        // 0x001C
    pub static_metadata_count: u16,                   // 0x001E
    pub enumerators: Ptr<[SchemaEnumeratorInfoData]>, // 0x0020
    pad_0028: [u8; 0x20],                             // 0x0028
}

impl SchemaEnumInfoData {
    #[inline]
    pub fn type_name(&self) -> &str {
        match self.alignment {
            1 => "uint8",
            2 => "uint16",
            4 => "uint32",
            8 => "uint64",
            _ => "unknown",
        }
    }
}
