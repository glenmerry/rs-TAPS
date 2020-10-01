use std::collections::HashMap;

use enum_map::{Enum, enum_map, EnumMap};

#[derive(Debug, Enum, PartialEq, Copy, Clone)]
pub enum PreferenceLevel {
    Require, // Select only protocols/paths providing the property, fail otherwise
    Prefer, // Prefer protocols/paths providing the property, proceed otherwise
    Ignore, // No preference
    Avoid, // Prefer protocols/paths not providing the property, proceed otherwise
    Prohibit, // Select only protocols/paths not providing the property, fail otherwise
}

#[derive(Debug, Enum, PartialEq, Copy, Clone)]
pub enum ServiceLevel{
    Provided,
    Optional,
    NotProvided,
}

#[derive(Debug, Enum, Copy, Clone)]
pub enum SelectionProperty {
    Reliability,
    PreserveMsgBoundaries,
    PerMsgReliability,
    PreserveOrder,
    ZeroRttMsg,
    Multistreaming,
    PerMsgChecksumLenSend,
    PerMsgChecksumLenRecv,
    CongestionControl,
    Multipath,
    Direction,
    RetransmitNotify,
    SoftErrorNotify,
}

pub fn get_supported_protocols() -> HashMap<&'static str, EnumMap<SelectionProperty, ServiceLevel>> {
    let mut supported_protocols = HashMap::new();

    supported_protocols.insert("tcp", enum_map! {
            SelectionProperty::Reliability              => ServiceLevel::Provided,
            SelectionProperty::PreserveMsgBoundaries    => ServiceLevel::NotProvided,
            SelectionProperty::PerMsgReliability        => ServiceLevel::NotProvided,
            SelectionProperty::PreserveOrder            => ServiceLevel::Provided,
            SelectionProperty::ZeroRttMsg               => ServiceLevel::Optional,
            SelectionProperty::Multistreaming           => ServiceLevel::NotProvided,
            SelectionProperty::PerMsgChecksumLenSend    => ServiceLevel::NotProvided,
            SelectionProperty::PerMsgChecksumLenRecv    => ServiceLevel::NotProvided,
            SelectionProperty::CongestionControl        => ServiceLevel::Provided,
            SelectionProperty::Multipath                => ServiceLevel::Optional,
            SelectionProperty::Direction                => ServiceLevel::Provided,
            SelectionProperty::RetransmitNotify         => ServiceLevel::Provided,
            SelectionProperty::SoftErrorNotify          => ServiceLevel::Provided,
    });

    supported_protocols.insert("udp", enum_map! {
        SelectionProperty::Reliability              => ServiceLevel::NotProvided,
        SelectionProperty::PreserveMsgBoundaries    => ServiceLevel::Provided,
        SelectionProperty::PerMsgReliability        => ServiceLevel::NotProvided,
        SelectionProperty::PreserveOrder            => ServiceLevel::NotProvided,
        SelectionProperty::ZeroRttMsg               => ServiceLevel::Provided,
        SelectionProperty::Multistreaming           => ServiceLevel::NotProvided,
        SelectionProperty::PerMsgChecksumLenSend    => ServiceLevel::NotProvided,
        SelectionProperty::PerMsgChecksumLenRecv    => ServiceLevel::NotProvided,
        SelectionProperty::CongestionControl        => ServiceLevel::NotProvided,
        SelectionProperty::Multipath                => ServiceLevel::NotProvided,
        SelectionProperty::Direction                => ServiceLevel::Provided,
        SelectionProperty::RetransmitNotify         => ServiceLevel::NotProvided,
        SelectionProperty::SoftErrorNotify          => ServiceLevel::Provided,
    });

    supported_protocols.insert("quic", enum_map! {
        SelectionProperty::Reliability              => ServiceLevel::Provided,
        SelectionProperty::PreserveMsgBoundaries    => ServiceLevel::NotProvided,
        SelectionProperty::PerMsgReliability        => ServiceLevel::NotProvided,
        SelectionProperty::PreserveOrder            => ServiceLevel::Provided,
        SelectionProperty::ZeroRttMsg               => ServiceLevel::Optional,
        SelectionProperty::Multistreaming           => ServiceLevel::Optional,
        SelectionProperty::PerMsgChecksumLenSend    => ServiceLevel::NotProvided,
        SelectionProperty::PerMsgChecksumLenRecv    => ServiceLevel::NotProvided,
        SelectionProperty::CongestionControl        => ServiceLevel::Provided,
        SelectionProperty::Multipath                => ServiceLevel::NotProvided,
        SelectionProperty::Direction                => ServiceLevel::Provided, //?????
        SelectionProperty::RetransmitNotify         => ServiceLevel::NotProvided,
        SelectionProperty::SoftErrorNotify          => ServiceLevel::Provided,
    });

    return supported_protocols
}
