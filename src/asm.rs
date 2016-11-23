use std::fs::File as File;
use asm_macro as Macro;

pub struct Assembler {
    pub output: Vec<u8>,
    pub length: u64
}
impl Assembler {

    pub fn exit(&mut self) {
        self.length += Macro::exit(&mut self.output);
    }

    pub fn get_length(&self) -> u64 {
        self.length
    }

    pub fn get_output(&self) -> &Vec<u8> {
        &self.output
    }
}
