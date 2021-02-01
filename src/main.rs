#![allow(incomplete_features)]
#![feature(const_evaluatable_checked)]
#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(const_generics)]
#![feature(const_ptr_offset)]
#![feature(const_raw_ptr_deref)]
#![feature(const_str_from_utf8_unchecked)]
#![feature(const_trait_impl)]
#![feature(format_args_capture)]

pub mod mem;
pub mod slice;
pub mod str;

use crate::str::Ref;

fn main() {
    const HELLO: Ref<6> = Ref::new("Hello ");
    const WORLD: Ref<5> = Ref::new("World");
    const HELLO_WORLD: Ref<{ HELLO.len() + WORLD.len() + 1 }> = HELLO + WORLD + Ref::<1>::new("!");

    const FOO: Ref<3> = Ref::new("Foo");
    const BAR: Ref<3> = Ref::new("Bar");
    const FOOBAR: Ref<{ FOO.len() + BAR.len() }> = FOO + BAR;

    println!("{}", &HELLO_WORLD as &str);
    println!("{}", &FOOBAR as &str);
}
