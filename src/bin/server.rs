use std::io::Cursor;

use bytes::Bytes;
use diaw::{packet::DNSPacket, TYPE_A, resolve::resolve_async};
use tracing::Level;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let socket = tokio::net::UdpSocket::bind("127.0.0.1:7777").await?;
    let mut buf = [0; 1024];
    loop {
        let client = socket.recv_from(&mut buf).await?;
        tracing::info!("Received {} bytes from {}", client.0, client.1);
        let packet =
            DNSPacket::parse_dns_packet(&mut Cursor::new(Bytes::copy_from_slice(&buf[..])));
        tracing::info!("Parsed packet: {:?}", packet);
        tokio::spawn(async move {
            let response = resolve_async("example.com", TYPE_A).await;
        });
    }
}
