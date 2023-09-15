mod encoding;
mod header;
mod packet;
mod question;
mod record;
mod resolve;

use std::{
    io::Cursor,
    net::{IpAddr, SocketAddr, UdpSocket},
    str::FromStr,
};

use bytes::Bytes;
use encoding::encode_domain_name;
use header::DNSHeader;
use packet::DNSPacket;
use question::DNSQuestion;
use rand::SeedableRng;
use socket2::{Domain, Type};

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
    let recursion_desired = 0;
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

fn send_query<R: rand::Rng>(
    rng: &mut R,
    ip_address: IpAddr,
    domain_name: &str,
    record_type: u16,
) -> eyre::Result<DNSPacket> {
    let query = build_query(rng, domain_name, record_type);
    let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    let socket: UdpSocket = socket.into();
    let addr: SocketAddr = SocketAddr::new(ip_address, 53);
    socket.send_to(&query, addr)?;
    let mut buf = [0; 1024];
    socket.recv_from(&mut buf)?;
    let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    Ok(DNSPacket::parse_dns_packet(&mut buf))
}

fn ip_to_string(ip: &[u8]) -> String {
    format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
}

fn main() -> eyre::Result<()> {
    let rng = &mut rand::rngs::SmallRng::from_entropy();
    let ip_addr = IpAddr::from_str("198.41.0.4").unwrap();
    let domain_name = "www.google.com";
    let record_type = TYPE_A;
    let response = send_query(rng, ip_addr, domain_name, record_type).unwrap();
    println!("Authorities {:#?}", response.authorities());
    println!("Additionals {:#?}", response.additionals());

    let response = send_query(
        rng,
        "192.12.94.30".parse().unwrap(),
        domain_name,
        record_type,
    )
    .unwrap();
    println!("Authorities {:#?}", response.authorities());
    println!("Additionals {:#?}", response.additionals());
    let response = send_query(
        rng,
        "216.239.32.10".parse().unwrap(),
        domain_name,
        record_type,
    )
    .unwrap();
    println!("Answer {:#?}", response.answers());
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
