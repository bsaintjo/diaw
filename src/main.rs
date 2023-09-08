mod encoding;
mod header;
mod packet;
mod question;
mod record;

use std::{
    io::Cursor,
    net::{SocketAddr, UdpSocket},
};

use bytes::Bytes;
use encoding::encode_domain_name;
use header::DNSHeader;
use packet::DNSPacket;
use question::DNSQuestion;
use socket2::{Domain, Type};

use crate::record::DNSRecord;

const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

struct QueryBuilder<R> {
    rng: R,
    domain_name: String,
    record_type: u16,
}

impl<R: rand::Rng> QueryBuilder<R> {
    fn new(rng: R, domain_name: String, record_type: u16) -> Self {
        Self {
            rng,
            domain_name,
            record_type,
        }
    }

    fn with_rng(&mut self, rng: R) -> &mut Self {
        self.rng = rng;
        self
    }

    fn build_query(&mut self) -> Vec<u8> {
        let name = encode_domain_name(&self.domain_name);
        let id = self.rng.gen::<u16>();
        let recursion_desired = 1 << 8;
        let mut header = DNSHeader::new(id, recursion_desired);
        header.num_questions = 1;
        let question = DNSQuestion {
            name,
            qtype: self.record_type,
            class: CLASS_IN,
        };
        let mut acc = Vec::new();
        acc.extend(header.to_be_bytes());
        acc.extend(question.to_be_bytes());
        acc
    }
}

fn build_query<R: rand::Rng>(rng: &mut R, domain_name: &str, record_type: u16) -> Vec<u8> {
    let name = encode_domain_name(domain_name);
    let id = rng.gen::<u16>();
    let recursion_desired = 1 << 8;
    let mut header = DNSHeader::new(id, recursion_desired);
    header.num_questions = 1;
    let question = DNSQuestion {
        name,
        qtype: record_type,
        class: CLASS_IN,
    };
    let mut acc = Vec::new();
    acc.extend(header.to_be_bytes());
    acc.extend(question.to_be_bytes());
    acc
}

fn ip_to_string(ip: &[u8]) -> String {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}

fn lookup_doman(domain: &str) -> eyre::Result<String> {
    let query = build_query(&mut rand::thread_rng(), domain, TYPE_A);
    let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    let socket: UdpSocket = socket.into();
    let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
    socket.send_to(&query, addr)?;

    let mut buf = [0; 1024];
    let _ = socket.recv_from(&mut buf)?;
    let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    let response = DNSPacket::parse_dns_packet(&mut buf);
    let ip = response.answers()[0].data();

    Ok(ip_to_string(ip))
}

fn main() -> eyre::Result<()> {
    println!("Domain: \"example.com\", IP: {}", lookup_doman("www.example.com")?);
    println!("Domain: \"recurse.com\", IP: {}", lookup_doman("recurse.com")?);
    println!("Domain: \"metafilter.com\", IP: {}", lookup_doman("metafilter.com")?);

    println!("Domain: \"www.facebook.com\", IP: {}", lookup_doman("www.facebook.com")?);
    // println!("Domain: \"www.metafilter.com\", IP: {}", lookup_doman("www.metafilter.com")?);
    // let mut rng = rand::thread_rng();
    // let query = build_query(&mut rng, "www.example.com", TYPE_A);
    // let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    // let socket: UdpSocket = socket.into();
    // let addr: SocketAddr = "8.8.8.8:53".parse().unwrap();
    // socket.send_to(&query, addr)?;

    // let mut buf = [0; 1024];
    // let _ = socket.recv_from(&mut buf)?;
    // let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    // let header = DNSHeader::parse_header(&mut buf);
    // let question = DNSQuestion::parse_question(&mut buf);
    // let record = DNSRecord::parse_record(&mut buf);

    // println!("{:#?}", header);

    Ok(())
}

#[cfg(test)]
mod test {

    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn test_encode_dns_name() {
        let xs = encode_domain_name("google.com");
        assert_eq!(xs, b"\x06google\x03com\x00");
    }

    #[test]
    fn test_build_query() {
        let mut mock_rng = StepRng::new(0x8298, 0);
        let res = build_query(&mut mock_rng, "www.example.com", TYPE_A);
        assert_eq!(
            res,
            [
                0x82, 0x98, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77,
                0x77, 0x77, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d,
                0x00, 0x00, 0x01, 0x00, 0x01
            ]
        )
    }
}
