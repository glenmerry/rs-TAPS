use enum_map::Enum;

#[derive(Debug, Enum)]
pub enum PreferenceLevel {
    Require, // Select only protocols/paths providing the property, fail otherwise
    Prefer, // Prefer protocols/paths providing the property, proceed otherwise
    Ignore, // No preference
    Avoid, // Prefer protocols/paths not providing the property, proceed otherwise
    Prohibit, // Select only protocols/paths not providing the property, fail otherwise
}

#[derive(Debug, Enum)]
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
    Interface,
    Pvd,
    UseTemporaryLocalAddress,
    Multipath,
    Direction,
    RetransmitNotify,
    SoftErrorNotify,
}
