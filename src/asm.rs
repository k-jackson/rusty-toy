use std::fs::File as File;
use asm_macro as Macro;

pub struct Assembler<'a> {
    pub output_file: &'a mut File,
}
impl<'a> Assembler<'a> {

    pub fn exit(&mut self) {
        Macro::exit(self.output_file);
    }
}
