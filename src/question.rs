use bytes::{Buf, BufMut};

use crate::encoding::decode_name_simple;

pub struct DNSQuestion {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub class: u16,
}

impl DNSQuestion {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + self.name.len());
        bytes.put_slice(&self.name);
        bytes.put_u16(self.qtype);
        bytes.put_u16(self.class);
        bytes
    }

    pub fn parse_question<B>(bytes: &mut B) -> Self
    where
        B: Buf,
    {
        let name = decode_name_simple(bytes);
        let qtype = bytes.get_u16();
        let class = bytes.get_u16();
        DNSQuestion {
            name: name.into_bytes(),
            qtype,
            class,
        }
    }
}
