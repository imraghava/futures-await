#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;

use futures::prelude::*;

#[async]
fn foo() -> u32 {
    3
}

#[async(boxed)]
fn bar() -> u32 {
    3
}

fn main() {}
