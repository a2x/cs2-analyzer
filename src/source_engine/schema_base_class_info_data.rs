use pelite::pe64::Ptr;
use pelite::Pod;

use super::SchemaClassInfoData;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SchemaBaseClassInfoData {
    pub offset: u32,                    // 0x0000
    pub prev: Ptr<SchemaClassInfoData>, // 0x0004
}

unsafe impl Pod for SchemaBaseClassInfoData {}
