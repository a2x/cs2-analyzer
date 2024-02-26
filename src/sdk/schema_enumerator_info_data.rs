use std::fmt;

use pelite::pe64::Ptr;
use pelite::util::CStr;
use pelite::Pod;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SchemaEnumeratorInfoData {
    pub name: Ptr<CStr>,                  // 0x0000
    pub u: SchemaEnumeratorInfoDataUnion, // 0x0008
    pub metadata_size: u32,               // 0x0010
    pub pad_0: [u8; 0x8],                 // 0x0014
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

impl fmt::Debug for SchemaEnumeratorInfoDataUnion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SchemaEnumeratorInfoDataUnion")
            .field("uchar", unsafe { &self.uchar })
            .field("ushort", unsafe { &self.ushort })
            .field("uint", unsafe { &self.uint })
            .field("ulong", unsafe { &self.ulong })
            .finish()
    }
}
