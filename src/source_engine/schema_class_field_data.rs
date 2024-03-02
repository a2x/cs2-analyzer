use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaFieldType;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SchemaClassFieldData {
    pub name: Ptr<CStr>,
    pub schema_type: u8,
    pad_0009: [u8; 0x7],
    pub offset: u32,
    pad_0020: [u8; 0x8],
}

impl SchemaClassFieldData {
    #[inline]
    pub fn schema_type(&self) -> Option<SchemaFieldType> {
        SchemaFieldType::try_from(self.schema_type).ok()
    }
}

unsafe impl Pod for SchemaClassFieldData {}
