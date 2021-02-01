use core::mem::ManuallyDrop;

pub const unsafe fn transmute<A, B>(a: A) -> B {
    union Transmute<A, B> {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>,
    }

    ManuallyDrop::into_inner(
        Transmute {
            a: ManuallyDrop::new(a),
        }
        .b,
    )
}
