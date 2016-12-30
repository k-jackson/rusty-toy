#![feature(box_syntax, box_patterns)]

extern crate byteorder;

mod scanner;
mod parser;
mod tree;
mod generator;
mod constdata;
mod elfwriter;
mod bytewriter;
mod asm;
mod asm_macro;
mod asm_opcode;
mod symboltable;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use constdata::ConstData as ConstData;
use parser::Parser as Parser;

fn main() {
    let program = read();
    let tokens = scanner::scan(program);
    for t in &tokens {
        println!("{:?}", t);
    }

    let mut p = Parser::new(&tokens);
    let ast = p.start();
    let const_data = p.get_const_data();

    println!("{:#?}", ast);
    if ast.is_some() {
        match ast {
            Some(tree) => generator::generate(tree, const_data, "out.bin"),
            None => panic!("No valid AST generated")
        }
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
