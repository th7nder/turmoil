//! This module contains the simulated TCP/UDP networking types.
//!
//! They mirror [tokio::net](https://docs.rs/tokio/latest/tokio/net/) to provide
//! a high fidelity implementation.

mod listener;
use std::net::SocketAddr;

pub use listener::TcpListener;

mod stream;
pub use stream::TcpStream;

mod udp;
pub use udp::UdpSocket;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct SocketPair {
    pub(crate) local: SocketAddr,
    pub(crate) remote: SocketAddr,
}

impl SocketPair {
    pub(crate) fn new(local: SocketAddr, remote: SocketAddr) -> SocketPair {
        assert_ne!(local, remote);
        SocketPair { local, remote }
    }
}
