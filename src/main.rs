extern crate byteorder;

mod scanner;
mod parser;
mod tree;
mod generator;
mod elfwriter;
mod bytewriter;
mod asm;
mod asm_macro;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use byteorder::{LittleEndian, WriteBytesExt};

fn main() {
    let program = read();
    let tokens = scanner::scan(program);
    for t in &tokens {
        println!("{:?}", t);
    }
    println!("parsing...");

    let ast = parser::parse(&tokens);
    match ast {
        Some(tree) => generator::generate(tree, "out.bin"),
        None       => panic!("nop")
    }
}

fn read() -> String {

    let path = Path::new("test.txt");
    let display = path.display();

    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    }

    return s;
}
