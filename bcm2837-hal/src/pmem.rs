//! Physical memory wrapper

// TODO - replace this with DMA utils like in https://github.com/astro/stm32f429-hal
// no mmu/vaddr's, can use normal slices/memory

use crate::cache::bus_address_bits;
use core::fmt;

/// A pmem error type
#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidSize,
}

#[derive(Debug, Copy, Clone)]
pub struct PMem {
    paddr: u32,
    size: usize,
}

impl PMem {
    pub fn new(paddr: u32, size: usize) -> Self {
        assert_ne!(paddr, 0);
        assert_ne!(size, 0);
        PMem { paddr, size }
    }

    pub fn paddr(&self) -> u32 {
        self.paddr
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn bus_paddr(&self) -> u32 {
        self.paddr | bus_address_bits::ALIAS_4_L2_COHERENT
    }

    pub fn contains_paddr(&self, paddr: u32) -> bool {
        if self.size == 0 {
            false
        } else {
            if (paddr >= self.paddr) && (paddr < (self.paddr + self.size as u32)) {
                true
            } else {
                false
            }
        }
    }

    pub fn split_off(&mut self, at: usize) -> Result<Self, Error> {
        if at >= self.size {
            Err(Error::InvalidSize)
        } else {
            let mut tail_region = self.clone();

            // New region consumes tail
            tail_region.paddr += at as u32;
            tail_region.size -= at;

            // Our region consumes the head, ending at the offset
            self.size = at;

            Ok(tail_region)
        }
    }

    pub fn shrink_to(&mut self, size: usize) -> Result<(), Error> {
        if size > self.size {
            Err(Error::InvalidSize)
        } else {
            self.size = size;
            Ok(())
        }
    }

    pub unsafe fn as_slice<T>(&self, count: usize) -> &[T] {
        core::slice::from_raw_parts(self.as_ptr(), count)
    }

    pub unsafe fn as_mut_slice<T>(&self, count: usize) -> &mut [T] {
        core::slice::from_raw_parts_mut(self.as_mut_ptr(), count)
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.paddr as *const T
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.paddr as *mut T
    }
}

impl fmt::Display for PMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PMem {{ paddr {:#010X} size: {:#010X} }}",
            self.paddr(),
            self.size(),
        )
    }
}
