use super::{mem, slice};
use core::ops::{Add, Deref};

pub struct Ref<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> Ref<N> {
    pub const fn new(inner: &'static str) -> Ref<N> {
        Ref {
            inner: unsafe { *mem::transmute::<_, *const [u8; N]>(inner.as_bytes()) },
        }
    }

    pub const fn as_str(&'static self) -> &'static str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

// rustfmt doesn't know what impl const is
#[rustfmt::skip]
impl<const N: usize> const Deref for Ref<N> {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

// rustfmt doesn't know what impl const is
#[rustfmt::skip]
impl<const A: usize, const B: usize> const Add<Ref<B>> for Ref<A>
where
    [u8; A]: Sized,
    [u8; B]: Sized,
    [u8; A + B]: Sized,
{
    type Output = Ref<{ A + B }>;

    fn add(self, other: Ref<B>) -> Ref<{ A + B }> {
        #[derive(Clone, Copy)]
        struct Concat<A, B>(A, B);

        unsafe { mem::transmute(
            Concat::<[u8; A], [u8; B]>(
                *mem::transmute::<_, *const [u8; A]>(self.as_ptr()),
                *mem::transmute::<_, *const [u8; B]>(other.as_ptr()),
            )
        ) }
    }
}

pub const fn concat<const A: usize, const B: usize>(
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
