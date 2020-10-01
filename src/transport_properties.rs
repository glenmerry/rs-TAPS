use crate::selection_properties::SelectionProperty;
use crate::selection_properties::PreferenceLevel;

use enum_map::{enum_map, EnumMap};

#[derive(Debug, Copy, Clone)]
pub struct TransportProperties {
    pub selection_properties: EnumMap<SelectionProperty, PreferenceLevel>,
}

impl Default for TransportProperties {
    fn default() -> TransportProperties {
        TransportProperties {
            selection_properties: enum_map! {
                SelectionProperty::Reliability              => PreferenceLevel::Require,
                SelectionProperty::PreserveMsgBoundaries    => PreferenceLevel::Prefer,
                SelectionProperty::PerMsgReliability        => PreferenceLevel::Ignore,
                SelectionProperty::PreserveOrder            => PreferenceLevel::Require,
                SelectionProperty::ZeroRttMsg               => PreferenceLevel::Prefer,
                SelectionProperty::Multistreaming           => PreferenceLevel::Prefer,
                SelectionProperty::PerMsgChecksumLenSend    => PreferenceLevel::Ignore,
                SelectionProperty::PerMsgChecksumLenRecv    => PreferenceLevel::Ignore,
                SelectionProperty::CongestionControl        => PreferenceLevel::Require,
                SelectionProperty::Multipath                => PreferenceLevel::Prefer,
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
