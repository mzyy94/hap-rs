// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{HapService, accessory_information::AccessoryInformationService, air_quality_sensor::AirQualitySensorService},
	HapType,
	Result,
};

/// Air Quality Sensor Accessory.
#[derive(Debug, Default)]
pub struct AirQualitySensorAccessory {
    /// ID of the Air Quality Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Air Quality Sensor Service.
    pub air_quality_sensor: AirQualitySensorService,
}

impl AirQualitySensorAccessory {
    /// Creates a new Air Quality Sensor Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let air_quality_sensor_id = accessory_information.get_characteristics().len() as u64;
        let mut air_quality_sensor = AirQualitySensorService::new(1 + air_quality_sensor_id + 1, id);
        air_quality_sensor.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            air_quality_sensor,
        })
    }
}

impl HapAccessory for AirQualitySensorAccessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![
            &self.accessory_information,
            &self.air_quality_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.air_quality_sensor,
        ]
    }
}

impl Serialize for AirQualitySensorAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
