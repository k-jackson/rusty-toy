use asm_macro as Macro;
use constdata::ConstData as ConstData;

pub struct Assembler<'a> {
    pub output: Vec<u8>,
    pub length: u64,
    pub const_data: &'a ConstData
}
impl<'a> Assembler<'a> {

    pub fn exit(&mut self) {
        self.length += Macro::exit(&mut self.output);
    }

    pub fn print_str(&mut self, str_offset: u64, str_len: u64) {
        self.length += Macro::print_str(&mut self.output, str_offset, str_len);
    }

    pub fn builtin_function(&mut self, func_name: &str, param: &str) {
        match func_name {
            "print" => {
                let index = self.const_data.get_const_address(param);
                let length = self.const_data.get_const_length(param);
                self.print_str(index, length);
            },
            _ => {}
        }
    }

    // todo :)
    #[allow(unused_variables)]
    pub fn assignment_statement(&mut self, variable: &str, value: u64) {
        let offset: u64 = self.const_data.get_end_of_section();
        self.length += Macro::store_int(&mut self.output, offset, value);
    }

    pub fn get_length(&self) -> u64 {
        self.length
    }

    pub fn get_output(&self) -> &Vec<u8> {
        &self.output
    }
}
