#[inline(always)]
pub unsafe fn send<T>(port: u16, value: T)
where
    T: Copy + Sized,
    usize: From<T>,
{
    unsafe {
        if core::mem::size_of::<T>() == 1 {
            core::arch::asm! {
                "out dx, al",
                in("al") usize::from(value) as u8,
                in("dx") port,
            }
        } else if core::mem::size_of::<T>() == 2 {
            core::arch::asm! {
                "out dx, ax",
                in("ax") usize::from(value) as u16,
                in("dx") port,
            }
        } else if core::mem::size_of::<T>() == 4 {
            core::arch::asm! {
                "out dx, eax",
                in("eax") usize::from(value) as u32,
                in("dx") port,
            }
        }
    }
}

#[inline(always)]
pub unsafe fn recv<T>(port: u16) -> T
where
    T: Copy + Sized,
    T: From<usize>,
{
    let mut value = core::mem::MaybeUninit::<T>::uninit();
    unsafe {
        if core::mem::size_of::<T>() == 1 {
            let mut reg_al: u8;
            core::arch::asm! {
                "in al, dx",
                out("al") reg_al,
                in("dx") port,
            }
            core::ptr::write(value.as_mut_ptr() as *mut u8, reg_al);
        } else if core::mem::size_of::<T>() == 2 {
            let mut reg_ax: u16;
            core::arch::asm! {
                "in ax, dx",
                out("ax") reg_ax,
                in("dx") port,
            }
            core::ptr::write(value.as_mut_ptr() as *mut u16, reg_ax);
        } else if core::mem::size_of::<T>() == 4 {
            let mut reg_eax: u32;
            core::arch::asm! {
                "in eax, dx",
                out("eax") reg_eax,
                in("dx") port,
            }
            core::ptr::write(value.as_mut_ptr() as *mut u32, reg_eax);
        }
        value.assume_init()
    }
}
