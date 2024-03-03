use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaEnumeratorInfoData;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SchemaEnumInfoData {
    pad_0000: [u8; 0x8],
    pub name: Ptr<CStr>,
    pad_0016: [u8; 0x8],
    pub align_of: u8,
    pad_0025: [u8; 0x3],
    pub size: u16,
    pub static_metadata_count: u16,
    pub enum_info: Ptr<SchemaEnumeratorInfoData>,
    pad_0040: [u8; 0x1C],
}

unsafe impl Pod for SchemaEnumInfoData {}

impl SchemaEnumInfoData {
    #[inline]
    pub fn type_name(&self) -> &'static str {
        match self.align_of {
            1 => "uint8_t",
            2 => "uint16_t",
            4 => "uint32_t",
            8 => "uint64_t",
            _ => "unknown",
        }
    }
}
