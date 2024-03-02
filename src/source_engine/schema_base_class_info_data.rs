use pelite::pe64::Ptr;
use pelite::Pod;

use super::SchemaClassInfoData;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SchemaBaseClassInfoData {
    pub offset: u32,
    pub prev: Ptr<SchemaClassInfoData>,
}

unsafe impl Pod for SchemaBaseClassInfoData {}
