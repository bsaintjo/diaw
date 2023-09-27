use std::{str::FromStr, sync::OnceLock, thread};

use diaw::r#async::listener;
use tracing::Level;
use trust_dns_client::{
    client::{AsyncClient, Client, ClientHandle, SyncClient},
    proto::udp::UdpClientConnect,
    rr::{DNSClass, Name, RecordType},
    udp::{UdpClientConnection, UdpClientStream},
};

static LOGGER: OnceLock<()> = OnceLock::new();

fn init_logger() {
    LOGGER.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .init();
    });
}

#[test]
fn test_resolver() -> eyre::Result<()> {
    init_logger();
    let (server_addr, server_future) = listener::listener().unwrap();
    thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(server_future)
    });

    let conn = UdpClientConnection::new(server_addr).unwrap();
    let client = SyncClient::new(conn);
    let name = Name::from_str("www.example.com.")?;
    let response = client.query(&name, DNSClass::IN, RecordType::A)?;
    Ok(())
}

#[tokio::test]
async fn test_resolver_async() {
    init_logger();
    let (server_addr, server_future) = listener::listener().unwrap();
    tokio::spawn(server_future);
    let conn: UdpClientConnect<tokio::net::UdpSocket> = UdpClientStream::new(server_addr);
    let (mut client, bg) = AsyncClient::connect(conn).await.unwrap();
    tokio::spawn(bg);
    let name = Name::from_str("www.example.com.").unwrap();
    let query = client.query(name, DNSClass::IN, RecordType::A);
    let response = query.await.unwrap();
}
