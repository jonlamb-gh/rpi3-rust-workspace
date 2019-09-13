//! RNG

use crate::MMIO_BASE;
use core::ops::{Deref, DerefMut};
use register::{mmio::ReadOnly, mmio::ReadWrite, register_bitfields};

pub const PADDR: u32 = MMIO_BASE + 0x0010_4000;

register_bitfields! {
    u32,

    CTRL [
        ENABLE OFFSET(0) NUMBITS(1) [
            True = 1,
            False = 0
        ]
    ],

    STATUS [
        COUNT OFFSET(0) NUMBITS(24) [],
        READY OFFSET(24) NUMBITS(1) [
            True = 1,
            False = 0
        ]
    ],

    INT_MASK [
        INT_OFF OFFSET(0) NUMBITS(1) [
            True = 1,
            False = 0
        ]
    ]
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    pub CTRL: ReadWrite<u32, CTRL::Register>,         // 0x00
    pub STATUS: ReadWrite<u32, STATUS::Register>,     // 0x04
    pub DATA: ReadOnly<u32>,                          // 0x08
    __reserved_0: u32,                                // 0x0c
    pub INT_MASK: ReadWrite<u32, INT_MASK::Register>, // 0x10
}

pub struct RNG {
    addr: *const u32,
}

impl From<u32> for RNG {
    fn from(addr: u32) -> RNG {
        assert_ne!(addr, 0);
        RNG {
            addr: addr as *const u32,
        }
    }
}

unsafe impl Send for RNG {}

impl RNG {
    pub fn as_ptr(&self) -> *const RegisterBlock {
        self.addr as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut RegisterBlock {
        self.addr as *mut _
    }
}

impl Deref for RNG {
    type Target = RegisterBlock;
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*self.as_ptr() }
    }
}

impl DerefMut for RNG {
    fn deref_mut(&mut self) -> &mut RegisterBlock {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
