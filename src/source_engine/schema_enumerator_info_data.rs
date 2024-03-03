use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaMetadataEntryData;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SchemaEnumeratorInfoData {
    pub name: Ptr<CStr>,
    pub union_data: SchemaEnumeratorInfoDataUnion,
    pub metadata_count: u32,
    pub metadata: Ptr<SchemaMetadataEntryData>,
}

unsafe impl Pod for SchemaEnumeratorInfoData {}

#[derive(Clone, Copy)]
#[repr(C)]
pub union SchemaEnumeratorInfoDataUnion {
    pub uchar: u8,
    pub ushort: u16,
    pub uint: u32,
    pub ulong: u64,
}
