/* This Source Code Form is subject to the terms of the Mozilla Public
 *  * License, v. 2.0. If a copy of the MPL was not distributed with this
 *   * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

const NOT_SUPPORTED_ON_REAL_ERROR: &'static str =
    "Error! Test functions are not supported on real devices!";

pub trait Adapter<B: Bluetooth> {
    /*fn init() -> Result<B::Adapter, Box<Error>>;
    fn get_id(&self) -> String;
    fn set_id(&self, _id: String) {
        ()
    }
    fn get_devices(&self) -> Result<Vec<B::Device>, Box<Error>>;
    fn get_device(&self, _address: String) -> Result<Option<B::Device>, Box<Error>>;
    fn get_address(&self) -> Result<String, Box<Error>>;
    fn set_address(&self, _address: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_name(&self) -> Result<String, Box<Error>>;
    fn set_name(&self, _name: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_alias(&self) -> Result<String, Box<Error>>;
    fn set_alias(&self, _alias: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_class(&self) -> Result<u32, Box<Error>>;
    fn set_class(&self, _class: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_powered(&self) -> Result<bool, Box<Error>>;
    fn set_powered(&self, _powered: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_present(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_present(&self, _present: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>>;
    fn set_discoverable(&self, _discoverable: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_pairable(&self) -> Result<bool, Box<Error>>;
    fn set_pairable(&self, _pairable: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>>;
    fn set_pairable_timeout(&self, _timeout: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>>;
    fn set_discoverable_timeout(&self, _timeout: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_discovering(&self) -> Result<bool, Box<Error>>;
    fn set_discovering(&self, _discovering: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_can_start_discovery(&self, _can_start_discovery: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_can_stop_discovery(&self, _can_stop_discovery: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn create_discovery_session(&self) -> Result<B::DiscoverySession, Box<Error>>;
    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>>;
    fn set_uuids(&self, _uuids: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_vendor_id_source(&self) -> Result<String, Box<Error>>;
    fn get_vendor_id(&self) -> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>>;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;
    fn set_modalias(&self, _modalias: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_ad_datas(&self, _ad_datas: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }*/
}

pub trait DiscoverySession<B: Bluetooth> {
    /*fn create_session(_adapter: Arc<B::Adapter>) -> Result<B::DiscoverySession, Box<Error>>;
    fn start_discovery(&self) -> Result<(), Box<Error>>;
    fn stop_discovery(&self) -> Result<(), Box<Error>>;*/
}

pub trait Device<B: Bluetooth> {
    /*fn create_device(_adapter: B::Adapter, _device: String) -> B::Device;
    fn get_id(&self) -> String;
    fn set_id(&self, _id: String) {
        ()
    }
    fn get_address(&self) -> Result<String, Box<Error>>;
    fn set_address(&self, _address: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_name(&self) -> Result<String, Box<Error>>;
    fn set_name(&self, _name: Option<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_icon(&self) -> Result<String, Box<Error>>;
    fn set_icon(&self, _icon: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_class(&self) -> Result<u32, Box<Error>>;
    fn set_class(&self, _class: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_appearance(&self) -> Result<u16, Box<Error>>;
    fn set_appearance(&self, _appearance: u16) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>>;
    fn set_uuids(&self, _uuids: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_paired(&self) -> Result<bool, Box<Error>>;
    fn set_paired(&self, _paired: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_connected(&self) -> Result<bool, Box<Error>>;
    fn set_connected(&self, _connected: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_connectable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_connectable(&self, _connectable: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_trusted(&self) -> Result<bool, Box<Error>>;
    fn set_trusted(&self, _trusted: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_blocked(&self) -> Result<bool, Box<Error>>;
    fn set_blocked(&self, _blocked: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_alias(&self) -> Result<String, Box<Error>>;
    fn set_alias(&self, _alias: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_legacy_pairing(&self) -> Result<bool, Box<Error>>;
    fn set_legacy_pairing(&self, _legacy_pairing: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_vendor_id_source(&self) -> Result<String, Box<Error>>;
    fn get_vendor_id(&self) -> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>>;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;
    fn set_modalias(&self, _modalias: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_rssi(&self) -> Result<i16, Box<Error>>;
    fn set_rssi(&self, _rssi: i16) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_tx_power(&self) -> Result<i16, Box<Error>>;
    fn set_tx_power(&self, _tx_power: i16) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_manufacturer_data(&self) -> Result<HashMap<u16, Vec<u8>>, Box<Error>>;
    fn set_manufacturer_data(
        &self,
        _manufacturer_data: HashMap<u16, Vec<u8>>,
    ) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_service_data(&self) -> Result<HashMap<String, Vec<u8>>, Box<Error>>;
    fn set_service_data(&self, _service_data: HashMap<String, Vec<u8>>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_gatt_services(&self) -> Result<Vec<B::GATTService>, Box<Error>>;
    fn connect(&self) -> Result<(), Box<Error>>;
    fn disconnect(&self) -> Result<(), Box<Error>>;
    fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>>;
    fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>>;
    fn pair(&self) -> Result<(), Box<Error>>;
    fn cancel_pairing(&self) -> Result<(), Box<Error>>;*/
}
pub trait Service<B: Bluetooth> {
    /*fn create_service(_device: B::Device, _service: String) -> B::GATTService;
    fn get_id(&self) -> String;
    fn set_id(&self, _id: String) {
        ()
    }
    fn get_uuid(&self) -> Result<String, Box<Error>>;
    fn set_uuid(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_primary(&self) -> Result<bool, Box<Error>>;
    fn set_primary(&self, _primary: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_includes(&self, _device: B::Device) -> Result<Vec<B::GATTService>, Box<Error>>;
    fn get_gatt_characteristics(&self) -> Result<Vec<B::GATTCharacteristic>, Box<Error>>;*/
}
pub trait Characteristic<B: Bluetooth> {
    /*fn create_characteristic(
        _service: B::GATTService,
        _characteristic: String,
    ) -> B::GATTCharacteristic;
    fn get_id(&self) -> String;
    fn set_id(&self, _id: String) {
        ()
    }
    fn get_uuid(&self) -> Result<String, Box<Error>>;
    fn set_uuid(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_value(&self) -> Result<Vec<u8>, Box<Error>>;
    fn set_value(&self, _value: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_notifying(&self) -> Result<bool, Box<Error>>;
    fn set_notifying(&self, _notifying: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_flags(&self) -> Result<Vec<String>, Box<Error>>;
    fn set_flags(&self, _flags: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_gatt_descriptors(&self) -> Result<Vec<B::GATTDescriptor>, Box<Error>>;
    fn read_value(&self) -> Result<Vec<u8>, Box<Error>>;
    fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>>;
    fn start_notify(&self) -> Result<(), Box<Error>>;
    fn stop_notify(&self) -> Result<(), Box<Error>>;*/
}
pub trait Descriptor<B: Bluetooth> {
    fn create_descriptor(
        _characteristic: B::GATTCharacteristic,
        _descriptor: String,
    ) -> B::GATTDescriptor;
    fn get_id(&self) -> String;
    fn set_id(&self, _id: String) {
        ()
    }
    fn get_uuid(&self) -> Result<String, Box<Error>>;
    fn set_uuid(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_value(&self) -> Result<Vec<u8>, Box<Error>>;
    fn set_value(&self, _value: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_flags(&self) -> Result<Vec<String>, Box<Error>>;
    fn set_flags(&self, _flags: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn read_value(&self) -> Result<Vec<u8>, Box<Error>>;
    fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>>;
}

pub trait Bluetooth: Sized + Send + Sync {
    type Adapter: Adapter<Self>;
    type DiscoverySession: DiscoverySession<Self>;
    type Device: Device<Self>;
    type GATTService: Service<Self>;
    type GATTCharacteristic: Characteristic<Self>;
    type GATTDescriptor: Descriptor<Self>;
}

#[cfg(feature = "mock")]
extern crate blurmock;

//mod empty;
#[cfg(feature = "mock")]
mod mock;

/*#[cfg(target_os = "windows")]
pub use empty::{EmptyAdapter as BluetoothAdpater,
                EmptyCharacteristic as BluetoothGATTCharacteristic,
                EmptyDescriptor as BluetoothGATTDescriptor, EmptyDevice as BluetoothDevice,
                EmptyDiscoverySession as BluetoothDiscoverySession,
                EmptyService as BluetoothGATTService};*/
