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

const fn greet() -> Array<u8, 13> {
    let hello = yacuyhac::string!("Hello, ");
    let world = yacuyhac::string!("World!");

    hello.concat(world)
}

fn main() {
    let greet = greet().as_str();

    println!("{greet}");

    let john = Array::<_, 6>::from_str(" John!");
    let greeting = Array::<_, 14>::from_str(greet).concat(john).as_str();

    println!("{greeting}");
}
