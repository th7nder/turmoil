use bytes::{Buf, BufMut, Bytes};
use tokio::sync::{mpsc, Mutex};

use crate::{
    envelope::{Datagram, Protocol},
    ToSocketAddrs, World, TRACING_TARGET,
};

use std::{cmp, io::Result, net::{SocketAddr}};

/// A simulated UDP socket.
///
/// All methods must be called from a host within a Turmoil simulation.
pub struct UdpSocket {
    local_addr: SocketAddr,
    rx: Mutex<mpsc::Receiver<(Datagram, SocketAddr)>>,
}

impl UdpSocket {
    pub(crate) fn new(local_addr: SocketAddr, rx: mpsc::Receiver<(Datagram, SocketAddr)>) -> Self {
        Self {
            local_addr,
            rx: Mutex::new(rx),
        }
    }

    /// Create a new simulated UDP socket and attempt to bind it to the `addr`
    /// provided.
    ///
    /// Supports binding to IPv4/IPv6 interfaces:
    /// - Unspecified: 0.0.0.0, :: 
    /// - Loopback: 127.0.0.1, ::1
    /// Binding directly to an IP address other than loopback is unsupported.
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<UdpSocket> {
        World::current(|world| {
            let addr = addr.to_socket_addr(&world.dns);
            let host = world.current_host_mut();

            host.udp.bind(addr)
        })
    }

    /// Sends data on the socket to the given address. On success, returns the
    /// number of bytes written.
    pub async fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], target: A) -> Result<usize> {
        World::current(|world| {
            let dst = target.to_socket_addr(&world.dns);

            // Use the sending host's primary address as sending interface.
            let src = if self.local_addr.ip().is_unspecified() {
                let host_addr = world.current_host_mut().addr;
                (host_addr, self.local_addr.port()).into()
            } else {
                self.local_addr
            };

            world.send_message(
                src,
                dst,
                Protocol::Udp(Datagram(Bytes::copy_from_slice(buf))),
            );

            Ok(buf.len())
        })
    }

    /// Receives a single datagram message on the socket. On success, returns
    /// the number of bytes read and the origin.
    ///
    /// The function must be called with valid byte array buf of sufficient size
    /// to hold the message bytes. If a message is too long to fit in the
    /// supplied buffer, excess bytes may be discarded.
    pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        let (datagram, origin) = self.rx.lock().await.recv().await.unwrap();

        tracing::trace!(target: TRACING_TARGET, local_addr = ?self.local_addr, src = ?origin, protocol = %datagram, "Recv");

        let bytes = datagram.0;
        let limit = cmp::min(buf.len(), bytes.len());

        buf.as_mut().put(bytes.take(limit));

        Ok((limit, origin))
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        World::current_if_set(|world| world.current_host_mut().udp.unbind(self.local_addr));
    }
}
