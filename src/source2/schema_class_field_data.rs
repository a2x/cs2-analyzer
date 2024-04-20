use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

use super::SchemaFieldType;

#[derive(Pod)]
#[repr(C)]
pub struct SchemaClassFieldData {
    pub name: Ptr<CStr>,                // 0x0000
    pub schema_type: u8,                // 0x0008
    pad_0009: [u8; 0x7],                // 0x0009
    pub single_inheritance_offset: i32, // 0x0010
    pad_0014: [u8; 0xC],                // 0x0014
}

impl SchemaClassFieldData {
    #[inline]
    pub fn schema_type(&self) -> Option<SchemaFieldType> {
        SchemaFieldType::try_from(self.schema_type).ok()
    }
}
