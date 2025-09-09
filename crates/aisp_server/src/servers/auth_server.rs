use aisp_packet::shared::server::ServerInfo;
use aisp_packet::{
    packets::{PacketId, auth, ping, version, world},
    util::fixed_array::FixedArray,
};

use crate::net::{
    net_error::NetError, packet_handler::ServerHandler, server_backend::ServerBackend,
    vce_peer::VcePeer, vce_server::VceServer,
};

pub struct VceAuthServer {
    // listen_ip: String,
    // listen_port: u16,
    server: VceServer<VceAuthHandler>,
}
pub struct VceAuthHandler {}

impl VceAuthServer {
    pub fn new(listen_ip: &str, port: u16, is_encrypted: bool) -> Self {
        let format_str = format!("{}:{}", listen_ip, port);

        let handler = VceAuthHandler::new();

        Self {
            // listen_ip: String::from(listen_ip),
            // listen_port: port,
            server: VceServer::new(
                ServerBackend::listen_tcp(&format_str).expect("Failed to listen server"),
                handler,
                is_encrypted,
            ),
        }
    }

    pub fn tick(&mut self) {
        self.server.tick();
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        }
    }
}

impl VceAuthHandler {
    fn new() -> Self {
        Self {}
    }
}

impl ServerHandler for VceAuthHandler {
    fn on_packet(&mut self, peer: &mut VcePeer, packet: &PacketId) -> Result<(), NetError> {
        println!("auth <- {:?}", packet);

        match packet {
            PacketId::IdVersionCheck(vers) => {
                peer.queue_packet(PacketId::from(version::VersionCheckResponse {
                    result: 0,
                    sv_major: vers.cl_major,
                    sv_minor: vers.cl_minor,
                    sv_ver: vers.cl_ver,
                }));
                Ok(())
            }
            PacketId::IdPing(ping) => {
                peer.queue_packet(PacketId::from(ping::Ping {
                    cur_time: ping.cur_time,
                }));

                Ok(())
            }

            PacketId::IdAuthenticateRequest(_auth) => {
                peer.queue_packet(PacketId::from(auth::AuthenticateResponseDevelop {
                    user_id: 31874, // This is probably the userid for "hideki@animetoshokan.org"
                }));

                Ok(())
            }
            PacketId::IdNotifyLogout(_) => Ok(()),
            PacketId::IdWorldListRequest(_) => {
                peer.queue_packet(PacketId::from(world::WorldListResponse {
                    result: 0,
                    world_list: vec![world::WorldEntry {
                        world_id: 0,
                        name: "test".into(),
                        description: "test2".into(),
                        _0x0364: 0,
                    }],
                }));
                Ok(())
            }
            PacketId::IdWorldSelectRequest(_) => {
                peer.queue_packet(PacketId::from(world::WorldSelectResponse {
                    result: 0,
                    msgsv_addrs: vec![ServerInfo {
                        port: 50052,
                        address: "127.0.0.1".into(),
                    }],
                    otp: FixedArray::<20>::default(),
                }));
                Ok(())
            }
            _ => Err(NetError::PacketNoHandler),
        }
    }
}
