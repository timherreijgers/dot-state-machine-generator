use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("test.out").expect("");
    file.write_all(b"Hello, world!").expect("");
}