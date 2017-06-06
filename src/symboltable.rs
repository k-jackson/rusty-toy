use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum VariableType {
    Integer
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SymbolTable {
    data: Vec<u8>,
    length: u64,
    indexes: HashMap<String, u64>,
    lengths: HashMap<String, u64>,
    types: HashMap<String, VariableType>,
    symbol_section_address: u64
}
impl SymbolTable {
    #[allow(dead_code)]
    pub fn new() -> SymbolTable {
        SymbolTable {
            data: Vec::new(),
            length: 0,
            indexes: HashMap::new(),
            lengths: HashMap::new(),
            types: HashMap::new(),
            symbol_section_address: 0x0
        }
    }

    #[allow(dead_code)]
    pub fn set_symbol_section_address(&mut self, address: u64) {
        self.symbol_section_address = address;
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, var_name: &str, var_type: VariableType) {
        let var_bytes = var_name.as_bytes();
        let var_length = var_bytes.len() as u64;
        let start_index = self.get_length();
        self.data.extend_from_slice(var_bytes);
        self.length += var_length;
        self.indexes.insert(var_name.to_string(), start_index);
        self.lengths.insert(var_name.to_string(), var_length);
        self.types.insert(var_name.to_string(), var_type);
    }

    #[allow(dead_code)]
    pub fn get_length(&self) -> u64 {
        self.length
    }

    #[allow(dead_code)]
    pub fn get_var_index(&self, var: &str) -> u64 {
        *self.indexes.get(var).unwrap()
    }

    #[allow(dead_code)]
    pub fn get_var_length(&self, var: &str) -> u64 {
        *self.lengths.get(var).unwrap()
    }

    #[allow(dead_code)]
    pub fn get_var_type(&self, var: &str) -> &VariableType {
        self.types.get(var).unwrap()
    }
}