use pelite::pe64::{Ptr, Va};
use pelite::util::CStr;
use pelite::Pod;

#[derive(Pod)]
#[repr(C)]
pub struct SchemaMetadataEntryData {
    pub name: Ptr<CStr>,   // 0x0000
    pub function: Ptr<Va>, // 0x0008
}
