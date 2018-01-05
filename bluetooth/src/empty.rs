/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use Bluetooth as BluetoothTrait;
use {Adapter, Characteristic, Descriptor, Device, DiscoverySession, Service};

const NOT_SUPPORTED_ERROR: &'static str = "Error! Not supported platform!";

pub struct EmptyAdapter {}
impl Adapter<Bluetooth> for EmptyAdapter {
    fn init() -> Result<EmptyAdapter, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_id(&self) -> String {
        String::new()
    }
    fn get_devices(&self) -> Result<Vec<EmptyDevice>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_device(&self, _address: String) -> Result<Option<EmptyDevice>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_address(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_name(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_powered(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_pairable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_discovering(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn create_discovery_session(&self) -> Result<EmptyDiscoverySession, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
pub struct EmptyDiscoverySession {}
impl DiscoverySession<Bluetooth> for EmptyDiscoverySession {
    fn create_session(_adapter: Arc<EmptyAdapter>) -> Result<EmptyDiscoverySession, Box<Error>> {
        Ok(EmptyDiscoverySession {})
    }

    fn start_discovery(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    fn stop_discovery(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
pub struct EmptyDevice {}
impl Device<Bluetooth> for EmptyDevice {
    fn create_device(_adapter: EmptyAdapter, _device: String) -> EmptyDevice {
        EmptyDevice {}
    }
    fn get_id(&self) -> String {
        String::new()
    }
    fn get_address(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_name(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_icon(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_appearance(&self) -> Result<u16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_paired(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_connected(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_trusted(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_blocked(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_rssi(&self) -> Result<i16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_manufacturer_data(&self) -> Result<HashMap<u16, Vec<u8>>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_service_data(&self) -> Result<HashMap<String, Vec<u8>>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_gatt_services(&self) -> Result<Vec<EmptyService>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn connect(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn disconnect(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn pair(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
pub struct EmptyService {}
impl Service<Bluetooth> for EmptyService {
    fn create_service(_device: EmptyDevice, _service: String) -> EmptyService {
        EmptyService {}
    }
    fn get_id(&self) -> String {
        String::new()
    }
    fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_primary(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_includes(&self, _device: EmptyDevice) -> Result<Vec<EmptyService>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_gatt_characteristics(&self) -> Result<Vec<EmptyCharacteristic>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
pub struct EmptyCharacteristic {}
impl Characteristic<Bluetooth> for EmptyCharacteristic {
    fn create_characteristic(
        _service: EmptyService,
        _characteristic: String,
    ) -> EmptyCharacteristic {
        EmptyCharacteristic {}
    }
    fn get_id(&self) -> String {
        String::new()
    }
    fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn is_notifying(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_gatt_descriptors(&self) -> Result<Vec<EmptyDescriptor>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn start_notify(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn stop_notify(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
pub struct EmptyDescriptor {}
impl Descriptor<Bluetooth> for EmptyDescriptor {
    fn create_descriptor(
        _characteristic: EmptyCharacteristic,
        _descriptor: String,
    ) -> EmptyDescriptor {
        EmptyDescriptor {}
    }
    fn get_id(&self) -> String {
        String::new()
    }
    fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
    fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

enum Bluetooth {}
impl BluetoothTrait for Bluetooth {
    type Adapter = EmptyAdapter;
    type DiscoverySession = EmptyDiscoverySession;
    type Device = EmptyDevice;
    type GATTService = EmptyService;
    type GATTCharacteristic = EmptyCharacteristic;
    type GATTDescriptor = EmptyDescriptor;
}
