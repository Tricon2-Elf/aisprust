use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, UdpSocket};

pub enum ServerBackend {
    None,
    Tcp(TcpListener),
    Udp,
    Loopback,
}

pub enum NetworkStream {
    Tcp(TcpStream),
    Udp(UdpSocket),
}

impl ServerBackend {
    pub fn listen_tcp(bind_addr: &str) -> io::Result<ServerBackend> {
        println!("Listening on [{}]", bind_addr);
        let listener = TcpListener::bind(bind_addr)?;

        listener
            .set_nonblocking(true)
            .expect("Listener: failed to set nonblocking");

        Ok(ServerBackend::Tcp(listener))
    }

    pub fn accept(&self) -> io::Result<NetworkStream> {
        match self {
            ServerBackend::Tcp(tcp) => match tcp.accept() {
                Ok((stream, _addr)) => {
                    stream
                        .set_nonblocking(true)
                        .expect("tcp Peer: failed to set nonblocking");
                    Ok(NetworkStream::Tcp(stream))
                }
                Err(e) => Err(e),
            },
            _ => unimplemented!(),
        }
    }

    // pub fn incoming(&self) -> impl Iterator<Item = io::Result<NetworkStream>> + '_ {
    //     match self {
    //         ServerBackend::Tcp(tcp) => tcp.incoming().map(|in_stream| in_stream.map(NetworkStream::Tcp)),
    //         _ => unimplemented!(),
    //     }
    // }
}

impl NetworkStream {
    pub fn receive(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            NetworkStream::Tcp(stream) => stream.read(buf),
            NetworkStream::Udp(socket) => {
                let (len, src) = socket.recv_from(buf)?;
                Ok(len)
            }
        }
    }

    pub fn send(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            NetworkStream::Tcp(stream) => stream.write(buf),
            NetworkStream::Udp(socket) => {
                panic!("udp sentto not supported")
            }
        }
    }
    pub fn set_nonblocking(&self, value: bool) -> io::Result<()> {
        match self {
            NetworkStream::Tcp(stream) => stream.set_nonblocking(value),
            NetworkStream::Udp(socket) => {
                panic!("udp sentto not supported")
            }
        }
    }

    pub fn peer_address(&self) -> io::Result<SocketAddr> {
        match self {
            NetworkStream::Tcp(stream) => stream.peer_addr(),
            NetworkStream::Udp(socket) => {
                panic!("udp sentto not supported")
            }
        }
    }
}
