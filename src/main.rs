#![allow(incomplete_features)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(const_generics)]
#![feature(const_ptr_offset)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_from_utf8_unchecked)]
#![feature(format_args_capture)]

pub mod mem {
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
}

pub mod slice {
    pub use super::mem;

    pub const unsafe fn concat<A, B, C>(a: &'static [u8], b: &'static [u8]) -> C
    where
        A: Copy,
        B: Copy,
        C: Copy,
    {
        #[repr(C)]
        #[derive(Copy, Clone)]
        struct Concat<A, B>(A, B);

        let bytes: Concat<A, B> = Concat(
            *mem::transmute::<_, *const A>(a.as_ptr()),
            *mem::transmute::<_, *const B>(b.as_ptr()),
        );

        mem::transmute(bytes)
    }
}

pub mod str {
    use super::slice;

    pub const unsafe fn concat<const A: usize, const B: usize>(
        a: &'static str,
        b: &'static str,
    ) -> [u8; A + B]
    where
        [u8; A]: Sized,
        [u8; B]: Sized,
        [u8; A + B]: Sized,
    {
        slice::concat::<[u8; A], [u8; B], [u8; A + B]>(a.as_bytes(), b.as_bytes())
    }
}

fn main() {
    const RESULT: &'static str =
        unsafe { std::str::from_utf8_unchecked(&str::concat::<6, 6>("Hello ", "World!")) };

    println!("{RESULT}");
}
