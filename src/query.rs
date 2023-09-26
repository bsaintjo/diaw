use std::{
    io::Cursor,
    net::{IpAddr, SocketAddr},
};

use bytes::Bytes;

use crate::simple::query::build_query;

use crate::packet::DNSPacket;

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
    socket.send_to(&query, addr).await?;
    tracing::debug!("Sent query to {}", addr);
    let mut buf = [0; 1024];
    let n = socket.recv_from(&mut buf).await?;
    tracing::debug!("Received response from {} of {} bytes", addr, n.1);
    let mut buf = Cursor::new(Bytes::copy_from_slice(&buf[..]));
    Ok(DNSPacket::parse_dns_packet(&mut buf))
}

#[cfg(test)]
mod test {
    use std::net::IpAddr;

    use rand::rngs::mock::StepRng;

    use crate::{query::send_query_async2, TYPE_A};

    #[tokio::test]
    async fn test_send_query_async2() {
        let mut mock_rng = StepRng::new(0x8298, 0);
        let nameserver: IpAddr = "198.41.0.4".parse().unwrap();
        let res = send_query_async2(&mut mock_rng, nameserver, "www.example.com", TYPE_A)
            .await
            .unwrap();
        println!("Response: {:?}", res);
    }
}
