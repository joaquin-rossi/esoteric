use byteorder::{BigEndian, ByteOrder, WriteBytesExt};

pub fn u8_to_u32(b8: &Vec<u8>) -> Vec<u32> {
    let mut b32 = Vec::with_capacity(b8.len() / 4);
    for i in (0..b8.len()).step_by(4) {
        b32.push(BigEndian::read_u32(&b8[i..]))
    }
    b32
}

pub fn u32_to_u8(b32: &Vec<u32>) -> Vec<u8> {
    let mut b8 = Vec::with_capacity(b32.len() * 4);
    for &i in b32 {
        b8.write_u32::<BigEndian>(i).unwrap();
    }
    b8
}
