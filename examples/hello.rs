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

use yacuyhac::Array;

fn main() {
    let hello = Array::<_, 7>::from_str("Hello, ");
    let world = Array::<_, 6>::from_str("World!");
    let greet = hello.concat(world).as_str();

    println!("{greet}");

    let john = Array::<_, 6>::from_str(" John!");
    let greeting = Array::<_, 14>::from_str(greet).concat(john).as_str();

    println!("{greeting}");
}
