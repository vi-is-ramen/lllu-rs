use crate::reg::Register;

/// Drops the TLB by writing the current CR3 value back to CR3.
///
/// # Safety
///
/// This function requires CPL 0.
pub unsafe fn drop_tlb()
{
    // SAFETY:
    // This code executes in CPL 0 and doesn't modify CR3 - only touch to
    // invalidate whole TLB.
    unsafe {
        crate::reg::Cr3.write(crate::reg::Cr3.read());
    }
}
