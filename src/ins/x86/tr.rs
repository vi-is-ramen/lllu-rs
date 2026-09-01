use crate::reg::Segment;

/// Returns the current value of the task register.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn str() -> Segment
{
    let segment: u16;
    unsafe {
        core::arch::asm!("str {0:x}",
        out(reg) segment,
        options(nostack, nomem, preserves_flags));
    }
    segment.into()
}

/// Loads the task register.
///
/// # Safety
/// Needs CPL 0.
pub unsafe fn ltr(sel: Segment)
{
    unsafe {
        core::arch::asm!("ltr {0:x}",
        in(reg) u16::from(sel),
        options(nostack, nomem, preserves_flags));
    }
}
