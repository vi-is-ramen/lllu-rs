use core::ops::{BitAnd, BitOr, Not};

use crate::reg::Cast as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Cr0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Cr0Value {
    pub raw: usize,
}

impl From<usize> for Cr0Value {
    fn from(raw: usize) -> Self {
        Self { raw }
    }
}

impl super::Register<{ size_of::<usize>() }> for Cr0 {
    type Inner = usize;

    unsafe fn try_read(&self) -> Option<Self::Inner> {
        Some(unsafe { self.read() })
    }

    unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()> {
        unsafe { self.write(value) }
        Some(())
    }

    unsafe fn try_read_raw(&self) -> Option<[u8; 8]> {
        Some(unsafe { self.read() }.cast())
    }

    unsafe fn try_write_raw(&mut self, value: [u8; 8]) -> Option<()> {
        unsafe { self.write(value.cast()) }
        Some(())
    }

    unsafe fn read(&self) -> Self::Inner {
        let ret: usize;
        unsafe {
            core::arch::asm!("mov %cr0, {0}", out(reg) ret, options(att_syntax));
        }
        ret
    }

    unsafe fn write(&mut self, value: Self::Inner) {
        unsafe {
            core::arch::asm!("mov {0}, %cr0", in(reg) value, options(att_syntax));
        }
    }

    unsafe fn read_raw(&self) -> [u8; 8] {
        unsafe { self.read() }.cast()
    }

    unsafe fn write_raw(&mut self, value: [u8; 8]) {
        unsafe { self.write(value.cast()) }
    }
}

impl Not for Cr0Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { raw: !self.raw }
    }
}

impl BitOr for Cr0Value {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self {
            raw: self.raw | other.raw,
        }
    }
}

impl BitAnd for Cr0Value {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self {
            raw: self.raw & other.raw,
        }
    }
}

impl Into<bool> for Cr0Value {
    fn into(self) -> bool {
        self.raw != 0
    }
}

impl Cr0Value {
    pub const fn enable_paging(self) -> Self {
        Self {
            raw: self.raw | 1 << 31,
        }
    }

    pub const fn cache_disable(self) -> Self {
        Self {
            raw: self.raw | 1 << 30,
        }
    }

    pub const fn not_write_through(self) -> Self {
        Self {
            raw: self.raw | 1 << 29,
        }
    }

    pub const fn alignment_mask(self) -> Self {
        Self {
            raw: self.raw | 1 << 18,
        }
    }

    pub const fn write_protet(self) -> Self {
        Self {
            raw: self.raw | 1 << 16,
        }
    }

    pub const fn numeric_error(self) -> Self {
        Self {
            raw: self.raw | 1 << 5,
        }
    }

    pub const fn extension_type(self) -> Self {
        Self {
            raw: self.raw | 1 << 4,
        }
    }

    pub const fn task_switched(self) -> Self {
        Self {
            raw: self.raw | 1 << 3,
        }
    }

    pub const fn emulate_coprocessor(self) -> Self {
        Self {
            raw: self.raw | 1 << 2,
        }
    }

    pub const fn monitor_coprocessor(self) -> Self {
        Self {
            raw: self.raw | 1 << 1,
        }
    }

    pub const fn protected_mode(self) -> Self {
        Self { raw: self.raw | 1 }
    }

    pub const fn mask(self, rhs: Self) -> Self {
        Self {
            raw: self.raw & !rhs.raw,
        }
    }
}
