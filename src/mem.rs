use core::mem;
use core::mem::{ManuallyDrop, MaybeUninit};

pub const unsafe fn different_sized_transmute<A, B>(a: A) -> B
where
    A: ?const Copy,
    B: ?const Copy,
{
    #[derive(Clone, Copy)]
    union DifferentSizedTransmute<A, B> {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>,
    }

    ManuallyDrop::into_inner(
        DifferentSizedTransmute {
            a: ManuallyDrop::new(a),
        }
        .b,
    )
}

pub const unsafe fn concat<T, const N: usize, const M: usize>(
    src1: [MaybeUninit<T>; N],
    src2: [MaybeUninit<T>; M],
) -> [MaybeUninit<T>; N + M]
where
    T: ?const Copy,
{
    #[derive(Clone, Copy)]
    struct Concat<T, const N: usize, const M: usize>([MaybeUninit<T>; N], [MaybeUninit<T>; M])
    where
        T: ?const Copy;

    different_sized_transmute(Concat::<T, { N }, { M }>(
        *mem::transmute::<*const MaybeUninit<T>, *const [MaybeUninit<T>; N]>(src1.as_ptr()),
        *mem::transmute::<*const MaybeUninit<T>, *const [MaybeUninit<T>; M]>(src2.as_ptr()),
    ))
}
