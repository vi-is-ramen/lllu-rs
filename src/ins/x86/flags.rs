#[inline(always)]
pub fn cli() {
    unsafe {
        core::arch::asm! { "cli", options(nomem, nostack) }
    }
}

#[inline(always)]
pub fn sti() {
    unsafe {
        core::arch::asm! { "sti", options(nomem, nostack) }
    }
}

#[inline(always)]
pub fn clc() {
    unsafe {
        core::arch::asm! { "clc", options(nomem, nostack) }
    }
}

#[inline(always)]
pub fn stc() {
    unsafe {
        core::arch::asm! { "stc", options(nomem, nostack) }
    }
}

#[inline(always)]
pub fn cld() {
    unsafe {
        core::arch::asm! { "cld", options(nomem, nostack) }
    }
}

#[inline(always)]
pub fn std() {
    unsafe {
        core::arch::asm! { "std", options(nomem, nostack) }
    }
}

#[inline(always)]
pub fn ckc() -> bool {
    unsafe {
        core::arch::asm! {
            "jc {}",
            label { return true },
            options(nomem, nostack),
        }
    }

    false
}

#[inline(always)]
pub fn ckz() -> bool {
    unsafe {
        core::arch::asm! {
            "jz {}",
            label { return true },
            options(nomem, nostack),
        }
    }

    false
}

#[inline(always)]
pub fn cks() -> bool {
    unsafe {
        core::arch::asm! {
            "js {}",
            label { return true },
            options(nomem, nostack),
        }
    }

    false
}
