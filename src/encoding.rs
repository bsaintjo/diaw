use std::io::{Seek, SeekFrom};

use bytes::Buf;

pub fn decode_compressed_name<B>(length: u8, reader: &mut B) -> String
where
    B: Buf + Seek,
{
    let mut pointer_bytes = vec![length & 0b0011_1111];
    pointer_bytes.push(reader.get_u8());
    let pointer = u16::from_be_bytes(pointer_bytes.try_into().unwrap()) as u64;

    let current_pos = reader.stream_position().unwrap();
    reader.seek(SeekFrom::Start(pointer)).unwrap();
    let name = decode_name(reader);
    reader.seek(SeekFrom::Start(current_pos)).unwrap();
    name
}

pub fn decode_name<B>(bytes: &mut B) -> String
where
    B: Buf + Seek,
{
    let mut acc: Vec<String> = Vec::new();
    let mut len = bytes.get_u8();
    while len != 0 {
        // First two bits are 0x11, so name is compressed
        if (len & 0b1100_0000) == 0b1100_0000 {
            let s = decode_compressed_name(len, bytes);
            acc.push(s);
            break;
        } else {
            let s = bytes.copy_to_bytes(len as usize);
            let s = String::from_utf8(s.to_vec()).unwrap();
            acc.push(s);
        }
        len = bytes.get_u8();
    }
    acc.join(".")
}

pub fn decode_name_simple<B>(bytes: &mut B) -> String
where
    B: Buf,
{
    let mut acc: Vec<String> = Vec::new();
    let mut len = bytes.get_u8() as usize;
    while len != 0 {
        let s = bytes.copy_to_bytes(len);
        let s = String::from_utf8(s.to_vec()).unwrap();
        acc.push(s);
        len = bytes.get_u8() as usize;
    }
    acc.join(".")
}

pub fn encode_domain_name(domain_name: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for label in domain_name.split('.') {
        bytes.push(label.len() as u8);
        bytes.extend_from_slice(label.as_bytes());
    }
    bytes.push(0);
    bytes
}

#[cfg(test)]
mod test {

    use bytes::Bytes;

    use super::*;

    #[test]
    fn test_decode_domain() {
        let decoded = decode_name_simple(&mut Bytes::from_static(
            b"\x03www\x07example\x03com\x00\x00\x01",
        ));
        assert_eq!("www.example.com", decoded)
    }

    #[test]
    fn test_encode_dns_name() {
        let xs = encode_domain_name("google.com");
        assert_eq!(xs, b"\x06google\x03com\x00");
    }
}
