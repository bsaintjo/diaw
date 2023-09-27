use std::{future::Future, io::Cursor, net::SocketAddr, pin::Pin};

use bytes::Bytes;

use crate::{packet::DNSPacket, r#async::resolve::resolve_async, TYPE_A};

async fn listen_inner(udp: std::net::UdpSocket) -> eyre::Result<()> {
    let mut buf = [0; 1024];
    let socket = tokio::net::UdpSocket::try_from(udp)?;
    // let socket = Arc::new(socket);
    tracing::debug!("Listening on {}", socket.local_addr()?);
    loop {
        let client = socket.recv_from(&mut buf).await?;
        tracing::info!("Received {} bytes from {}", client.0, client.1);
        let packet =
            DNSPacket::parse_dns_packet(&mut Cursor::new(Bytes::copy_from_slice(&buf[..])));
        tracing::info!("Parsed packet: {:?}", packet);
        // let s = socket.clone();
        tokio::spawn(async move {
            let response = resolve_async("example.com", TYPE_A).await?;
            tracing::info!("Response: {:?}", response);
            eyre::Result::<()>::Ok(())
        });
        // let response = resolve_async(&socket, "example.com", TYPE_A).await.unwrap();
    }
}

pub fn listener() -> Result<
    (SocketAddr, Pin<Box<impl Future<Output = eyre::Result<()>>>>),
    Box<dyn std::error::Error>,
> {
    let socket = std::net::UdpSocket::bind("127.0.0.1:0")?;
    socket.set_nonblocking(true)?;
    let address = socket.local_addr()?;
    // let mut buf = [0; 1024];
    // let _ = tokio::spawn(listen_inner(socket));
    Ok((address, Box::pin(listen_inner(socket))))
}
