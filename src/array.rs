use crate::mem::{concat, different_sized_transmute};
use core::mem::MaybeUninit;

pub struct Array<T, const N: usize>([MaybeUninit<T>; N])
where
    T: ?const Copy;

impl<T, const N: usize> Array<T, N>
where
    T: ?const Copy,
{
    /// create a new uninitialized array
    pub const fn new() -> Array<T, N> {
        Array([MaybeUninit::<T>::uninit(); N])
    }

    /// concat this array with another
    pub const fn concat<const M: usize>(&self, other: Array<T, M>) -> Array<T, { N + M }> {
        Array(unsafe { concat(self.0, other.0) })
    }

    /// obtains a pointer to the begining of the array
    pub const fn as_ptr(&self) -> *const T {
        self.0.as_ptr() as *const T
    }

    /// obtains the length
    pub const fn len(&self) -> usize {
        N
    }
}

impl<const N: usize> Array<u8, N> {
    /// create a new array from a str
    pub const fn from_str(string: &'static str) -> Array<u8, N> {
        Array(unsafe {
            *different_sized_transmute::<_, *const [MaybeUninit<u8>; N]>(string.as_bytes())
        })
    }

    /// create a str from an array
    pub const fn as_str<'a>(&self) -> &'a str {
        #[repr(C)]
        struct Ref(*const u8, usize);

        unsafe { core::mem::transmute(Ref(self.as_ptr(), N)) }
    }
}
