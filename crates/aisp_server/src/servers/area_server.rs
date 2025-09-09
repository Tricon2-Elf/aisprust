use aisp_packet::{
    packets::{self, PacketId, areasv, ping, robo, version},
    shared::{chara, item},
};

use crate::net::{
    net_error::NetError, packet_handler::ServerHandler, server_backend::ServerBackend,
    vce_peer::VcePeer, vce_server::VceServer,
};

pub struct VceAreaServer {
    // listen_ip: String,
    // listen_port: u16,
    server: VceServer<VceAreaHandler>,
}
pub struct VceAreaHandler {}

impl VceAreaServer {
    pub fn new(listen_ip: &str, port: u16, is_encrypted: bool) -> Self {
        let format_str = format!("{}:{}", listen_ip, port);

        let handler = VceAreaHandler::new();

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

impl VceAreaHandler {
    fn new() -> Self {
        Self {}
    }
}

impl ServerHandler for VceAreaHandler {
    fn on_packet(&mut self, peer: &mut VcePeer, packet: &PacketId) -> Result<(), NetError> {
        println!("area <- {:?}", packet);

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
            PacketId::IdAreasvEnterRequest(req) => {
                peer.queue_packet(PacketId::from(areasv::AreasvEnterResponse {
                    result: 0,
                    obj_id: 0,
                }));

                Ok(())
            }
            PacketId::IdAreasvLeaveRequest(req) => {
                peer.queue_packet(PacketId::from(areasv::AreasvLeaveResponse { result: 0 }));

                Ok(())
            }

            PacketId::IdAvatarMove(req) => {
                // for moves in &req.moves {
                //     println!(
                //         "move ({}, {}, {}), {} {}",
                //         moves.position[0],
                //         moves.position[1],
                //         moves.position[2],
                //         moves.yaw,
                //         moves._0x000d
                //     );
                // }
                Ok(())
            }

            // start of data request stuff
            PacketId::IdAvatarGetDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::avatar::AvatarNotifyData {
                    result: 0,
                    avatar_data: packets::avatar::AvatarData {
                        avatar_id: 0,
                        chara: chara::CharaData {
                            chara_id: 0,
                            name: "test".into(),
                            visual: chara::CharaVisual {
                                blood_type: 1,
                                month: 1,
                                day: 1,
                                gender: 1,
                                chara_id: 2,
                                face: 0,
                                hairstyle: 0,
                            },

                            equipment: {
                                let mut arr = [item::ItemSlotInfo::default(); 30];

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
                            ..chara::CharaData::default()
                        },
                        ..packets::avatar::AvatarData::default()
                    },
                }));

                Ok(())
            }

            PacketId::IdRoboGetListRequest(req) => {
                peer.queue_packet(PacketId::from(robo::RoboGetListResponse {
                    result: 0,
                    robo_datas: vec![],
                }));

                Ok(())
            }

            PacketId::IdItemGetListRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::ItemGetListResponse {
                    result: 0,
                }));

                Ok(())
            }
            PacketId::IdEquipOrderListRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::EquipOrderListResponse {
                    result: 0,
                    chara_order: vec![],
                    job_order: vec![],
                }));

                Ok(())
            }

            PacketId::IdFurnitureGetBaseListRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::item::FurnitureGetBaseListResponse {
                        result: 0,
                        furniture: vec![],
                    },
                ));

                Ok(())
            }

            PacketId::IdEmotionGetBaseListRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::EmotionGetBaseListResponse {
                    result: 0,
                    emotions: vec![],
                }));

                Ok(())
            }
            PacketId::IdUccAdvFigureBaseListRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::item::UccAdvFigureBaseListResponse {
                        result: 0,
                        adv_figures: vec![],
                    },
                ));

                Ok(())
            }

            PacketId::IdUccVoiceBaseListRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::UccVoiceBaseListResponse {
                    result: 0,
                    voice_data: vec![],
                }));

                Ok(())
            }

            PacketId::IdNiconiCommonsBaseListRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::item::NiconiCommonsBaseListResponse {
                        result: 0,
                        commons_base: vec![],
                    },
                ));

                Ok(())
            }

            PacketId::IdMissionDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::MissionDataResponse {
                    result: 0,
                }));

                Ok(())
            }

            PacketId::IdMapDataEnterEndRequest(req) => {
                peer.queue_packet(PacketId::from(packets::item::MapDataEnterEndResponse {
                    result: 0,
                }));

                Ok(())
            }

            // end of get data stuff

            // -- level load
            PacketId::IdFriendLinkTagGetRequest(req) => {
                peer.queue_packet(PacketId::from(packets::friend::FriendLinkTagGetResponse {
                    result: 0,
                    avatar_id: 0,
                    tagdata: vec![],
                    slot: vec![],
                    questionnaire_tagdata: vec![],
                    questionnaire_slot: vec![],
                }));

                Ok(())
            }

            PacketId::IdFriendGetListDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::friend::FriendGetListDataResponse {
                    result: 0,
                    friend_data: vec![],
                    already_in: vec![],
                    comment: vec![],
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

            PacketId::IdEmotionGetObtainedListRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::item::EmotionGetObtainedListResponse {
                        result: 0,
                        emotion_ids: vec![],
                    },
                ));

                Ok(())
            }

            PacketId::IdRoboGetObtainedSkillListRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::robo::RoboGetObtainedSkillListResponse {
                        result: 0,
                        robo_id: req.robo_id,
                        skill_id: vec![],
                    },
                ));

                Ok(())
            }

            PacketId::IdHeroineGetTicketBaseRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::heroine::HeroineGetTicketBaseResponse {
                        heroine_tickets: vec![],
                    },
                ));

                Ok(())
            }

            PacketId::IdNpcGetDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::npc::NpcGetDataResponse {
                    result: 0,
                }));

                Ok(())
            }

            PacketId::IdMapLinkGetDataRequest(req) => {
                peer.queue_packet(PacketId::from(packets::world::MapLinkGetDataResponse {
                    result: 0,
                }));

                Ok(())
            }
            PacketId::IdMyRoomGetFurnitureRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::myroom::MyRoomGetFurnitureResponse { result: 0 },
                ));

                Ok(())
            }
            // -- level load end
            PacketId::IdTimeZoneGetRequest(req) => {
                peer.queue_packet(PacketId::from(packets::misc::TimeZoneGetResponse {
                    result: 0,
                    time_zone: 1,
                    time: 0,
                    time_zone_max: 8,
                    flag: 0,
                }));

                Ok(())
            }

            PacketId::IdMascotGetCountRequest(req) => {
                peer.queue_packet(PacketId::from(packets::mascot::MascotGetCountResponse {
                    result: 0,
                    count: 0,
                    serial_id: 0,
                    name: vec![],
                }));

                Ok(())
            }

            PacketId::IdAdventureUploadRateGetRequest(req) => {
                peer.queue_packet(PacketId::from(
                    packets::adventure::AdventureUploadRateGetResponse {
                        // rate: 100, // rate * unk / 100
                        rate: 1,
                    },
                ));

                Ok(())
            }

            PacketId::IdAiDownloadListGetRequest(req) => {
                peer.queue_packet(PacketId::from(packets::ai::AiDownloadListGetResponse {
                    result: 0,
                    downs: vec![],
                }));

                Ok(())
            }
            PacketId::IdAiUploadRateGetRequest(req) => {
                peer.queue_packet(PacketId::from(packets::ai::AiUploadRateGetResponse {
                    // rate: 100,
                    rate: 1,
                }));

                Ok(())
            }

            PacketId::IdMoneyDataGetRequest(req) => {
                peer.queue_packet(PacketId::from(packets::misc::MoneyDataGetResponse {
                    result: 0,
                }));

                Ok(())
            }

            PacketId::IdUpdateOptionRequest(req) => {
                peer.queue_packet(PacketId::from(packets::misc::UpdateOptionResponse {
                    result: 0,
                }));

                Ok(())
            }

            PacketId::IdRoboVoiceTypeUpdateRequest(req) => {
                peer.queue_packet(PacketId::from(packets::robo::RoboVoiceTypeUpdateResponse {
                    result: 0,
                    voice_type: req.voice_type,
                }));

                Ok(())
            }

            PacketId::IdMapEnterRequest(req) => {
                peer.queue_packet(PacketId::from(packets::world::MapEnterResponse {
                    result: 0,
                }));

                Ok(())
            }

            _ => Err(NetError::PacketNoHandler),
        }
    }
}
