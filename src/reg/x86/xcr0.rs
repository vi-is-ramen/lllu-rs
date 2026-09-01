use core::ops::{BitAnd, BitOr, Not};

use crate::reg::Cast as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Xcr0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Xcr0Value {
    pub raw: u64,
}

impl From<u64> for Xcr0Value {
    fn from(raw: u64) -> Self {
        Self { raw }
    }
}

impl From<Xcr0Value> for u64 {
    fn from(value: Xcr0Value) -> Self {
        value.raw
    }
}

impl Xcr0 {
    /// Checks CPUID.01H:ECX.XSAVE (bit 26).
    ///
    /// Note: actually executing `XGETBV` / `XSETBV` also generally requires
    /// `CR4.OSXSAVE = 1` in the current OS context.
    #[inline(always)]
    pub fn supported() -> bool {
        #[cfg(target_env = "sgx")]
        {
            false
        }

        #[cfg(not(target_env = "sgx"))]
        {
            let cpuid = crate::ins::cpuid(1);
            cpuid.ecx & (1u32 << 26) != 0
        }
    }
}

impl super::Register<8> for Xcr0 {
    type Inner = u64;

    unsafe fn try_read(&self) -> Option<Self::Inner> {
        if !Self::supported() {
            return None;
        }

        Some(unsafe { self.read() })
    }

    unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()> {
        if !Self::supported() {
            return None;
        }

        unsafe { self.write(value) }
        Some(())
    }

    unsafe fn try_read_raw(&self) -> Option<[u8; 8]> {
        Some(unsafe { self.try_read()? }.cast())
    }

    unsafe fn try_write_raw(&mut self, value: [u8; 8]) -> Option<()> {
        unsafe { self.try_write(value.cast())? }
        Some(())
    }

    unsafe fn read(&self) -> Self::Inner {
        let low: u32;
        let high: u32;

        unsafe {
            core::arch::asm!(
                "xgetbv",
                in("ecx") 0u32,
                out("eax") low,
                out("edx") high,
                options(nostack, preserves_flags),
            );
        }

        ((high as u64) << 32) | (low as u64)
    }

    unsafe fn write(&mut self, value: Self::Inner) {
        let low = value as u32;
        let high = (value >> 32) as u32;

        unsafe {
            core::arch::asm!(
                "xsetbv",
                in("ecx") 0u32,
                in("eax") low,
                in("edx") high,
                options(nostack, preserves_flags),
            );
        }
    }

    unsafe fn read_raw(&self) -> [u8; 8] {
        unsafe { self.read() }.cast()
    }

    unsafe fn write_raw(&mut self, value: [u8; 8]) {
        unsafe { self.write(value.cast()) }
    }
}

impl Not for Xcr0Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { raw: !self.raw }
    }
}

impl BitOr for Xcr0Value {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self {
            raw: self.raw | other.raw,
        }
    }
}

impl BitAnd for Xcr0Value {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self {
            raw: self.raw & other.raw,
        }
    }
}

impl Into<bool> for Xcr0Value {
    fn into(self) -> bool {
        self.raw != 0
    }
}

impl Xcr0Value {
    pub const fn mask(self, rhs: Self) -> Self {
        Self {
            raw: self.raw & !rhs.raw,
        }
    }

    pub const fn set_bit(self, bit: u32) -> Self {
        Self {
            raw: self.raw | (1u64 << bit),
        }
    }

    pub const fn clear_bit(self, bit: u32) -> Self {
        Self {
            raw: self.raw & !(1u64 << bit),
        }
    }

    pub const fn toggle_bit(self, bit: u32) -> Self {
        Self {
            raw: self.raw ^ (1u64 << bit),
        }
    }

    pub const fn has_bit(self, bit: u32) -> bool {
        (self.raw & (1u64 << bit)) != 0
    }

    pub const fn x87(self) -> Self {
        self.set_bit(0)
    }

    pub const fn sse(self) -> Self {
        self.set_bit(1)
    }

    pub const fn avx(self) -> Self {
        self.set_bit(2)
    }

    pub const fn bndreg(self) -> Self {
        self.set_bit(3)
    }

    pub const fn bndcsr(self) -> Self {
        self.set_bit(4)
    }

    pub const fn opmask(self) -> Self {
        self.set_bit(5)
    }

    pub const fn zmm_hi256(self) -> Self {
        self.set_bit(6)
    }

    pub const fn hi16_zmm(self) -> Self {
        self.set_bit(7)
    }

    pub const fn pkru(self) -> Self {
        self.set_bit(9)
    }
}
