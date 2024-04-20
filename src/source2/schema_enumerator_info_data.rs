use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaMetadataEntryData;

#[derive(Pod)]
#[repr(C)]
pub struct SchemaEnumeratorInfoData {
    pub name: Ptr<CStr>,                               // 0x0000
    pub value: SchemaEnumeratorInfoDataUnion,          // 0x0008
    pub static_metadata_count: i32,                    // 0x0010
    pad_0014: [u8; 0x4],                               // 0x0014
    pub static_metadata: Ptr<SchemaMetadataEntryData>, // 0x0018
}

#[repr(C)]
pub union SchemaEnumeratorInfoDataUnion {
    pub uchar: u8,
    pub ushort: u16,
    pub uint: u32,
    pub ulong: u64,
}

unsafe impl Pod for SchemaEnumeratorInfoDataUnion {}
