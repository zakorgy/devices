/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(target_os = "linux")]
use blurz::bluetooth_adapter::BluetoothAdapter as BluetoothAdapterBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_device::BluetoothDevice as BluetoothDeviceBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as BluetoothGATTCharacteristicBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor as BluetoothGATTDescriptorBluez;
#[cfg(target_os = "linux")]
use blurz::bluetooth_gatt_service::BluetoothGATTService as BluetoothGATTServiceBluez;

use std::error::Error;

#[cfg(not(target_os = "linux"))]
const NOT_SUPPORTED_ERROR: &'static str = "Error! Not supported platform!";

#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    #[cfg(target_os = "linux")]
    adapter: BluetoothAdapterBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothDevice {
    #[cfg(target_os = "linux")]
    device: BluetoothDeviceBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTService {
    #[cfg(target_os = "linux")]
    gatt_service: BluetoothGATTServiceBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTCharacteristic {
    #[cfg(target_os = "linux")]
    gatt_characteristic: BluetoothGATTCharacteristicBluez,
}

#[derive(Clone, Debug)]
pub struct BluetoothGATTDescriptor {
    #[cfg(target_os = "linux")]
    gatt_descriptor: BluetoothGATTDescriptorBluez,
}

impl BluetoothAdapter {

    // Linux

    #[cfg(target_os = "linux")]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::init());
        Ok(BluetoothAdapter::new(bluez_adapter))
    }

    #[cfg(target_os = "linux")]
    pub fn create_adapter(object_path: String) -> Result<BluetoothAdapter, Box<Error>> {
        let bluez_adapter = try!(BluetoothAdapterBluez::create_adapter(object_path));
        Ok(BluetoothAdapter::new(bluez_adapter))
    }

    #[cfg(target_os = "linux")]
    fn new(adapter: BluetoothAdapterBluez) -> BluetoothAdapter {
        BluetoothAdapter {
            adapter: adapter,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_adapter().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_adapter(&self) -> BluetoothAdapterBluez {
        self.adapter.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_devices(&self) -> Result<Vec<BluetoothDevice>, Box<Error>> {
        let device_list = try!(self.get_adapter().get_device_list());
        Ok(device_list.into_iter().map(|device| BluetoothDevice::create_device(device)).collect())
    }

    #[cfg(target_os = "linux")]
    pub fn get_device(&self, address: String) -> Result<Option<BluetoothDevice>, Box<Error>> {
        let devices = try!(self.get_devices());
        for device in devices {
            if try!(device.get_address()) == address {
                return Ok(Some(device));
            }
        }
        Ok(None)
    }

    #[cfg(target_os = "linux")]
    pub fn get_address(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_address()
    }

    #[cfg(target_os = "linux")]
    pub fn get_name(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_name()
    }

    #[cfg(target_os = "linux")]
    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_alias()
    }

    #[cfg(target_os = "linux")]
    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        self.get_adapter().set_alias(value)
    }

    #[cfg(target_os = "linux")]
    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_class()
    }

    #[cfg(target_os = "linux")]
    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_powered()
    }

    #[cfg(target_os = "linux")]
    pub fn set_powered(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_powered(value)
    }

    #[cfg(target_os = "linux")]
    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_discoverable()
    }

    #[cfg(target_os = "linux")]
    pub fn set_discoverable(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_discoverable(value)
    }

    #[cfg(target_os = "linux")]
    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_pairable()
    }

    #[cfg(target_os = "linux")]
    pub fn set_pairable(&self, value: bool) -> Result<(), Box<Error>> {
        self.get_adapter().set_pairable(value)
    }

    #[cfg(target_os = "linux")]
    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_pairable_timeout()
    }

    #[cfg(target_os = "linux")]
    pub fn set_pairable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        self.get_adapter().set_pairable_timeout(value)
    }

    #[cfg(target_os = "linux")]
    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_discoverable_timeout()
    }

    #[cfg(target_os = "linux")]
    pub fn set_discoverable_timeout(&self, value: u32) -> Result<(), Box<Error>> {
        self.get_adapter().set_discoverable_timeout(value)
    }

    #[cfg(target_os = "linux")]
    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        self.get_adapter().is_discovering()
    }

    #[cfg(target_os = "linux")]
    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_adapter().get_uuids()
    }

    #[cfg(target_os = "linux")]
    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.get_adapter().get_vendor_id_source()
    }

    #[cfg(target_os = "linux")]
    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_vendor_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_product_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.get_adapter().get_device_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.get_adapter().get_modalias()
    }

    #[cfg(target_os = "linux")]
    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        self.get_adapter().start_discovery()
    }

    #[cfg(target_os = "linux")]
    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        self.get_adapter().stop_discovery()
    }

    // Unsupported

    #[cfg(not(target_os = "linux"))]
    pub fn init() -> Result<BluetoothAdapter, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn create_adapter(_object_path: String) -> Result<BluetoothAdapter, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_object_path(&self) -> String {
        String::new()
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_alias(&self, _value: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_powered(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_powered(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_discoverable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_discoverable(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_pairable(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_pairable(&self, _value: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_pairable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_pairable_timeout(&self, _value: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_discoverable_timeout(&self, _value: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_discovering(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn start_discovery(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn stop_discovery(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

}

impl BluetoothDevice {

    // Linux

    #[cfg(target_os = "linux")]
    fn new(device: BluetoothDeviceBluez) -> BluetoothDevice {
        BluetoothDevice {
            device: device,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn create_device(device: String) -> BluetoothDevice {
        BluetoothDevice::new(
            BluetoothDeviceBluez::new(device.clone()))
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_device().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_device(&self) -> BluetoothDeviceBluez {
        self.device.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_address(&self) -> Result<String, Box<Error>> {
        self.get_device().get_address()
    }

    #[cfg(target_os = "linux")]
    pub fn get_name(&self) -> Result<String, Box<Error>> {
        self.get_device().get_name()
    }

    #[cfg(target_os = "linux")]
    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        self.get_device().get_icon()
    }

    #[cfg(target_os = "linux")]
    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_class()
    }

    #[cfg(target_os = "linux")]
    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        self.get_device().get_appearance()
    }

    #[cfg(target_os = "linux")]
    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_device().get_uuids()
    }

    #[cfg(target_os = "linux")]
    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_paired()
    }

    #[cfg(target_os = "linux")]
    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_connected()
    }

    #[cfg(target_os = "linux")]
    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_trusted()
    }

    #[cfg(target_os = "linux")]
    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_blocked()
    }

    #[cfg(target_os = "linux")]
    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        self.get_device().get_alias()
    }

    #[cfg(target_os = "linux")]
    pub fn set_alias(&self, value: String) -> Result<(), Box<Error>> {
        self.get_device().set_alias(value)
    }

    #[cfg(target_os = "linux")]
    pub fn get_adapter(&self) -> Result<String, Box<Error>> {
        self.get_device().get_adapter()
    }

    #[cfg(target_os = "linux")]
    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        self.get_device().is_legacy_pairing()
    }

    #[cfg(target_os = "linux")]
    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        self.get_device().get_vendor_id_source()
    }

    #[cfg(target_os = "linux")]
    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_vendor_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_product_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        self.get_device().get_device_id()
    }

    #[cfg(target_os = "linux")]
    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        self.get_device().get_modalias()
    }

    #[cfg(target_os = "linux")]
    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        self.get_device().get_rssi()
    }

    #[cfg(target_os = "linux")]
    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        self.get_device().get_tx_power()
    }

    #[cfg(target_os = "linux")]
    pub fn get_gatt_services(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        let services = try!(self.get_device().get_gatt_services());
        Ok(services.into_iter().map(|service| BluetoothGATTService::create_service(service)).collect())
    }

    #[cfg(target_os = "linux")]
    pub fn connect(&self) -> Result<(), Box<Error>> {
        self.get_device().connect()
    }

    #[cfg(target_os = "linux")]
    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        self.get_device().disconnect()
    }

    #[cfg(target_os = "linux")]
    pub fn connect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        self.get_device().connect_profile(uuid)
    }

    #[cfg(target_os = "linux")]
    pub fn disconnect_profile(&self, uuid: String) -> Result<(), Box<Error>> {
        self.get_device().disconnect_profile(uuid)
    }

    #[cfg(target_os = "linux")]
    pub fn pair(&self) -> Result<(), Box<Error>> {
        self.get_device().pair()
    }

    #[cfg(target_os = "linux")]
    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        self.get_device().cancel_pairing()
    }

    // Unsupported

    #[cfg(not(target_os = "linux"))]
    pub fn create_device(_device: String) -> BluetoothDevice {
        BluetoothDevice { }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_object_path(&self) -> String {
        String::new()
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_address(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_name(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_icon(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_class(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_appearance(&self) -> Result<u16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_uuids(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_paired(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_connected(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_trusted(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_blocked(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_alias(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn set_alias(&self, _value: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_adapter(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_legacy_pairing(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_vendor_id_source(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_vendor_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_product_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_device_id(&self) -> Result<u32, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_rssi(&self) -> Result<i16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_tx_power(&self) -> Result<i16, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_gatt_services(&self) -> Result<Vec<BluetoothGATTService>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn connect(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn disconnect(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn connect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn disconnect_profile(&self, _uuid: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn pair(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn cancel_pairing(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

impl BluetoothGATTService {

    // Linux

    #[cfg(target_os = "linux")]
    fn new(gatt_service: BluetoothGATTServiceBluez)
           -> BluetoothGATTService {
        BluetoothGATTService {
            gatt_service: gatt_service
        }
    }

    #[cfg(target_os = "linux")]
    pub fn create_service(service: String) -> BluetoothGATTService {
        BluetoothGATTService::new(
            BluetoothGATTServiceBluez::new(service.clone()))
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_gatt_service().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_service(&self) -> BluetoothGATTServiceBluez {
        self.gatt_service.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        self.get_gatt_service().get_uuid()
    }

    #[cfg(target_os = "linux")]
    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        self.get_gatt_service().is_primary()
    }

    #[cfg(target_os = "linux")]
    pub fn get_device(&self) -> Result<String, Box<Error>> {
        self.get_gatt_service().get_device()
    }

    #[cfg(target_os = "linux")]
    pub fn get_gatt_characteristics(&self) -> Result<Vec<BluetoothGATTCharacteristic>, Box<Error>> {
        let characteristics = try!(self.get_gatt_service().get_gatt_characteristics());
        Ok(characteristics.into_iter().map(|characteristic| BluetoothGATTCharacteristic::create_characteristic(characteristic)).collect())
    }

    // Unsupported

    #[cfg(not(target_os = "linux"))]
    pub fn create_service(_service: String) -> BluetoothGATTService {
        BluetoothGATTService { }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_object_path(&self) -> String {
        String::new()
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_primary(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_device(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_gatt_characteristics(&self) -> Result<Vec<BluetoothGATTCharacteristic>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

impl BluetoothGATTCharacteristic {

    // Linux

    #[cfg(target_os = "linux")]
    fn new(gatt_characteristic: BluetoothGATTCharacteristicBluez)
           -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            gatt_characteristic: gatt_characteristic
        }
    }

    #[cfg(target_os = "linux")]
    pub fn create_characteristic(characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic::new(
            BluetoothGATTCharacteristicBluez::new(characteristic.clone()))
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_gatt_characteristic().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_characteristic(&self) -> BluetoothGATTCharacteristicBluez {
        self.gatt_characteristic.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        self.get_gatt_characteristic().get_uuid()
    }

    #[cfg(target_os = "linux")]
    pub fn get_service(&self) -> Result<String, Box<Error>> {
        self.get_gatt_characteristic().get_service()
    }

    #[cfg(target_os = "linux")]
    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_characteristic().get_value()
    }

    #[cfg(target_os = "linux")]
    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        self.get_gatt_characteristic().is_notifying()
    }

    #[cfg(target_os = "linux")]
    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_gatt_characteristic().get_flags()
    }

    #[cfg(target_os = "linux")]
    pub fn get_gatt_descriptors(&self) -> Result<Vec<BluetoothGATTDescriptor>, Box<Error>> {
        let descriptors =  try!(self.get_gatt_characteristic().get_gatt_descriptors());
        Ok(descriptors.into_iter().map(|descriptor| BluetoothGATTDescriptor::create_descriptor(descriptor)).collect())
    }

    #[cfg(target_os = "linux")]
    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_characteristic().read_value()
    }

    #[cfg(target_os = "linux")]
    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        self.get_gatt_characteristic().write_value(values)
    }

    #[cfg(target_os = "linux")]
    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        self.get_gatt_characteristic().start_notify()
    }

    #[cfg(target_os = "linux")]
    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        self.get_gatt_characteristic().stop_notify()
    }

    // Unsupported

    #[cfg(not(target_os = "linux"))]
    pub fn create_characteristic(_characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic { }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_object_path(&self) -> String {
        String::new()
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_service(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_gatt_descriptors(&self) -> Result<Vec<BluetoothGATTDescriptor>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}

impl BluetoothGATTDescriptor {

    // Linux

    #[cfg(target_os = "linux")]
    fn new(gatt_descriptor: BluetoothGATTDescriptorBluez)
           -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor {
            gatt_descriptor: gatt_descriptor
        }
    }

    #[cfg(target_os = "linux")]
    pub fn create_descriptor(descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor::new(
            BluetoothGATTDescriptorBluez::new(descriptor.clone()))
    }

    #[cfg(target_os = "linux")]
    pub fn get_object_path(&self) -> String {
        self.get_gatt_descriptor().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_descriptor(&self) -> BluetoothGATTDescriptorBluez {
        self.gatt_descriptor.clone()
    }

    #[cfg(target_os = "linux")]
    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        self.get_gatt_descriptor().get_uuid()
    }

    #[cfg(target_os = "linux")]
    pub fn get_characteristic(&self) -> Result<String, Box<Error>> {
        self.get_gatt_descriptor().get_characteristic()
    }

    #[cfg(target_os = "linux")]
    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_descriptor().get_value()
    }

    #[cfg(target_os = "linux")]
    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        self.get_gatt_descriptor().get_flags()
    }

    #[cfg(target_os = "linux")]
    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        self.get_gatt_descriptor().read_value()
    }

    #[cfg(target_os = "linux")]
    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        self.get_gatt_descriptor().write_value(values)
    }

    // Unsupported

    #[cfg(not(target_os = "linux"))]
    pub fn create_descriptor(_descriptor: String) -> BluetoothGATTDescriptor {
        BluetoothGATTDescriptor { }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_object_path(&self) -> String {
        String::new()
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_characteristic(&self) -> Result<String, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn write_value(&self, _values: Vec<u8>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ERROR))
    }
}
