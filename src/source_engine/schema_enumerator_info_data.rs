use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SchemaEnumeratorInfoData {
    pub name: Ptr<CStr>,
    pub union_data: SchemaEnumeratorInfoDataUnion,
    pub metadata_size: u32,
    pad_0020: [u8; 0x8],
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
