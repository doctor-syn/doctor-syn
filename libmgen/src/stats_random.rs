use crate::Config;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_runif(num_terms: usize, config: &Config) -> TokenStream {
    quote!(
        /// See https://xorshift.di.unimi.it/splitmix64.c
        /// Returns half-close range 0-1
        pub fn runif(index: usize, min: fty, max: fty) -> fty {
            let z : uty = (index as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15);
            let z1 : uty = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            let z2 : uty = (z1 ^ (z1 >> 27)).wrapping_mul(0x94d049bb133111eb);
            let z3 : uty = z2 ^ (z2 >> 31);
            let x : fty = fty::from_bits((z3 as uty >> 2) | ONE_BITS) - 1.0;
            (x * (max - min)) + min
        }
    )
}
