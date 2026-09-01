#[inline(always)]
pub unsafe fn rdpid() -> u32
{
    let aux: u32;
    unsafe {
        core::arch::asm! {
            "rdpid {0:e}",
            out(reg) aux,
            options(nomem, nostack, preserves_flags),
        }
    }
    aux
}
