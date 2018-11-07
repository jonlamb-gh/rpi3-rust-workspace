use super::MMIO_BASE;

use core::marker::PhantomData;
use core::ops::Deref;
use register::mmio::{ReadOnly, WriteOnly};

register_bitfields! {
    u32,

    STATUS [
        FULL  OFFSET(31) NUMBITS(1) [],
        EMPTY OFFSET(30) NUMBITS(1) []
    ]
}

const VIDEOCORE_MBOX: u32 = MMIO_BASE + 0xB880;

#[allow(non_snake_case)]
#[repr(C)]
pub struct RegisterBlock {
    READ: ReadOnly<u32>,                     // 0x00
    __reserved_0: [u32; 5],                  // 0x04
    STATUS: ReadOnly<u32, STATUS::Register>, // 0x18
    __reserved_1: u32,                       // 0x1C
    WRITE: WriteOnly<u32>,                   // 0x20
}

pub struct MBOX {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MBOX {}

impl MBOX {
    pub fn ptr() -> *const RegisterBlock {
        VIDEOCORE_MBOX as *const _
    }
}

impl Deref for MBOX {
    type Target = RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}
