#[inline(always)]
pub unsafe fn hlt() -> ! {
    unsafe {
        core::arch::asm!(
            "hlt",
            "ud2",
            options(noreturn, nostack, nomem, preserves_flags)
        )
    }
}
