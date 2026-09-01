#[inline(always)]
pub unsafe fn lgdt<T>(gdt: &crate::tab::TablePtr<T>)
where T: crate::tab::Table
{
    unsafe {
        core::arch::asm!("lgdt [{0}]", in(reg) gdt);
    }
}

#[inline(always)]
pub unsafe fn sgdt<T>() -> crate::tab::TablePtr<T>
where T: crate::tab::Table
{
    let ret: crate::tab::TablePtr<T> = crate::tab::TablePtr::default();
    unsafe {
        core::arch::asm!("sgdt [{0}]", in(reg) &raw const ret);
    };
    ret
}

#[inline(always)]
pub unsafe fn lidt<T>(gdt: &crate::tab::TablePtr<T>)
where T: crate::tab::Table
{
    unsafe {
        core::arch::asm!("lidt [{0}]", in(reg) gdt);
    }
}

#[inline(always)]
pub unsafe fn sidt<T>() -> crate::tab::TablePtr<T>
where T: crate::tab::Table
{
    let ret: crate::tab::TablePtr<T> = crate::tab::TablePtr::default();
    unsafe {
        core::arch::asm!("sidt [{0}]", in(reg) &raw const ret);
    };
    ret
}

#[inline(always)]
pub unsafe fn lldt(selector: crate::reg::Segment)
{
    unsafe {
        core::arch::asm!("lldt {0:x}", in(reg) u16::from(selector));
    }
}

#[inline(always)]
pub unsafe fn sldt() -> crate::reg::Segment
{
    let selector: u16;
    unsafe {
        core::arch::asm!("sldt {0:x}", out(reg) selector);
    }
    crate::reg::Segment::from(selector)
}
