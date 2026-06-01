use std::io;

use crate::protocol::{format, Error, Serializable, VarInt, UUID};

#[derive(Debug)]
pub struct PlayerInfoData {
    pub action: VarInt,
    pub players: Vec<PlayerDetail>,
}

impl Serializable for PlayerInfoData {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let mut m = PlayerInfoData {
            action: Serializable::read_from(buf)?,
            players: Vec::new(),
        };
        let len = VarInt::read_from(buf)?;
        for _ in 0..len.0 {
            let uuid = UUID::read_from(buf)?;
            match m.action.0 {
                0 => {
                    let name = String::read_from(buf)?;
                    let mut props = Vec::new();
                    let plen = VarInt::read_from(buf)?.0;
                    for _ in 0..plen {
                        let mut prop = PlayerProperty {
                            name: String::read_from(buf)?,
                            value: String::read_from(buf)?,
                            signature: Default::default(),
                        };
                        if bool::read_from(buf)? {
                            prop.signature = Some(String::read_from(buf)?);
                        }
                        props.push(prop);
                    }
                    let p = PlayerDetail::Add {
                        uuid,
                        name,
                        properties: props,
                        gamemode: Serializable::read_from(buf)?,
                        ping: Serializable::read_from(buf)?,
                        display: {
                            if bool::read_from(buf)? {
                                Some(Serializable::read_from(buf)?)
                            } else {
                                None
                            }
                        },
                    };
                    m.players.push(p);
                }
                1 => m.players.push(PlayerDetail::UpdateGamemode {
                    uuid,
                    gamemode: Serializable::read_from(buf)?,
                }),
                2 => m.players.push(PlayerDetail::UpdateLatency {
                    uuid,
                    ping: Serializable::read_from(buf)?,
                }),
                3 => m.players.push(PlayerDetail::UpdateDisplayName {
                    uuid,
                    display: {
                        if bool::read_from(buf)? {
                            Some(Serializable::read_from(buf)?)
                        } else {
                            None
                        }
                    },
                }),
                4 => m.players.push(PlayerDetail::Remove { uuid }),
                _ => panic!(),
            }
        }
        Ok(m)
    }

    fn write_to<W: io::Write>(&self, _: &mut W) -> Result<(), Error> {
        unimplemented!() // I'm lazy
    }
}

impl Default for PlayerInfoData {
    fn default() -> Self {
        PlayerInfoData {
            action: VarInt(0),
            players: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum PlayerDetail {
    Add {
        uuid: UUID,
        name: String,
        properties: Vec<PlayerProperty>,
        gamemode: VarInt,
        ping: VarInt,
        display: Option<format::Component>,
    },
    UpdateGamemode {
        uuid: UUID,
        gamemode: VarInt,
    },
    UpdateLatency {
        uuid: UUID,
        ping: VarInt,
    },
    UpdateDisplayName {
        uuid: UUID,
        display: Option<format::Component>,
    },
    Remove {
        uuid: UUID,
    },
}

#[derive(Debug)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
