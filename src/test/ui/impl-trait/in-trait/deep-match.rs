#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

struct Wrapper<T>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

impl Foo for () {
    fn bar() -> i32 { 0 }
    //~^ ERROR method `bar` has an incompatible type for trait
}

fn main() {}
