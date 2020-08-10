// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{HapService, accessory_information::AccessoryInformationService, contact_sensor::ContactSensorService},
	HapType,
	Result,
};

/// Contact Sensor Accessory.
#[derive(Debug, Default)]
pub struct ContactSensorAccessory {
    /// ID of the Contact Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Contact Sensor Service.
    pub contact_sensor: ContactSensorService,
}

impl ContactSensorAccessory {
    /// Creates a new Contact Sensor Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let contact_sensor_id = accessory_information.get_characteristics().len() as u64;
        let mut contact_sensor = ContactSensorService::new(1 + contact_sensor_id + 1, id);
        contact_sensor.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            contact_sensor,
        })
    }
}

impl HapAccessory for ContactSensorAccessory {
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
            &self.contact_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.contact_sensor,
        ]
    }
}

impl Serialize for ContactSensorAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
