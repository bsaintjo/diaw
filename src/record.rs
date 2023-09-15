use std::{io::Seek, net::IpAddr};

use bytes::Buf;

use crate::encoding;

pub const TYPE_A: u16 = 1;
pub const TYPE_NS: u16 = 2;

#[derive(Debug, Clone)]
pub enum DNSRecordResult {
    NameServer(String),
    Address(IpAddr),
    Unknown(Vec<u8>),
}

#[derive(Debug)]
pub struct DNSRecord {
    name: String,
    qtype: u16,
    class: u16,
    ttl: u32,
    res: DNSRecordResult,
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

        // Note: data_len isn't used for NameServer records, but value will be 4
        let data_len = bytes.get_u16() as usize;

        let res = match qtype {
            TYPE_A => {
                let mut data = vec![0; data_len];
                bytes.copy_to_slice(&mut data);
                let raw_ip: [u8; 4] = data.as_slice().try_into().unwrap();
                let ip = IpAddr::from(raw_ip);
                DNSRecordResult::Address(ip)
            }
            TYPE_NS => {
                let name = encoding::decode_name(bytes);
                DNSRecordResult::NameServer(name)
            }
            _ => {
                let mut data = vec![0; data_len];
                bytes.copy_to_slice(&mut data);
                DNSRecordResult::Unknown(data)
            }
        };
        DNSRecord {
            name,
            qtype,
            class,
            ttl,
            res,
        }
    }

    pub fn qtype(&self) -> u16 {
        self.qtype
    }

    pub fn res(&self) -> &DNSRecordResult {
        &self.res
    }
}
