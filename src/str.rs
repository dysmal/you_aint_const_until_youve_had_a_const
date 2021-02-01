use super::slice;
use core::ops::Deref;

pub struct Ref<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> Deref for Ref<N> {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

pub const unsafe fn concat<const A: usize, const B: usize>(
    a: &'static str,
    b: &'static str,
) -> Ref<{ A + B }>
where
    [u8; A]: Sized,
    [u8; B]: Sized,
    [u8; A + B]: Sized,
{
    Ref {
        inner: slice::concat::<[u8; A], [u8; B], [u8; A + B]>(a.as_bytes(), b.as_bytes()),
    }
}
