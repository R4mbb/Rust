use rand::Rng;
// use std::collections::HashMap;
use std::collections::*;

// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
}
