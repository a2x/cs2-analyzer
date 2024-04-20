use pelite::pe64::Ptr;
use pelite::Pod;

use super::SchemaClassInfoData;

#[derive(Pod)]
#[repr(C)]
pub struct SchemaBaseClassInfoData {
    pub offset: u32,                    // 0x0000
    pad_0004: [u8; 4],                  // 0x0004
    pub prev: Ptr<SchemaClassInfoData>, // 0x0008
}
