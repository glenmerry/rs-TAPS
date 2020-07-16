use crate::selection_properties::SelectionProperty;
use crate::selection_properties::PreferenceLevel;

use enum_map::{enum_map, EnumMap};

pub struct TransportProperties {
    selection_properties: EnumMap<SelectionProperty, PreferenceLevel>,
}

impl Default for TransportProperties {
    fn default() -> TransportProperties {
        TransportProperties {
            selection_properties: enum_map! {
                SelectionProperty::Reliability              => PreferenceLevel::Ignore,
                SelectionProperty::PreserveMsgBoundaries    => PreferenceLevel::Ignore,
                SelectionProperty::PerMsgReliability        => PreferenceLevel::Ignore,
                SelectionProperty::PreserveOrder            => PreferenceLevel::Ignore,
                SelectionProperty::ZeroRttMsg               => PreferenceLevel::Ignore,
                SelectionProperty::Multistreaming           => PreferenceLevel::Ignore,
                SelectionProperty::PerMsgChecksumLenSend    => PreferenceLevel::Ignore,
                SelectionProperty::PerMsgChecksumLenRecv    => PreferenceLevel::Ignore,
                SelectionProperty::CongestionControl        => PreferenceLevel::Ignore,
                SelectionProperty::Interface                => PreferenceLevel::Ignore,
                SelectionProperty::Pvd                      => PreferenceLevel::Ignore,
                SelectionProperty::UseTemporaryLocalAddress => PreferenceLevel::Ignore,
                SelectionProperty::Multipath                => PreferenceLevel::Ignore,
                SelectionProperty::Direction                => PreferenceLevel::Ignore,
                SelectionProperty::RetransmitNotify         => PreferenceLevel::Ignore,
                SelectionProperty::SoftErrorNotify          => PreferenceLevel::Ignore,
            }
        }
    }
}

impl TransportProperties {
    pub fn add(&mut self, property: SelectionProperty, preference_level: PreferenceLevel) {
        self.selection_properties[property] = preference_level;
    }

    pub fn require(&mut self, property: SelectionProperty) {
        self.selection_properties[property] = PreferenceLevel::Require;
    }

    pub fn prefer(&mut self, property: SelectionProperty) {
        self.selection_properties[property] = PreferenceLevel::Prefer;
    }

    pub fn ignore(&mut self, property: SelectionProperty) {
        self.selection_properties[property] = PreferenceLevel::Ignore;
    }

    pub fn avoid(&mut self, property: SelectionProperty) {
        self.selection_properties[property] = PreferenceLevel::Avoid;
    }

    pub fn prohibit(&mut self, property: SelectionProperty) {
        self.selection_properties[property] = PreferenceLevel::Prohibit;
    }
}