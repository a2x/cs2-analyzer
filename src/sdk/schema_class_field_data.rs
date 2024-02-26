use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SchemaClassFieldData {
    pub name: Ptr<CStr>,  // 0x0000
    pub pad_0: [u8; 0x8], // 0x0008
    pub offset: u32,      // 0x0010
    pub pad_1: [u8; 0x8], // 0x0014
}

unsafe impl Pod for SchemaClassFieldData {}
