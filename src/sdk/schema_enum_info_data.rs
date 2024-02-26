use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaEnumeratorInfoData;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SchemaEnumInfoData {
    pub pad_0: [u8; 0x8],                         // 0x0000
    pub name: Ptr<CStr>,                          // 0x0008
    pub pad_1: [u8; 0x8],                         // 0x0010
    pub alignment: u8,                            // 0x0018
    pub pad_2: [u8; 0x3],                         // 0x0019
    pub size: u16,                                // 0x001C
    pub static_metadata_size: u16,                // 0x001E
    pub enum_info: Ptr<SchemaEnumeratorInfoData>, // 0x0020
    pub pad_3: [u8; 0x1C],                        // 0x0030
}

impl SchemaEnumInfoData {
    #[inline]
    pub fn type_name(&self) -> &str {
        match self.alignment {
            1 => "uint8_t",
            2 => "uint16_t",
            4 => "uint32_t",
            8 => "uint64_t",
            _ => "unknown",
        }
    }
}

unsafe impl Pod for SchemaEnumInfoData {}
