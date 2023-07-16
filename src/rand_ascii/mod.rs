use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashSet;
use std::iter::{IntoIterator, Iterator};
extern crate lazy_static;

lazy_static! {
    static ref FORBIDDEN: HashSet<u8> = vec![127, 129, 141, 143, 144, 157, 160, 182]
        .into_iter()
        .collect();
}

pub fn get_random_ascii_printable_code() -> char {
    let mut num: u8 = 127;
    let mut rng = rand::thread_rng();
    while FORBIDDEN.contains(&num) {
        num = rng.gen_range(33..=255);
    }
    num as char
}
