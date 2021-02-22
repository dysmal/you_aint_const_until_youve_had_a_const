#![allow(incomplete_features)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]
#![feature(const_fn_transmute)]
#![feature(const_fn_union)]
#![feature(const_generics)]
#![feature(const_ptr_offset)]
#![feature(const_refs_to_cell)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_from_utf8_unchecked)]
#![feature(const_trait_bound_opt_out)]
#![feature(const_trait_impl)]
#![feature(format_args_capture)]

pub mod array;
pub mod mem;

pub use self::array::Array;

#[macro_export]
macro_rules! string {
    ($array:expr) => {{
        const RODATA: &'static str = $array;
        const ARRAY: Array<u8, { RODATA.len() }> = Array::from(RODATA);

        ARRAY
    }};
}
