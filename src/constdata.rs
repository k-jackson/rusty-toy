use std::collections::HashMap;

pub struct ConstData {
    data: Vec<u8>,
    length: u64,
    indexes: HashMap<String, u64>,
    lengths: HashMap<String, u64>
}
impl ConstData
{
    pub fn new() -> ConstData {
        ConstData {
            data: Vec::new(),
            length: 0,
            indexes: HashMap::new(),
            lengths: HashMap::new()
        }
    }

    pub fn insert(&mut self, constant: &str) -> (u64, u64) {
        let str_bytes = constant.as_bytes();
        let str_length = str_bytes.len() as u64;
        self.data.extend_from_slice(str_bytes);
        self.length += str_length;
        self.indexes.insert(constant.to_string(), self.length);
        self.lengths.insert(constant.to_string(), str_length);

        (self.length, str_length)
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_const_index(&self, constant: &str) -> u64 {
        *self.indexes.get(constant).unwrap()
    }

    pub fn get_const_length(&self, constant: &str) -> u64 {
        *self.lengths.get(constant).unwrap()
    }
}