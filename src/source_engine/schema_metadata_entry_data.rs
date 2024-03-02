use pelite::pe64::{Ptr, Va};
use pelite::util::CStr;
use pelite::Pod;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SchemaMetadataEntryData {
    pub name: Ptr<CStr>,
    pub function: Ptr<Va>,
}

unsafe impl Pod for SchemaMetadataEntryData {}
