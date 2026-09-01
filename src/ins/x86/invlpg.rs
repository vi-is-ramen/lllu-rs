pub unsafe fn invlpg(addr: usize)
{
    unsafe {
        core::arch::asm! {
            "invlpg [{}]",
            in(reg) addr,
            options(nostack, preserves_flags)
        }
    }
}
