#![feature(test)]

pub use modexp::ModExp;

extern crate num;
extern crate test;

pub mod modexp;

#[cfg(test)]
mod tests;

#[cfg(all(test))]
mod bench;