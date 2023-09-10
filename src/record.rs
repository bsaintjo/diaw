use std::{io::Seek, net::IpAddr};

use bytes::Buf;

use crate::encoding;

pub const TYPE_A: u16 = 1;
pub const TYPE_NS: u16 = 2;

pub enum DNSRecordResult {
    NameServer(String),
    Address(IpAddr),
}

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
        let data = match qtype {
            TYPE_A => todo!(),
            TYPE_NS => todo!(),
            _ => todo!(),
        };
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
