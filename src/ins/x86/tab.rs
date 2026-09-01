#[inline(always)]
pub unsafe fn lgdt<T>(gdt: &crate::tab::TablePtr<T>)
where
    T: crate::tab::Table,
{
    unsafe {
        core::arch::asm!("lgdt [{0}]", in(reg) gdt);
    }
}

#[inline(always)]
pub unsafe fn sgdt<T>() -> crate::tab::TablePtr<T>
where
    T: crate::tab::Table,
{
    let ret: crate::tab::TablePtr<T> = crate::tab::TablePtr::default();
    unsafe {
        core::arch::asm!("sgdt [{0}]", in(reg) &raw const ret);
    };
    ret
}

#[inline(always)]
pub unsafe fn lidt<T>(gdt: &crate::tab::TablePtr<T>)
where
    T: crate::tab::Table,
{
    unsafe {
        core::arch::asm!("lidt [{0}]", in(reg) gdt);
    }
}

#[inline(always)]
pub unsafe fn sidt<T>() -> crate::tab::TablePtr<T>
where
    T: crate::tab::Table,
{
    let ret: crate::tab::TablePtr<T> = crate::tab::TablePtr::default();
    unsafe {
        core::arch::asm!("sidt [{0}]", in(reg) &raw const ret);
    };
    ret
}
