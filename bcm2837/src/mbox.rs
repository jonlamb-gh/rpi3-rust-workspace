//! VideoCore Mailbox

use crate::MMIO_BASE;
use core::ops::{Deref, DerefMut};
use register::{mmio::ReadOnly, mmio::WriteOnly, register_bitfields};

pub const BASE_PADDR: u32 = MMIO_BASE + 0xB000;
pub const BASE_OFFSET: u32 = 0x0880;
pub const PADDR: u32 = BASE_PADDR + BASE_OFFSET;

register_bitfields! {
    u32,

    STATUS [
        FULL  OFFSET(31) NUMBITS(1) [],
        EMPTY OFFSET(30) NUMBITS(1) []
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub READ: ReadOnly<u32>,                     // 0x00
    __reserved_0: [u32; 5],                      // 0x04
    pub STATUS: ReadOnly<u32, STATUS::Register>, // 0x18
    __reserved_1: u32,                           // 0x1C
    pub WRITE: WriteOnly<u32>,                   // 0x20
    __reserved_2: [u32; 5],                      // 0x24
}

pub struct MBOX {
    addr: *const u32,
}

impl From<u32> for MBOX {
    fn from(addr: u32) -> MBOX {
        assert_ne!(addr, 0);
        MBOX {
            addr: addr as *const u32,
        }
    }
}

unsafe impl Send for MBOX {}

impl MBOX {
    pub fn as_ptr(&self) -> *const RegisterBlock {
        self.addr as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut RegisterBlock {
        self.addr as *mut _
    }
}

impl Deref for MBOX {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for MBOX {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
