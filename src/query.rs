use std::{
    io::Cursor,
    net::{IpAddr, SocketAddr, UdpSocket}, sync::Arc,
};

use bytes::Bytes;
use socket2::{Domain, Type};

use crate::CLASS_IN;

use crate::{
    encoding::encode_domain_name, header::DNSHeader, packet::DNSPacket, question::DNSQuestion,
};

pub fn build_query<R: rand::Rng>(rng: &mut R, domain_name: &str, record_type: u16) -> Vec<u8> {
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

pub async fn send_query_async<R: rand::Rng>(
    rng: &mut R,
    socket: &tokio::net::UdpSocket,
    ip_address: IpAddr,
    domain_name: &str,
    record_type: u16,
) -> eyre::Result<DNSPacket> {
    tracing::debug!("Sending query to {}", ip_address);
    let query = build_query(rng, domain_name, record_type);
    let addr: SocketAddr = SocketAddr::new(ip_address, 53);
    // socket.connect(addr).await?;
    // tracing::debug!("Connected to {}", addr);
    socket.send_to(&query, addr).await?;
    tracing::debug!("Sent query to {}", addr);
    let mut buf = [0; 1024];
    let n = socket.recv_from(&mut buf).await?;
    tracing::debug!("Received response from {} of {} bytes", addr, n.1);
    let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    Ok(DNSPacket::parse_dns_packet(&mut buf))
}

pub async fn send_query_async2<R: rand::Rng>(
    rng: &mut R,
    ip_address: IpAddr,
    domain_name: &str,
    record_type: u16,
) -> eyre::Result<DNSPacket> {
    tracing::debug!("Sending query to {}", ip_address);
    let query = build_query(rng, domain_name, record_type);
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    let addr: SocketAddr = SocketAddr::new(ip_address, 53);
    // socket.connect(addr).await?;
    // tracing::debug!("Connected to {}", addr);
    socket.send_to(&query, addr).await?;
    tracing::debug!("Sent query to {}", addr);
    let mut buf = [0; 1024];
    let n = socket.recv_from(&mut buf).await?;
    tracing::debug!("Received response from {} of {} bytes", addr, n.1);
    let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    Ok(DNSPacket::parse_dns_packet(&mut buf))
}

pub fn send_query<R: rand::Rng>(
    rng: &mut R,
    ip_address: IpAddr,
    domain_name: &str,
    record_type: u16,
) -> eyre::Result<DNSPacket> {
    let query = build_query(rng, domain_name, record_type);
    let socket = socket2::Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    let socket: UdpSocket = socket.into();
    println!("socket addr: {:?}", socket.local_addr()?);
    let addr: SocketAddr = SocketAddr::new(ip_address, 53);
    socket.send_to(&query, addr)?;
    let mut buf = [0; 1024];
    socket.recv_from(&mut buf)?;
    let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    Ok(DNSPacket::parse_dns_packet(&mut buf))
}

#[cfg(test)]
mod test {
    use rand::rngs::mock::StepRng;

    use crate::record::TYPE_A;

    use super::*;
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

    #[test]
    fn test_send_query() {
        let mut mock_rng = StepRng::new(0x8298, 0);
        let nameserver: IpAddr = "198.41.0.4".parse().unwrap();
        let res = send_query(&mut mock_rng, nameserver, "www.example.com", TYPE_A).unwrap();
        println!("Response: {:?}", res);
    }
    #[tokio::test]
    async fn test_send_query_async2() {
        let mut mock_rng = StepRng::new(0x8298, 0);
        let nameserver: IpAddr = "198.41.0.4".parse().unwrap();
        let res = send_query_async2(&mut mock_rng, nameserver, "www.example.com", TYPE_A).await.unwrap();
        println!("Response: {:?}", res);
    }
}
