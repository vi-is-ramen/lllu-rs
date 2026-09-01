use core::mem::size_of;
use core::ops::{BitAnd, BitOr, Not};

use crate::reg::Cast as _;

macro_rules! impl_reg_value {
    ($val:ident) => {
        impl From<usize> for $val
        {
            fn from(raw: usize) -> Self
            {
                Self { raw }
            }
        }

        impl From<$val> for usize
        {
            fn from(value: $val) -> Self
            {
                value.raw
            }
        }

        impl Not for $val
        {
            type Output = Self;

            fn not(self) -> Self::Output
            {
                Self { raw: !self.raw }
            }
        }

        impl BitOr for $val
        {
            type Output = Self;

            fn bitor(self, other: Self) -> Self::Output
            {
                Self {
                    raw: self.raw | other.raw,
                }
            }
        }

        impl BitAnd for $val
        {
            type Output = Self;

            fn bitand(self, other: Self) -> Self::Output
            {
                Self {
                    raw: self.raw & other.raw,
                }
            }
        }

        impl Into<bool> for $val
        {
            fn into(self) -> bool
            {
                self.raw != 0
            }
        }

        impl $val
        {
            pub const fn mask(self, rhs: Self) -> Self
            {
                Self {
                    raw: self.raw & !rhs.raw,
                }
            }

            pub const fn set_bit(self, bit: usize) -> Self
            {
                Self {
                    raw: self.raw | (1usize << bit),
                }
            }

            pub const fn clear_bit(self, bit: usize) -> Self
            {
                Self {
                    raw: self.raw & !(1usize << bit),
                }
            }

            pub const fn toggle_bit(self, bit: usize) -> Self
            {
                Self {
                    raw: self.raw ^ (1usize << bit),
                }
            }

            pub const fn has_bit(self, bit: usize) -> bool
            {
                (self.raw & (1usize << bit)) != 0
            }
        }
    };
}

macro_rules! impl_mov_reg {
    ($reg:ident, $val:ident, $read_asm:literal, $write_asm:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
        pub struct $reg;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
        pub struct $val {
            pub raw: usize,
        }

        impl_reg_value!($val);

        impl super::Register<{ size_of::<usize>() }> for $reg {
            type Inner = usize;

            unsafe fn try_read(&self) -> Option<Self::Inner> {
                Some(unsafe { self.read() })
            }

            unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()> {
                unsafe { self.write(value) }
                Some(())
            }

            unsafe fn try_read_raw(&self) -> Option<[u8; size_of::<usize>()]> {
                Some(unsafe { self.read() }.cast())
            }

            unsafe fn try_write_raw(&mut self, value: [u8; size_of::<usize>()]) -> Option<()> {
                unsafe { self.write(value.cast()) }
                Some(())
            }

            unsafe fn read(&self) -> Self::Inner {
                let ret: usize;
                unsafe {
                    core::arch::asm!(
                        $read_asm,
                        out(reg) ret,
                    );
                }
                ret
            }

            unsafe fn write(&mut self, value: Self::Inner) {
                unsafe {
                    core::arch::asm!(
                        $write_asm,
                        in(reg) value,
                    );
                }
            }

            unsafe fn read_raw(&self) -> [u8; size_of::<usize>()] {
                unsafe { self.read() }.cast()
            }

            unsafe fn write_raw(&mut self, value: [u8; size_of::<usize>()]) {
                unsafe { self.write(value.cast()) }
            }
        }
    };
}

/// CR1 is architecturally reserved.
///
/// `try_*` operations return `None`.
/// Direct `read`/`write` operations panic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Cr1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Cr1Value
{
    pub raw: usize,
}

impl_reg_value!(Cr1Value);

impl super::Register<{ size_of::<usize>() }> for Cr1
{
    type Inner = usize;

    unsafe fn try_read(&self) -> Option<Self::Inner>
    {
        None
    }

    unsafe fn try_write(&mut self, _value: Self::Inner) -> Option<()>
    {
        None
    }

    unsafe fn try_read_raw(&self) -> Option<[u8; size_of::<usize>()]>
    {
        None
    }

    unsafe fn try_write_raw(
        &mut self,
        _value: [u8; size_of::<usize>()],
    ) -> Option<()>
    {
        None
    }

    unsafe fn read(&self) -> Self::Inner
    {
        panic!("CR1 is reserved")
    }

    unsafe fn write(&mut self, _value: Self::Inner)
    {
        panic!("CR1 is reserved")
    }

    unsafe fn read_raw(&self) -> [u8; size_of::<usize>()]
    {
        panic!("CR1 is reserved")
    }

    unsafe fn write_raw(&mut self, _value: [u8; size_of::<usize>()])
    {
        panic!("CR1 is reserved")
    }
}

// ------------------------------------------------------------------
// Control registers
// ------------------------------------------------------------------

impl_mov_reg!(Cr0, Cr0Value, "mov {0}, cr0", "mov cr0, {0}");
impl_mov_reg!(Cr2, Cr2Value, "mov {0}, cr2", "mov cr2, {0}");
impl_mov_reg!(Cr3, Cr3Value, "mov {0}, cr3", "mov cr3, {0}");
impl_mov_reg!(Cr4, Cr4Value, "mov {0}, cr4", "mov cr4, {0}");

// CR8 is only available in 64-bit mode.
#[cfg(target_arch = "x86_64")]
impl_mov_reg!(Cr8, Cr8Value, "mov {0}, cr8", "mov cr8, {0}");

// ------------------------------------------------------------------
// Debug registers
// ------------------------------------------------------------------

impl_mov_reg!(Dr0, Dr0Value, "mov {0}, db0", "mov db0, {0}");
impl_mov_reg!(Dr1, Dr1Value, "mov {0}, db1", "mov db1, {0}");
impl_mov_reg!(Dr2, Dr2Value, "mov {0}, db2", "mov db2, {0}");
impl_mov_reg!(Dr3, Dr3Value, "mov {0}, db3", "mov db3, {0}");

// DR4/DR5 are conditional/aliased depending on CR4.DE.
impl_mov_reg!(Dr4, Dr4Value, "mov {0}, db4", "mov db4, {0}");
impl_mov_reg!(Dr5, Dr5Value, "mov {0}, db5", "mov db5, {0}");

impl_mov_reg!(Dr6, Dr6Value, "mov {0}, db6", "mov db6, {0}");
impl_mov_reg!(Dr7, Dr7Value, "mov {0}, db7", "mov db7, {0}");

// ------------------------------------------------------------------
// Control registers manipulation
// ------------------------------------------------------------------

impl Cr0Value
{
    pub const fn pe(self) -> Self
    {
        self.set_bit(0)
    }
    pub const fn protection_enable(self) -> Self
    {
        self.set_bit(0)
    }
    pub const fn mp(self) -> Self
    {
        self.set_bit(1)
    }
    pub const fn monitor_coprocessor(self) -> Self
    {
        self.set_bit(1)
    }
    pub const fn em(self) -> Self
    {
        self.set_bit(2)
    }
    pub const fn emulate_coprocessor(self) -> Self
    {
        self.set_bit(2)
    }
    pub const fn ts(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn task_switched(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn et(self) -> Self
    {
        self.set_bit(4)
    }
    pub const fn extension_type(self) -> Self
    {
        self.set_bit(4)
    }
    pub const fn ne(self) -> Self
    {
        self.set_bit(5)
    }
    pub const fn numeric_error(self) -> Self
    {
        self.set_bit(5)
    }
    pub const fn wp(self) -> Self
    {
        self.set_bit(16)
    }
    pub const fn write_protect(self) -> Self
    {
        self.set_bit(16)
    }
    pub const fn am(self) -> Self
    {
        self.set_bit(18)
    }
    pub const fn alignment_mask(self) -> Self
    {
        self.set_bit(18)
    }
    pub const fn nw(self) -> Self
    {
        self.set_bit(29)
    }
    pub const fn not_write_through(self) -> Self
    {
        self.set_bit(29)
    }
    pub const fn cd(self) -> Self
    {
        self.set_bit(30)
    }
    pub const fn cache_disable(self) -> Self
    {
        self.set_bit(30)
    }
    pub const fn pg(self) -> Self
    {
        self.set_bit(31)
    }
    pub const fn paging(self) -> Self
    {
        self.set_bit(31)
    }
}

impl Cr2Value
{
    // CR2 holds the page fault linear address.
    // It does not have individual feature bits, but you can extract the
    // address:
    pub const fn address(self) -> usize
    {
        self.raw
    }
}

impl Cr3Value
{
    pub const fn pwt(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn page_level_write_through(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn pcd(self) -> Self
    {
        self.set_bit(4)
    }
    pub const fn page_level_cache_disable(self) -> Self
    {
        self.set_bit(4)
    }

    /// Sets the PCID (Process-Context Identifier) in the lower 12 bits.
    /// Only valid if CR4.PCIDE is set.
    pub const fn pcid(self, pcid: u16) -> Self
    {
        Self {
            raw: (self.raw & !0xFFF) | ((pcid as usize) & 0xFFF),
        }
    }

    /// Sets the page directory base address (clears lower 12 bits and flags).
    pub const fn base_address(self, addr: usize) -> Self
    {
        Self {
            // Preserve PWT (bit 3) and PCD (bit 4)
            raw: (addr & !0xFFF) | (self.raw & 0x18),
        }
    }
}

impl Cr4Value
{
    pub const fn vme(self) -> Self
    {
        self.set_bit(0)
    }
    pub const fn virtual_8086_mode_extensions(self) -> Self
    {
        self.set_bit(0)
    }
    pub const fn pvi(self) -> Self
    {
        self.set_bit(1)
    }
    pub const fn protected_mode_virtual_interrupts(self) -> Self
    {
        self.set_bit(1)
    }
    pub const fn tsd(self) -> Self
    {
        self.set_bit(2)
    }
    pub const fn time_stamp_disable(self) -> Self
    {
        self.set_bit(2)
    }
    pub const fn de(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn debugging_extensions(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn pse(self) -> Self
    {
        self.set_bit(4)
    }
    pub const fn page_size_extensions(self) -> Self
    {
        self.set_bit(4)
    }
    pub const fn pae(self) -> Self
    {
        self.set_bit(5)
    }
    pub const fn physical_address_extension(self) -> Self
    {
        self.set_bit(5)
    }
    pub const fn mce(self) -> Self
    {
        self.set_bit(6)
    }
    pub const fn machine_check_enable(self) -> Self
    {
        self.set_bit(6)
    }
    pub const fn pge(self) -> Self
    {
        self.set_bit(7)
    }
    pub const fn page_global_enable(self) -> Self
    {
        self.set_bit(7)
    }
    pub const fn pce(self) -> Self
    {
        self.set_bit(8)
    }
    pub const fn performance_monitoring_counter_enable(self) -> Self
    {
        self.set_bit(8)
    }
    pub const fn osfxsr(self) -> Self
    {
        self.set_bit(9)
    }
    pub const fn os_support_fxsave_fxrstor(self) -> Self
    {
        self.set_bit(9)
    }
    pub const fn osxmmexcpt(self) -> Self
    {
        self.set_bit(10)
    }
    pub const fn os_support_unmasked_simd_floating_point_exceptions(
        self,
    ) -> Self
    {
        self.set_bit(10)
    }
    pub const fn umip(self) -> Self
    {
        self.set_bit(11)
    }
    pub const fn user_mode_instruction_prevention(self) -> Self
    {
        self.set_bit(11)
    }
    pub const fn la57(self) -> Self
    {
        self.set_bit(12)
    }
    pub const fn linear_addresses_57(self) -> Self
    {
        self.set_bit(12)
    }
    pub const fn vmxe(self) -> Self
    {
        self.set_bit(13)
    }
    pub const fn vmx_enable(self) -> Self
    {
        self.set_bit(13)
    }
    pub const fn smxe(self) -> Self
    {
        self.set_bit(14)
    }
    pub const fn smx_enable(self) -> Self
    {
        self.set_bit(14)
    }
    pub const fn fsgsbase(self) -> Self
    {
        self.set_bit(16)
    }
    pub const fn fs_gs_base_access(self) -> Self
    {
        self.set_bit(16)
    }
    pub const fn pcide(self) -> Self
    {
        self.set_bit(17)
    }
    pub const fn pcid_enable(self) -> Self
    {
        self.set_bit(17)
    }
    pub const fn osxsave(self) -> Self
    {
        self.set_bit(18)
    }
    pub const fn xsave_enable(self) -> Self
    {
        self.set_bit(18)
    }
    pub const fn smep(self) -> Self
    {
        self.set_bit(20)
    }
    pub const fn supervisor_mode_execution_prevention(self) -> Self
    {
        self.set_bit(20)
    }
    pub const fn smap(self) -> Self
    {
        self.set_bit(21)
    }
    pub const fn supervisor_mode_access_prevention(self) -> Self
    {
        self.set_bit(21)
    }
    pub const fn pke(self) -> Self
    {
        self.set_bit(22)
    }
    pub const fn protection_key_enable(self) -> Self
    {
        self.set_bit(22)
    }
    pub const fn cet(self) -> Self
    {
        self.set_bit(23)
    }
    pub const fn control_flow_enforcement_technology(self) -> Self
    {
        self.set_bit(23)
    }
    pub const fn pks(self) -> Self
    {
        self.set_bit(24)
    }
    pub const fn protection_keys_for_supervisor_mode_pages(self) -> Self
    {
        self.set_bit(24)
    }
}

#[cfg(target_arch = "x86_64")]
impl Cr8Value
{
    /// Sets the Task Priority Register (TPR) value (bits 0-3).
    pub const fn tpr(self, priority: u8) -> Self
    {
        Self {
            raw: (self.raw & !0xF) | ((priority as usize) & 0xF),
        }
    }

    /// Gets the Task Priority Register (TPR) value.
    pub const fn get_tpr(self) -> u8
    {
        (self.raw & 0xF) as u8
    }
}

// ------------------------------------------------------------------
// Debug registers manipulation
// ------------------------------------------------------------------

impl Dr6Value
{
    pub const fn b0(self) -> Self
    {
        self.set_bit(0)
    }
    pub const fn b1(self) -> Self
    {
        self.set_bit(1)
    }
    pub const fn b2(self) -> Self
    {
        self.set_bit(2)
    }
    pub const fn b3(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn bd(self) -> Self
    {
        self.set_bit(13)
    }
    pub const fn debug_register_access_detected(self) -> Self
    {
        self.set_bit(13)
    }
    pub const fn bs(self) -> Self
    {
        self.set_bit(14)
    }
    pub const fn single_step(self) -> Self
    {
        self.set_bit(14)
    }
    pub const fn bt(self) -> Self
    {
        self.set_bit(15)
    }
    pub const fn task_switch(self) -> Self
    {
        self.set_bit(15)
    }
    pub const fn rtm(self) -> Self
    {
        self.set_bit(16)
    }
    pub const fn restricted_transactional_memory(self) -> Self
    {
        self.set_bit(16)
    }

    pub const fn has_b0(self) -> bool
    {
        self.has_bit(0)
    }
    pub const fn has_b1(self) -> bool
    {
        self.has_bit(1)
    }
    pub const fn has_b2(self) -> bool
    {
        self.has_bit(2)
    }
    pub const fn has_b3(self) -> bool
    {
        self.has_bit(3)
    }
    pub const fn has_bd(self) -> bool
    {
        self.has_bit(13)
    }
    pub const fn has_bs(self) -> bool
    {
        self.has_bit(14)
    }
    pub const fn has_bt(self) -> bool
    {
        self.has_bit(15)
    }
    pub const fn has_rtm(self) -> bool
    {
        self.has_bit(16)
    }
}

impl Dr7Value
{
    pub const fn l0(self) -> Self
    {
        self.set_bit(0)
    }
    pub const fn g0(self) -> Self
    {
        self.set_bit(1)
    }
    pub const fn l1(self) -> Self
    {
        self.set_bit(2)
    }
    pub const fn g1(self) -> Self
    {
        self.set_bit(3)
    }
    pub const fn l2(self) -> Self
    {
        self.set_bit(4)
    }
    pub const fn g2(self) -> Self
    {
        self.set_bit(5)
    }
    pub const fn l3(self) -> Self
    {
        self.set_bit(6)
    }
    pub const fn g3(self) -> Self
    {
        self.set_bit(7)
    }
    pub const fn le(self) -> Self
    {
        self.set_bit(8)
    }
    pub const fn ge(self) -> Self
    {
        self.set_bit(9)
    }
    pub const fn gd(self) -> Self
    {
        self.set_bit(13)
    }
    pub const fn general_detect_enable(self) -> Self
    {
        self.set_bit(13)
    }

    /// Sets the condition and length for breakpoint 0.
    /// `rw` is 2 bits (0-3), `len` is 2 bits (0-3).
    pub const fn bp0(self, rw: u8, len: u8) -> Self
    {
        let mask = !(0xFusize << 16);
        let val = (((rw & 0x3) as usize) | (((len & 0x3) as usize) << 2)) << 16;
        Self {
            raw: (self.raw & mask) | val,
        }
    }

    /// Sets the condition and length for breakpoint 1.
    pub const fn bp1(self, rw: u8, len: u8) -> Self
    {
        let mask = !(0xFusize << 20);
        let val = (((rw & 0x3) as usize) | (((len & 0x3) as usize) << 2)) << 20;
        Self {
            raw: (self.raw & mask) | val,
        }
    }

    /// Sets the condition and length for breakpoint 2.
    pub const fn bp2(self, rw: u8, len: u8) -> Self
    {
        let mask = !(0xFusize << 24);
        let val = (((rw & 0x3) as usize) | (((len & 0x3) as usize) << 2)) << 24;
        Self {
            raw: (self.raw & mask) | val,
        }
    }

    /// Sets the condition and length for breakpoint 3.
    pub const fn bp3(self, rw: u8, len: u8) -> Self
    {
        let mask = !(0xFusize << 28);
        let val = (((rw & 0x3) as usize) | (((len & 0x3) as usize) << 2)) << 28;
        Self {
            raw: (self.raw & mask) | val,
        }
    }
}
