#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    _rdrand16_step, _rdrand32_step, _rdrand64_step, _rdseed16_step,
    _rdseed32_step, _rdseed64_step,
};

#[cfg(target_arch = "x86")]
use core::arch::x86::{
    _rdrand16_step, _rdrand32_step, _rdseed16_step, _rdseed32_step,
};

/// Generates a 16-bit random value and stores it in `e`.
///
/// # Safety
/// Will crash if RDRAND instructions are not supported.
#[inline(always)]
pub unsafe fn rdrand16(e: &mut u16) -> bool
{
    unsafe { _rdrand16_step(e) == 1 }
}

/// Generates a 32-bit random value and stores it in `e`.
///
/// # Safety
/// Will crash if RDRAND instructions are not supported.
#[inline(always)]
pub unsafe fn rdrand32(e: &mut u32) -> bool
{
    unsafe { _rdrand32_step(e) == 1 }
}

/// Generates a 64-bit random value and stores it in `e`.
///
/// # Safety
/// Will crash if RDRAND instructions are not supported.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rdrand64(e: &mut u64) -> bool
{
    unsafe { _rdrand64_step(e) == 1 }
}

/// Generates a 64-bit random value and stores it in `e`.
///
/// This function uses the RDSEED instruction twice to generate two 32-bit
/// halves of the 64-bit value. First result is low half, second is high.
///
/// # Safety
/// Will crash if RDRAND instructions are not supported.
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn rdrand64(e: &mut u64) -> bool
{
    let (mut a, mut b): (u32, u32);
    unsafe { _rdrand32_step(&mut a) == 1 };
    let ret = unsafe { _rdrand32_step(&mut b) == 1 };
    e = a & (b << 32);
    ret
}

/// RdRand trait to implement the generic rdrand_slice function.
pub trait RdRand
{
    /// Fills `self` with random bits. Returns true on success or false
    /// otherwise.
    ///
    /// # Safety
    /// RDRAND is not supported on all architectures, so using this may crash
    /// you.
    unsafe fn fill_random(&mut self) -> bool;
}

impl RdRand for u8
{
    /// Fills the 16-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        let mut r: u16 = 0;
        let ret = unsafe { rdrand16(&mut r) };
        *self = r as u8;
        ret
    }
}

impl RdRand for u16
{
    /// Fills the 16-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDRAND instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        unsafe { rdrand16(self) }
    }
}

impl RdRand for u32
{
    /// Fills the 32-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDRAND instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        unsafe { rdrand32(self) }
    }
}

impl RdRand for u64
{
    /// Fills the 64-bit value with a random bit string.
    ///
    /// This function uses the RDSEED instruction twice to generate two 32-bit
    /// halves of the 64-bit value. First result is low half, second is high.
    ///
    /// # Safety
    /// Will crash if RDRAND instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        unsafe { rdrand64(self) }
    }
}

impl<T> RdRand for [T]
where T: RdRand
{
    /// Fills the 64-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDRAND instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        let mut last = false;
        for element in self
        {
            last = unsafe { element.fill_random() };
        }
        last
    }
}

impl<T, const N: usize> RdRand for [T; N]
where T: RdRand
{
    /// Fills the 64-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDRAND instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        let mut last = false;
        for element in self
        {
            last = unsafe { element.fill_random() };
        }
        last
    }
}

/// Generates a 16-bit random value and stores it in `e`.
///
/// # Safety
/// Will crash if RDSEED instructions are not supported.
#[inline(always)]
pub unsafe fn rdseed16(e: &mut u16) -> bool
{
    unsafe { _rdseed16_step(e) == 1 }
}

/// Generates a 32-bit random value and stores it in `e`.
///
/// # Safety
/// Will crash if RDSEED instructions are not supported.
#[inline(always)]
pub unsafe fn rdseed32(e: &mut u32) -> bool
{
    unsafe { _rdseed32_step(e) == 1 }
}

/// Generates a 64-bit random value and stores it in `e`.
///
/// # Safety
/// Will crash if RDSEED instructions are not supported.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rdseed64(e: &mut u64) -> bool
{
    unsafe { _rdseed64_step(e) == 1 }
}

/// Generates a 64-bit random value and stores it in `e`.
///
/// This function uses the RDSEED instruction twice to generate two 32-bit
/// halves of the 64-bit value. First result is low half, second is high.
///
/// # Safety
/// Will crash if RDSEED instructions are not supported.
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn rdseed64(e: &mut u64) -> bool
{
    let (mut a, mut b): (u32, u32);
    unsafe { _rdseed32_step(&mut a) == 1 };
    let ret = unsafe { _rdseed32_step(&mut b) == 1 };
    e = a & (b << 32);
    ret
}

/// RdSeed trait to implement the generic rdseed_slice function.
pub trait RdSeed
{
    /// Fills `self` with random bits. Returns true on success or false
    /// otherwise.
    ///
    /// # Safety
    /// RDSEED is not supported on all architectures, so using this may crash
    /// you.
    unsafe fn fill_random(&mut self) -> bool;
}

impl RdSeed for u8
{
    /// Fills the 16-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        let mut r: u16 = 0;
        let ret = unsafe { rdseed16(&mut r) };
        *self = r as u8;
        ret
    }
}

impl RdSeed for u16
{
    /// Fills the 16-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        unsafe { rdseed16(self) }
    }
}

impl RdSeed for u32
{
    /// Fills the 32-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        unsafe { rdseed32(self) }
    }
}

impl RdSeed for u64
{
    /// Fills the 64-bit value with a random bit string.
    ///
    /// This function uses the RDSEED instruction twice to generate two 32-bit
    /// halves of the 64-bit value. First result is low half, second is high.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        unsafe { rdseed64(self) }
    }
}

impl<T> RdSeed for [T]
where T: RdSeed
{
    /// Fills the 64-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        let mut last = false;
        for element in self
        {
            last = unsafe { element.fill_random() };
        }
        last
    }
}

impl<T, const N: usize> RdSeed for [T; N]
where T: RdSeed
{
    /// Fills the 64-bit value with a random bit string.
    ///
    /// # Safety
    /// Will crash if RDSEED instructions are not supported.
    unsafe fn fill_random(&mut self) -> bool
    {
        let mut last = false;
        for element in self
        {
            last = unsafe { element.fill_random() };
        }
        last
    }
}
