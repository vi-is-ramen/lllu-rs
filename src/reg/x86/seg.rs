use crate::reg::Cast as _;

macro_rules! impl_seg_value {
    ($val:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash,
        )]
        pub struct $val
        {
            pub raw: u16,
        }

        impl From<u16> for $val
        {
            fn from(raw: u16) -> Self
            {
                Self { raw }
            }
        }

        impl From<$val> for u16
        {
            fn from(value: $val) -> Self
            {
                value.raw
            }
        }

        impl $val
        {
            pub const fn new(raw: u16) -> Self
            {
                Self { raw }
            }
        }

        impl $val
        {
            /// Requested Privilege Level (bits 0-1)
            pub const fn rpl(self) -> u8
            {
                (self.raw & 0x3) as u8
            }

            pub const fn with_rpl(self, rpl: u8) -> Self
            {
                Self {
                    raw: (self.raw & !0x3) | ((rpl as u16) & 0x3),
                }
            }

            pub const fn gdt(self) -> bool
            {
                (self.raw & (1 << 2)) == 0
            }

            pub const fn ldt(self) -> bool
            {
                (self.raw & (1 << 2)) != 0
            }

            pub const fn with_gdt(self) -> Self
            {
                Self {
                    raw: self.raw & !(1 << 2),
                }
            }

            pub const fn with_ldt(self) -> Self
            {
                Self {
                    raw: self.raw | (1 << 2),
                }
            }

            /// Segment Index (bits 3-15)
            pub const fn index(self) -> u16
            {
                self.raw >> 3
            }

            pub const fn with_index(self, index: u16) -> Self
            {
                Self {
                    raw: (self.raw & 0x7) | (index << 3),
                }
            }

            /// Returns true if the selector points to the null descriptor
            /// (index 0, GDT). Note: RPL is ignored when checking for null.
            pub const fn is_null(self) -> bool
            {
                self.raw & !0x3 == 0
            }
        }
    };
}

macro_rules! impl_seg_reg {
    ($reg:ident, $val:ident, $read_asm:literal, $write_asm:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
        pub struct $reg;

        impl super::Register<2> for $reg {
            type Inner = u16;

            unsafe fn try_read(&self) -> Option<Self::Inner> {
                Some(unsafe { self.read() })
            }

            unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()> {
                unsafe { self.write(value) }
                Some(())
            }

            unsafe fn try_read_raw(&self) -> Option<[u8; 2]> {
                Some(unsafe { self.read() }.cast())
            }

            unsafe fn try_write_raw(&mut self, value: [u8; 2]) -> Option<()> {
                unsafe { self.write(value.cast()) }
                Some(())
            }

            unsafe fn read(&self) -> Self::Inner {
                let ret: u16;
                unsafe {
                    core::arch::asm!(
                        $read_asm,
                        out(reg) ret,
                        options(att_syntax),
                    );
                }
                ret
            }

            unsafe fn write(&mut self, value: Self::Inner) {
                unsafe {
                    core::arch::asm!(
                        $write_asm,
                        in(reg) value,
                        options(att_syntax),
                    );
                }
            }

            unsafe fn read_raw(&self) -> [u8; 2] {
                unsafe { self.read() }.cast()
            }

            unsafe fn write_raw(&mut self, value: [u8; 2]) {
                unsafe { self.write(value.cast()) }
            }
        }
    };
}

// ------------------------------------------------------------------
// Code Segment (CS)
// ------------------------------------------------------------------

impl_seg_value!(Segment);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Cs;

impl super::Register<2> for Cs
{
    type Inner = u16;

    unsafe fn try_read(&self) -> Option<Self::Inner>
    {
        Some(unsafe { self.read() })
    }

    unsafe fn try_write(&mut self, _value: Self::Inner) -> Option<()>
    {
        // Writing to CS directly via MOV is architecturally illegal and causes
        // #UD.
        None
    }

    unsafe fn try_read_raw(&self) -> Option<[u8; 2]>
    {
        Some(unsafe { self.read() }.cast())
    }

    unsafe fn try_write_raw(&mut self, _value: [u8; 2]) -> Option<()>
    {
        None
    }

    unsafe fn read(&self) -> Self::Inner
    {
        let ret: u16;
        unsafe {
            core::arch::asm!(
                "mov %cs, {0:x}",
                out(reg) ret,
                options(att_syntax),
            );
        }
        ret
    }

    unsafe fn write(&mut self, _value: Self::Inner)
    {
        panic!(
            "Cannot write to CS directly. Use a far jump, far call, or iret."
        );
    }

    unsafe fn read_raw(&self) -> [u8; 2]
    {
        unsafe { self.read() }.cast()
    }

    unsafe fn write_raw(&mut self, _value: [u8; 2])
    {
        panic!(
            "Cannot write to CS directly. Use a far jump, far call, or iret."
        );
    }
}

// ------------------------------------------------------------------
// Data / Stack / Extra Segments
// ------------------------------------------------------------------

impl_seg_reg!(Ds, Segment, "mov %ds, {0:x}", "mov {0:x}, %ds");
impl_seg_reg!(Ss, Segment, "mov %ss, {0:x}", "mov {0:x}, %ss");
impl_seg_reg!(Es, Segment, "mov %es, {0:x}", "mov {0:x}, %es");
impl_seg_reg!(Fs, Segment, "mov %fs, {0:x}", "mov {0:x}, %fs");
impl_seg_reg!(Gs, Segment, "mov %gs, {0:x}", "mov {0:x}, %gs");
