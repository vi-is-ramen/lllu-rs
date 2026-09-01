use crate::reg::Cast;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
#[repr(u8)]
pub enum FlagShift {
    /// Carry Flag.
    CF = 0x00,
    /// I/O Trap (NEC V25/V35/V55 only), reserved and always 1 in EFLAGS on all
    /// other x86 processors.
    BRKI = 0x01,
    /// Parity Flag.
    PF = 0x02,
    // 03,
    /// Auxiliary Carry Flag.
    AF = 0x04,
    // 05,
    /// Zero Flag.
    ZF = 0x06,
    /// Sign Flag.
    SF = 0x07,
    /// Trap Flag.
    TF = 0x08,
    /// Interrupt Flag.
    IF = 0x09,
    /// Direction Flag.
    DF = 0x0A,
    /// Overflow Flag.
    OF = 0x0B,
    /// I/O Privilege Level Low Bit (286+ only), always 1 on 8086 and 186.
    IOPLLow = 0x0C,
    /// I/O Privilege Level High Bit (286+ only), always 1 on 8086 and 186.
    IOPLHigh = 0x0D,
    /// Nested Task Flag (286+ only), always 1 on 8086 and 186.
    NT = 0x0E,
    /// Mode flag (NEC V-series only), reserved on all Intel CPUs. Always 1 on
    /// 8086/186, 0 on 286 and later.
    MD = 0x0F,
    /// Resume flag (386+ only).
    RF = 0x10,
    /// Virtual 8086 mode flag (386+ only).
    VM = 0x11,
    /// Alignment Check (486+, ring 3), SMAP Access Check (Broadwell+, ring 0-2).
    AC = 0x12,
    /// Virtual interrupt flag (Pentium+).
    VIF = 0x13,
    /// Virtual interrupt pending (Pentium+).
    VIP = 0x14,
    /// Able to use CPUID instruction (Pentium+).
    ID = 0x15,
    // 0x16,
    // 0x17,
    // 0x18,
    // 0x19,
    // 0x1A,
    // 0x1B,
    // 0x1C,
    // 0x1D,
    /// AES Key Schedule Loaded Flag (VIA C3/C7 CPUs with VIA PadLock only).
    AES = 0x1E,
    /// "REX32" (an alternate to compatibility mode that allows access to 16
    /// GPRs).
    ///
    /// # ***OR:***
    /// Alternate Instruction Set enabled (VIA C5XL processors only).
    RX32 = 0x1F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Flag(usize);

impl Flag {
    pub const fn new() -> Self {
        Self(0)
    }
}

impl core::ops::Not for Flag {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl core::ops::BitAnd for Flag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitAnd<FlagShift> for Flag {
    type Output = Self;

    fn bitand(self, rhs: FlagShift) -> Self::Output {
        self & Self::from(rhs)
    }
}

impl core::ops::BitOr for Flag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl core::ops::BitOr<FlagShift> for Flag {
    type Output = Self;

    fn bitor(self, rhs: FlagShift) -> Self::Output {
        self | Self::from(rhs)
    }
}

impl From<FlagShift> for Flag {
    fn from(rhs: FlagShift) -> Self {
        Self(1 << rhs as usize)
    }
}

impl Cast<Flag> for [u8; const { size_of::<usize>() }] {
    fn cast(self) -> Flag {
        Flag(u64::from_le_bytes(self) as usize)
    }
}

impl Cast<[u8; const { size_of::<usize>() }]> for Flag {
    fn cast(self) -> [u8; const { size_of::<usize>() }] {
        self.0.to_le_bytes()
    }
}

pub struct Flags;

impl crate::reg::Register<{ size_of::<usize>() }> for Flags {
    type Inner = Flag;

    unsafe fn try_read(&self) -> Option<Self::Inner> {
        Some(unsafe { self.read() })
    }

    unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()> {
        Some(unsafe { self.write(value) })
    }

    unsafe fn try_read_raw(&self) -> Option<[u8; const { size_of::<usize>() }]> {
        Some(unsafe { self.read_raw() })
    }

    unsafe fn try_write_raw(&mut self, value: [u8; const { size_of::<usize>() }]) -> Option<()> {
        Some(unsafe { self.write_raw(value) })
    }

    unsafe fn read(&self) -> Self::Inner {
        let ret: usize;

        unsafe {
            #[cfg(target_arch = "x86")]
            core::arch::asm! {
                "pushf",
                "pop {0:e}",
                out(reg) ret
            }

            #[cfg(target_arch = "x86_64")]
            core::arch::asm! {
                "pushfq",
                "pop {0:r}",
                out(reg) ret
            }
        }

        Flag(ret)
    }

    unsafe fn write(&mut self, value: Self::Inner) {
        let value = value.0;

        unsafe {
            #[cfg(target_arch = "x86")]
            core::arch::asm! {
                "push {0:e}",
                "popf",
                in(reg) value
            }

            #[cfg(target_arch = "x86_64")]
            core::arch::asm! {
                "push {0:r}",
                "popfq",
                in(reg) value
            }
        }
    }

    unsafe fn read_raw(&self) -> [u8; const { size_of::<usize>() }] {
        unsafe { self.read() }.cast()
    }

    unsafe fn write_raw(&mut self, value: [u8; const { size_of::<usize>() }]) {
        unsafe { self.write(Flag(value.cast())) }
    }
}
