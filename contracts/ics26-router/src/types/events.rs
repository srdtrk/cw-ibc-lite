//! `cw-ibc-lite-ics26-router` Event Keys

/// `EVENT_TYPE_REGISTER_IBC_APP` is the event type for a register IBC app event
pub const EVENT_TYPE_REGISTER_IBC_APP: &str = "register_ibc_app";
/// `EVENT_TYPE_SEND_PACKET` is the event type for a send packet event
pub const EVENT_TYPE_SEND_PACKET: &str = "send_packet";
/// `EVENT_TYPE_RECV_PACKET` is the event type for a receive packet event
pub const EVENT_TYPE_RECV_PACKET: &str = "recv_packet";
/// `EVENT_TYPE_ACKNOWLEDGE_PACKET` is the event type for writing an acknowledgement
pub const EVENT_TYPE_WRITE_ACKNOWLEDGEMENT: &str = "write_acknowledgement";

/// `ATTRIBUTE_KEY_CONTRACT_ADDRESS` is the attribute key for the contract address
pub const ATTRIBUTE_KEY_CONTRACT_ADDRESS: &str = "contract_address";
/// `ATTRIBUTE_KEY_PORT_ID` is the attribute key for the port id
pub const ATTRIBUTE_KEY_PORT_ID: &str = "port_id";
/// `ATTRIBUTE_KEY_SENDER` is the attribute key for the sender
pub const ATTRIBUTE_KEY_SENDER: &str = "sender";
/// `ATTRIBUTE_KEY_DATA_HEX` is the attribute key for the packet data hex
pub const ATTRIBUTE_KEY_DATA_HEX: &str = "packet_data_hex";
/// `ATTRIBUTE_KEY_TIMEOUT_TIMESTAMP` is the attribute key for the packet timeout timestamp
pub const ATTRIBUTE_KEY_TIMEOUT_TIMESTAMP: &str = "packet_timeout_timestamp";
/// `ATTRIBUTE_KEY_SEQUENCE` is the attribute key for the packet sequence
pub const ATTRIBUTE_KEY_SEQUENCE: &str = "packet_sequence";
/// `ATTRIBUTE_KEY_SRC_PORT` is the attribute key for the packet source port
pub const ATTRIBUTE_KEY_SRC_PORT: &str = "packet_src_port";
/// `ATTRIBUTE_KEY_SRC_CHANNEL` is the attribute key for the packet source channel
pub const ATTRIBUTE_KEY_SRC_CHANNEL: &str = "packet_src_channel";
/// `ATTRIBUTE_KEY_DST_PORT` is the attribute key for the packet destination port
pub const ATTRIBUTE_KEY_DST_PORT: &str = "packet_dst_port";
/// `ATTRIBUTE_KEY_DST_CHANNEL` is the attribute key for the packet destination channel
pub const ATTRIBUTE_KEY_DST_CHANNEL: &str = "packet_dst_channel";
/// `ATTRIBUTE_KEY_ACK_DATA_HEX` is the attribute key for the packet acknowledgement data hex
pub const ATTRIBUTE_KEY_ACK_DATA_HEX: &str = "packet_ack_hex";

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::RegisterIbcApp`]
pub mod register_ibc_app {
    use cosmwasm_std::{Attribute, Event};

    /// `register_ibc_app` is the event message for a register IBC app event
    #[must_use]
    pub fn success(port_id: &str, contract_address: &str, sender: &str) -> Event {
        Event::new(super::EVENT_TYPE_REGISTER_IBC_APP).add_attributes(vec![
            Attribute::new(super::ATTRIBUTE_KEY_CONTRACT_ADDRESS, contract_address),
            Attribute::new(super::ATTRIBUTE_KEY_PORT_ID, port_id),
            Attribute::new(super::ATTRIBUTE_KEY_SENDER, sender),
        ])
    }
}

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::SendPacket`]
pub mod send_packet {
    use cosmwasm_std::{Attribute, Event, HexBinary};
    use cw_ibc_lite_shared::types::ibc::Packet;

    /// `send_packet` is the event message for a send packet event
    ///
    /// # Panics
    /// Panics if the packet timeout timestamp is not set
    #[must_use]
    pub fn success(packet: &Packet) -> Event {
        Event::new(super::EVENT_TYPE_SEND_PACKET).add_attributes(vec![
            Attribute::new(
                super::ATTRIBUTE_KEY_DATA_HEX,
                HexBinary::from(packet.data.as_slice()).to_hex(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_TIMEOUT_TIMESTAMP,
                packet.timeout.timestamp().unwrap().nanos().to_string(),
            ),
            Attribute::new(super::ATTRIBUTE_KEY_SEQUENCE, packet.sequence.to_string()),
            Attribute::new(super::ATTRIBUTE_KEY_SRC_PORT, packet.source_port.as_str()),
            Attribute::new(
                super::ATTRIBUTE_KEY_SRC_CHANNEL,
                packet.source_channel.as_str(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_DST_PORT,
                packet.destination_port.as_str(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_DST_CHANNEL,
                packet.destination_channel.as_str(),
            ),
        ])
    }
}

/// Contains event messages emitted during [`super::super::msg::ExecuteMsg::RecvPacket`]
pub mod recv_packet {
    use cosmwasm_std::{Attribute, Event, HexBinary};
    use cw_ibc_lite_shared::types::ibc::Packet;

    /// `recv_packet` is the event message for a receive packet event
    #[must_use]
    pub fn success(packet: &Packet) -> Event {
        Event::new(super::EVENT_TYPE_RECV_PACKET).add_attributes(vec![
            Attribute::new(
                super::ATTRIBUTE_KEY_DATA_HEX,
                HexBinary::from(packet.data.as_slice()).to_hex(),
            ),
            Attribute::new(super::ATTRIBUTE_KEY_SRC_PORT, packet.source_port.as_str()),
            Attribute::new(
                super::ATTRIBUTE_KEY_SRC_CHANNEL,
                packet.source_channel.as_str(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_DST_PORT,
                packet.destination_port.as_str(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_DST_CHANNEL,
                packet.destination_channel.as_str(),
            ),
        ])
    }
}

/// Contains event messages emitted during the reply to
/// [`cw_ibc_lite_shared::types::apps::callbacks::IbcAppCallbackMsg::OnRecvPacket`]
pub mod write_acknowledgement {
    use cosmwasm_std::{Attribute, Event, HexBinary};
    use cw_ibc_lite_shared::types::ibc;

    /// `write_acknowledgement` is the event message for writing an acknowledgement
    #[must_use]
    pub fn success(packet: &ibc::Packet, ack: &ibc::Acknowledgement) -> Event {
        Event::new(super::EVENT_TYPE_WRITE_ACKNOWLEDGEMENT).add_attributes(vec![
            Attribute::new(
                super::ATTRIBUTE_KEY_DATA_HEX,
                HexBinary::from(packet.data.as_slice()).to_hex(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_ACK_DATA_HEX,
                HexBinary::from(ack.as_slice()).to_hex(),
            ),
            Attribute::new(super::ATTRIBUTE_KEY_SRC_PORT, packet.source_port.as_str()),
            Attribute::new(
                super::ATTRIBUTE_KEY_SRC_CHANNEL,
                packet.source_channel.as_str(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_DST_PORT,
                packet.destination_port.as_str(),
            ),
            Attribute::new(
                super::ATTRIBUTE_KEY_DST_CHANNEL,
                packet.destination_channel.as_str(),
            ),
        ])
    }
}
