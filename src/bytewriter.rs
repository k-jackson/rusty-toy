use byteorder::*;
use std::fs::File;

pub trait ByteWriter {
    fn write(self, output_file: &mut File);
}

impl<'a> ByteWriter for &'a[u8] {
    #[allow(unused_must_use)]
    fn write(self, output_file: &mut File) {
        for &v in self {
            output_file.write_u8(v);
        }
    }
}

impl ByteWriter for u8 {
    #[allow(unused_must_use)]
    fn write(self, output_file: &mut File) {
        output_file.write_u8(self);
    }
}

impl ByteWriter for u16 {
    #[allow(unused_must_use)]
    fn write(self, output_file: &mut File) {
        output_file.write_u16::<BigEndian>(self);
    }
}

impl ByteWriter for u32 {
    #[allow(unused_must_use)]
    fn write(self, output_file: &mut File) {
        output_file.write_u32::<BigEndian>(self);
    }
}

impl ByteWriter for u64 {
    #[allow(unused_must_use)]
    fn write(self, output_file: &mut File) {
        output_file.write_u64::<BigEndian>(self);
    }
}
