pub trait Table
where Self: Default
{
    type Entry;
    fn len(&self) -> usize;
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TablePtr<T>
where T: Table
{
    limit: u16,
    base:  *const T,
}

impl<T> Default for TablePtr<T>
where T: Table
{
    fn default() -> Self
    {
        Self {
            limit: 0,
            base:  core::ptr::null(),
        }
    }
}

impl<T> TablePtr<T>
where T: Table
{
    pub fn from_table(table: &T) -> Self
    {
        Self {
            limit: (table.len() * core::mem::size_of::<T::Entry>()) as u16,
            base:  table as *const T,
        }
    }

    pub fn limit(&self) -> u16
    {
        self.limit
    }

    pub fn base(&self) -> *const T
    {
        self.base
    }

    pub unsafe fn as_ref(&self) -> Option<&T>
    {
        unsafe { self.base.as_ref() }
    }

    pub unsafe fn as_ref_unchecked(&self) -> &T
    {
        unsafe { self.base.as_ref_unchecked() }
    }

    pub unsafe fn as_mut(&mut self) -> Option<&mut T>
    {
        unsafe { (self.base as *mut T).as_mut() }
    }

    pub unsafe fn as_mut_unchecked(&mut self) -> &mut T
    {
        unsafe { (self.base as *mut T).as_mut_unchecked() }
    }
}

ins_mod!(irq as pub);
