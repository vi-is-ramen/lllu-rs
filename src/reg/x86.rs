macro_rules! impl_cast_int {
    ($int:ty) => {
        impl Cast<$int> for [u8; const { size_of::<$int>() }] {
            fn cast(self) -> $int {
                <$int>::from_le_bytes(self)
            }
        }

        impl Cast<[u8; const { size_of::<$int>() }]> for $int {
            fn cast(self) -> [u8; const { size_of::<$int>() }] {
                self.to_le_bytes()
            }
        }
    };
}

pub trait Cast<U> {
    fn cast(self) -> U;
}

impl<T> Cast<T> for T {
    // for cases where Self is already the target type
    fn cast(self) -> T {
        self
    }
}

impl_cast_int!(u8);
impl_cast_int!(u16);
impl_cast_int!(u32);
impl_cast_int!(u64);
impl_cast_int!(u128);
impl_cast_int!(usize);

pub trait Register<const SIZEOF: usize>
where
    Self::Inner: Sized + Cast<[u8; SIZEOF]>,
{
    type Inner;

    unsafe fn try_read(&self) -> Option<Self::Inner>;

    unsafe fn try_write(&mut self, value: Self::Inner) -> Option<()>;

    unsafe fn try_read_raw(&self) -> Option<[u8; SIZEOF]>;

    unsafe fn try_write_raw(&mut self, value: [u8; SIZEOF]) -> Option<()>;

    unsafe fn read(&self) -> Self::Inner;

    unsafe fn write(&mut self, value: Self::Inner);

    unsafe fn read_raw(&self) -> [u8; SIZEOF];

    unsafe fn write_raw(&mut self, value: [u8; SIZEOF]);
}

ins_mod!(msr as pub);
ins_mod!(cr_dr as pub);
ins_mod!(flags as pub);
ins_mod!(xcr0 as pub);
ins_mod!(seg as pub);
