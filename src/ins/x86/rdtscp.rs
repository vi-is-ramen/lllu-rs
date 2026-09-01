pub struct Rdtscp {
    pub ticks: u64,
    pub aux: u32,
}

#[inline(always)]
pub unsafe fn rdtscp() -> Rdtscp {
    let lo: u32;
    let hi: u32;
    let aux: u32;

    unsafe {
        core::arch::asm! {
            "rdtscp",
            out("eax") lo,
            out("edx") hi,
            out("ecx") aux,
            options(nomem, nostack, preserves_flags),
        }
    }

    Rdtscp {
        ticks: ((hi as u64) << 32) | (lo as u64),
        aux,
    }
}
