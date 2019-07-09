use core::ops::{Deref, DerefMut};

#[repr(C)]
pub struct ManuallyDrop<T: ?Sized> {
    value: T,
}

impl<T> ManuallyDrop<T> {
    pub fn new(value: T) -> ManuallyDrop<T> {
        ManuallyDrop { value : value }
    }
    pub fn into_inner(slot: ManuallyDrop<T>) -> T {
        slot.value
    }
}

impl<T: ?Sized> Deref for ManuallyDrop<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: ?Sized> DerefMut for ManuallyDrop<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
