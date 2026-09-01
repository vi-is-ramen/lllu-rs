pub struct Rdtscp {
    pub ticks: u64,
    pub aux: u32,
}

impl From<(u64, u32)> for Rdtscp {
    fn from((ticks, aux): (u64, u32)) -> Self {
        Self { ticks, aux }
    }
}

#[inline]
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
    (((hi as u64) << 32) | (lo as u64), aux).into()
}
