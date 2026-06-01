use crate::nbt;
use crate::protocol::*;
use std::io;

#[allow(non_upper_case_globals)]
pub mod internal_ids {
    pub const ConfigurationFinishConfigurationServerbound: i32 = 0;
    pub const ConfigurationKeepAliveServerbound_i64: i32 = 1;
    pub const ConfigurationPongServerbound_i32: i32 = 2;
    pub const ConfigurationClientInformationServerbound: i32 = 3;
    pub const ConfigurationResourcePackServerbound: i32 = 4;
    pub const ConfigurationSelectKnownPacksServerbound: i32 = 5;
    pub const ConfigurationCustomClickActionServerbound: i32 = 6;
    pub const ConfigurationAcceptCodeOfConductServerbound: i32 = 7;
    pub const ConfigurationCookieResponseServerbound: i32 = 8;
    pub const ConfigurationCustomPayloadServerbound: i32 = 9;
}

#[derive(Default, Debug)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl Serializable for KnownPack {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<KnownPack, Error> {
        Ok(KnownPack {
            namespace: Serializable::read_from(buf)?,
            id: Serializable::read_from(buf)?,
            version: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.namespace.write_to(buf)?;
        self.id.write_to(buf)?;
        self.version.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationClientInformationServerbound {
    pub language: String,
    pub view_distance: u8,
    pub chat_visibility: VarInt,
    pub chat_colors: bool,
    pub model_customisation: u8,
    pub main_hand: VarInt,
    pub text_filtering_enabled: bool,
    pub allows_listing: bool,
    pub particle_status: VarInt,
}

impl PacketType for ConfigurationClientInformationServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationClientInformationServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.language.write_to(buf)?;
        self.view_distance.write_to(buf)?;
        self.chat_visibility.write_to(buf)?;
        self.chat_colors.write_to(buf)?;
        self.model_customisation.write_to(buf)?;
        self.main_hand.write_to(buf)?;
        self.text_filtering_enabled.write_to(buf)?;
        self.allows_listing.write_to(buf)?;
        self.particle_status.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationCookieResponseServerbound {
    pub key: String,
    pub payload: Option<LenPrefixedBytes<VarInt>>,
}

impl PacketType for ConfigurationCookieResponseServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationCookieResponseServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        self.payload.is_some().write_to(buf)?;
        if let Some(ref payload) = self.payload {
            payload.write_to(buf)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationCustomPayloadServerbound {
    pub channel: String,
    pub data: Vec<u8>,
}

impl PacketType for ConfigurationCustomPayloadServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationCustomPayloadServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.channel.write_to(buf)?;
        self.data.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationFinishConfigurationServerbound {
    pub empty: (),
}

impl PacketType for ConfigurationFinishConfigurationServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationFinishConfigurationServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationKeepAliveServerbound_i64 {
    pub id: i64,
}

impl PacketType for ConfigurationKeepAliveServerbound_i64 {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationKeepAliveServerbound_i64,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationPongServerbound_i32 {
    pub id: i32,
}

impl PacketType for ConfigurationPongServerbound_i32 {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationPongServerbound_i32,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationResourcePackServerbound {
    pub id: UUID,
    pub action: VarInt,
}

impl PacketType for ConfigurationResourcePackServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationResourcePackServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        self.action.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationSelectKnownPacksServerbound {
    pub known_packs: LenPrefixed<VarInt, KnownPack>,
}

impl PacketType for ConfigurationSelectKnownPacksServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationSelectKnownPacksServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.known_packs.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationCustomClickActionServerbound {
    pub id: String,
    pub payload: Option<nbt::NamedTag>,
}

impl PacketType for ConfigurationCustomClickActionServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationCustomClickActionServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        match self.payload {
            Some(ref payload) => {
                let mut payload_bytes = Vec::new();
                10u8.write_to(&mut payload_bytes)?;
                payload.1.write_to(&mut payload_bytes)?;
                VarInt(payload_bytes.len() as i32).write_to(buf)?;
                buf.write_all(&payload_bytes)?;
            }
            None => VarInt(0).write_to(buf)?,
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationAcceptCodeOfConductServerbound {
    pub empty: (),
}

impl PacketType for ConfigurationAcceptCodeOfConductServerbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Serverbound,
            internal_ids::ConfigurationAcceptCodeOfConductServerbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}
