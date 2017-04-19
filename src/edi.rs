extern crate byteorder;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian, ByteOrder};

fn main() {
    let mut a = vec![0;8];// Vec::with_capacity(1);
    BigEndian::write_u64(a.as_mut_slice(), !390299077241733128);
    let q:u64 = 390299077241733128;
    let p:u64=!q;
    println!("{:?}", p);
}
// use std::io::{Read, Write};

// impl<T: Read> NumberDecoder for T {}

// pub trait NumberDecoder: Read {
//     /// `decode_u64_desc` decodes value encoded by `encode_u64_desc` before.
//     fn decode_u64_desc(&mut self) -> Result<u64> {
//         let v = try!(self.read_u64::<BigEndian>());
//         Ok(!v)
//     }
// }