#![allow(incomplete_features)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(const_generics)]
#![feature(const_ptr_offset)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_from_utf8_unchecked)]
#![feature(format_args_capture)]

use core::ops::Deref;

pub mod mem;
pub mod slice;
pub mod str;

fn main() {
    const HELLO: &'static str = "Hello ";
    const WORLD: &'static str = "World!";
    const RESULT: str::Ref<{ HELLO.len() + WORLD.len() }> =
        unsafe { str::concat::<{ HELLO.len() }, { WORLD.len() }>(HELLO, WORLD) };

    println!("{}", (&RESULT).deref());
}
