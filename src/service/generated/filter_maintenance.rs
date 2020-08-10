// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		filter_change_indication::FilterChangeIndicationCharacteristic,
		filter_life_level::FilterLifeLevelCharacteristic,
		reset_filter_indication::ResetFilterIndicationCharacteristic,
		name::NameCharacteristic,
	},
    HapType,
};

/// Filter Maintenance Service.
#[derive(Debug, Default)]
pub struct FilterMaintenanceService {
    /// ID of the Filter Maintenance Service.
    id: u64,
    /// `HapType` of the Filter Maintenance Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Filter Change Indication Characteristic (required).
	pub filter_change_indication: FilterChangeIndicationCharacteristic,

	/// Filter Life Level Characteristic (optional).
	pub filter_life_level: Option<FilterLifeLevelCharacteristic>,
	/// Reset Filter Indication Characteristic (optional).
	pub reset_filter_indication: Option<ResetFilterIndicationCharacteristic>,
	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
}

impl FilterMaintenanceService {
    /// Creates a new Filter Maintenance Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::FilterMaintenance,
			filter_change_indication: FilterChangeIndicationCharacteristic::new(id + 1 + 0, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for FilterMaintenanceService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.filter_change_indication,
		];
		if let Some(c) = &self.filter_life_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.reset_filter_indication {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.filter_change_indication,
		];
		if let Some(c) = &mut self.filter_life_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.reset_filter_indication {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for FilterMaintenanceService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
