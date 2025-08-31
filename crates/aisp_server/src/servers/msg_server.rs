use std::fs::File;

use aisp_packet::{
    packets::{
        self, PacketId, avatar, channel, enquete,
        msg::{self},
        ping, version,
    },
    shared::{chara, item, server::ServerInfo},
    util::fixed_array::FixedArray,
};

use crate::net::{
    net_error::NetError, packet_handler::ServerHandler, server_backend::ServerBackend,
    vce_peer::VcePeer, vce_server::VceServer,
};

pub struct VceMsgServer {
    listen_ip: String,
    listen_port: u16,

    server: VceServer<VceMsgHandler>,
}
pub struct VceMsgHandler {}

impl VceMsgServer {
    pub fn new(listen_ip: &str, port: u16) -> Self {
        let format_str = format!("{}:{}", listen_ip, port);

        let handler = VceMsgHandler::new();

        Self {
            listen_ip: String::from(listen_ip),
            listen_port: port,

            server: VceServer::new(
                ServerBackend::listen_tcp(&format_str).expect("Failed to listen server"),
                handler,
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

impl VceMsgHandler {
    fn new() -> Self {
        Self {}
    }

    fn load_items(&self) -> Vec<item::ItemData> {
        let mut out_items: Vec<item::ItemData> = Vec::new();

        let item_file = File::open("/home/txt/Documents/RE/aispace/testitems.csv")
            .expect("Failed to load items");

        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(item_file);

        for entry in csv_reader.records() {
            let entry_record = entry.expect("failed to read entry");

            let id = if let Some(val) = entry_record.get(0) {
                val.parse::<u32>().expect("failed to parse id")
            } else {
                continue;
            };

            let slot = if let Some(val) = entry_record.get(1) {
                val.parse::<u32>().expect("failed to parse slot")
            } else {
                continue;
            };

            let name_utf8: String = if let Some(val) = entry_record.get(2) {
                val.into()
            } else {
                continue;
            };

            let item = item::ItemData {
                key: id,
                sorted_list_priority: id,
                item_id: id,

                name: FixedArray::<97>::from_str_to_sjis(&name_utf8),

                socket_1: slot,
                socket_2: slot,

                ..item::ItemData::default()
            };

            out_items.push(item);
        }

        println!("items {:}", out_items.len());

        out_items
    }
}

impl ServerHandler for VceMsgHandler {
    fn on_packet(&mut self, peer: &mut VcePeer, packet: &PacketId) -> Result<(), NetError> {
        println!("msg <- {:?}", packet);

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
            PacketId::IdItemGetBaseListRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::ItemGetBaseListResponse {
                    result: 0,
                    // items: vec![],
                    items: self.load_items(),
                }));

                Ok(())
            }

            PacketId::IdLoginRequest(req) => {
                peer.queue_packet(PacketId::from(msg::LoginResponse { result: 0 }));

                Ok(())
            }

            PacketId::IdAvatarGetDataRequest(req) => {
                // to enable intro, dont send any characters here. probably should somewhat of a db
                // support

                peer.queue_packet(PacketId::from(avatar::AvatarDataResponse {
                    result: 0,
                    name: "test".into(),
                    // model_id: 0, // character model/build id
                    model_id: 1001011, // character model/build id
                    visual: chara::CharaVisual {
                        blood_type: 1,
                        month: 1,
                        day: 1,
                        gender: 1,
                        chara_id: 2,
                        face: 0,
                        hairstyle: 0,
                    },
                    reg_islandid: 0,
                    slot_id: 0,
                    equips: {
                        let mut arr = [item::ItemSlotInfo::default(); 30];
                        // arr[0] = item::ItemSlotInfo {
                        //     id: 2012020,
                        //     socket: 0,
                        // };

                        arr[0] = item::ItemSlotInfo {
                            id: 10100140,
                            socket: 0,
                        };
                        arr[1] = item::ItemSlotInfo {
                            id: 10200130,
                            socket: 0,
                        };
                        arr[2] = item::ItemSlotInfo {
                            id: 10100190,
                            socket: 0,
                        };
                        arr
                    },
                }));
                peer.queue_packet(PacketId::from(avatar::AvatarGetDataResponse {
                    // result: 100,
                    result: 0,
                }));
                // peer.queue_packet(PacketId::from(avatar::AvatarGetDataResponse { result: 0 }));

                Ok(())
            }

            PacketId::IdAvatarGetCreateInfoRequest(req) => {
                peer.queue_packet(PacketId::from(avatar::AvatarGetCreateInfoResponse {
                    m_builds: vec![1001011], // model id, default 1001011 for male, 1002011 for female.
                    m_faces: vec![0],
                    m_hairstyles: vec![0, 10, 20],
                    m_haircolors: vec![0, 1, 2, 3],
                    // equips can have a "ga character", whish is basicly character presets
                    m_equips: vec![item::ItemSlotInfo {
                        id: 2012020,
                        socket: 0,
                    }],

                    f_builds: vec![1002011],
                    f_faces: vec![0],
                    f_hairstyles: vec![0, 10, 20],
                    f_haircolors: vec![0, 1, 2, 3],
                    f_equips: vec![item::ItemSlotInfo {
                        id: 2012030,
                        socket: 0,
                    }],
                }));
                Ok(())
            }

            PacketId::IdAvatarCreateRequest(req) => {
                peer.queue_packet(PacketId::from(avatar::AvatarCreateResponse { result: 0 }));
                Ok(())
            }

            PacketId::IdEnqueteGetRequest(req) => {
                peer.queue_packet(PacketId::from(enquete::EnqueteGetResponse {
                    result: 0,
                    enquetes: vec![enquete::EnqueteData {
                        enquet_id: 0,
                        question: "正体不明の赤い発光体が追っています。どうしますか？".into(),
                        answers: [
                            "全力でにげる".into(),
                            "詳しく観察する".into(),
                            "ホタルでしょう".into(),
                            "叩き潰す".into(),
                            "隠れる".into(),
                            "微笑む".into(),
                            "手を振る".into(),
                            "叫ぶ".into(),
                            "".into(),
                            "".into(),
                        ],
                    }],
                }));
                Ok(())
            }

            PacketId::IdEnqueteAnswerRequest(req) => {
                peer.queue_packet(PacketId::from(enquete::EnqueteAnswerResponse { result: 0 }));

                Ok(())
            }

            PacketId::IdAvatarSelectRequest(req) => {
                peer.queue_packet(PacketId::from(avatar::AvatarSelectResponse { result: 0 }));

                Ok(())
            }
            PacketId::IdChannelListGetRequest(req) => {
                peer.queue_packet(PacketId::from(channel::ChannelListGetResponse {
                    result: 0,
                    channels: vec![channel::ChannelInfo {
                        channel_id: 0,
                        _0x0004: 0,
                        _0x0008: 1,
                        server_info: ServerInfo {
                            address: "127.0.0.1".into(),
                            port: 50054,
                        },
                    }],
                }));

                Ok(())
            }

            PacketId::IdChannelSelectRequest(req) => {
                peer.queue_packet(PacketId::from(channel::ChannelSelectResponse {
                    result: 0,
                    areasv_info: ServerInfo {
                        address: "127.0.0.1".into(),
                        port: 50054,
                    },
                    // akihabara
                    // map_id: 10990100,
                    // map_serial_id: 10990100,
                    // dc2 商店街?
                    // map_id: 10010200,
                    // map_serial_id: 10010200,

                    // clannad school
                    map_id: 10020100,
                    map_serial_id: 10020100,
                    // test level
                    // map_id: 40999900,
                    // map_serial_id: 40999900,
                }));

                Ok(())
            }

            PacketId::IdMailBoxGetDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::mail::MailBoxGetDataResponse {
                    result: 0,
                    mail: vec![],
                }));

                Ok(())
            }

            PacketId::IdCircleGetDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::circle::CircleGetDataResponse {
                    result: 0,
                    circle_data: vec![],
                    auth_level: vec![],
                }));

                Ok(())
            }

            PacketId::IdLogoutRequest(req) => {
                // peer.queue_packet(PacketId::from(msg::LogoutResponse { result: 0 }));

                Ok(())
            }

            _ => Err(NetError::PacketNoHandler),
        }
    }
}
