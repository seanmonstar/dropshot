// Copyright 2024 Oxide Computer Company

#![allow(unused_imports)]

// Test for a trait with a macro invocation inside that has an endpoint
// annotation. (Endpoint annotations can only live on functions.)

macro_rules! life_the_universe_and_everything {
    () => {
        fn the_answer() -> u32;
    };
}

macro_rules! mostly_harmless {
    () => {
        fn six_times_nine() -> u32;
    };
}

#[dropshot::api_description]
trait MyApi {
    type Context;

    #[endpoint { method = GET }]
    life_the_universe_and_everything!();

    #[channel { protocol = WEBSOCKETS }]
    mostly_harmless!();
}

enum MyImpl {}

// This should not produce errors about the trait or any of the items within
// being missing.
impl MyApi for MyImpl {
    type Context = ();

    fn the_answer() -> u32 {
        42
    }

    fn six_times_nine() -> u32 {
        42
    }
}

fn main() {
    // These items should be generated and accessible.
    my_api_mod::api_description::<MyImpl>();
    my_api_mod::stub_api_description();
}
