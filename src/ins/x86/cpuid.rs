#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CpuidResult
{
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

#[inline(always)]
pub fn cpuid_count(leaf: u32, sub_leaf: u32) -> CpuidResult
{
    if cfg!(target_env = "sgx")
    {
        panic!("`cpuid` cannot be used in SGX");
    }

    let eax;
    let ebx;
    let ecx;
    let edx;

    // LLVM sometimes reserves `ebx` for its internal use, we so we need to use
    // a scratch register for it instead.
    #[cfg(target_arch = "x86")]
    unsafe {
        core::arch::asm!(
            "mov {0}, ebx",
            "cpuid",
            "xchg {0}, ebx",
            out(reg) ebx,
            inout("eax") leaf => eax,
            inout("ecx") sub_leaf => ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "mov {0:r}, rbx",
            "cpuid",
            "xchg {0:r}, rbx",
            out(reg) ebx,
            inout("eax") leaf => eax,
            inout("ecx") sub_leaf => ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );
    }
    CpuidResult { eax, ebx, ecx, edx }
}

/// Calls CPUID with the provided `leaf` value, with `sub_leaf` set to 0.
/// See [`cpuid_count`].
#[inline(always)]
pub fn cpuid(leaf: u32) -> CpuidResult
{
    cpuid_count(leaf, 0)
}
