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
    const RESULT: str::Ref<12> = unsafe { str::concat::<6, 6>("Hello ", "World!") };

    println!("{}", (&RESULT).deref());
}
