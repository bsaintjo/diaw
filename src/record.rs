use std::io::Seek;

use bytes::Buf;

use crate::encoding;

pub struct DNSRecord {
    name: String,
    qtype: u16,
    class: u16,
    ttl: u32,
    data: Vec<u8>,
}

impl DNSRecord {
    pub fn parse_record<B>(bytes: &mut B) -> Self
    where
        B: Buf + Seek,
    {
        let name = encoding::decode_name(bytes);
        let qtype = bytes.get_u16();
        let class = bytes.get_u16();
        let ttl = bytes.get_u32();
        let data_len = bytes.get_u16() as usize;
        // let mut data = Vec::with_capacity(data_len);
        let mut data = vec![0; data_len];
        bytes.copy_to_slice(&mut data);
        DNSRecord {
            name,
            qtype,
            class,
            ttl,
            data,
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }
}
