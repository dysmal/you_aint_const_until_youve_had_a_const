pub use super::mem;

pub const fn concat<A, B, C>(a: &'static [u8], b: &'static [u8]) -> C
where
    A: Copy,
    B: Copy,
    C: Copy,
{
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Concat<A, B>(A, B);

    let bytes: Concat<A, B> = unsafe {
        Concat(
            *mem::transmute::<_, *const A>(a.as_ptr()),
            *mem::transmute::<_, *const B>(b.as_ptr()),
        )
    };

    unsafe { mem::transmute(bytes) }
}
