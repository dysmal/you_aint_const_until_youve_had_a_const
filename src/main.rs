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

fn main() {
    const HELLO: &'static str = "Hello ";
    const WORLD: &'static str = "World";
    const HELLO_WORLD: str::Ref<{ HELLO.len() + WORLD.len() }> =
        str::concat::<{ HELLO.len() }, { WORLD.len() }>(HELLO, WORLD);

    const RESULT: str::Ref<{ HELLO_WORLD.len() + 1 }> =
        str::concat::<{ HELLO_WORLD.len() }, 1>(&HELLO_WORLD, "!");

    const FOO: str::Ref<3> = str::Ref::new("Foo");
    const BAR: str::Ref<3> = str::Ref::new("Bar");
    const FOOBAR: str::Ref<{ FOO.len() + BAR.len() }> = FOO + BAR;

    println!("{RESULT}", RESULT = &RESULT as &str);
    println!("{FOOBAR}", FOOBAR = &FOOBAR as &str);
}
