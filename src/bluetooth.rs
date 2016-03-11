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


#[derive(Clone, Debug)]
pub struct BluetoothAdapter {
    #[cfg(target_os = "linux")]
    adapter: Option<BluetoothAdapterBluez>,
    initialized: bool,
    devices: Vec<BluetoothDevice>,
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
    #[cfg(target_os = "linux")]
    pub fn init() -> BluetoothAdapter {
        let mut adapter = BluetoothAdapter::new();
        let bluez_adapter = match BluetoothAdapterBluez::init() {
            Ok(a) => Some(a),
            Err(_) => None,
        };
        adapter.set_adapter(bluez_adapter);
        adapter
    }

    pub fn create_adapter(object_path: String) -> BluetoothAdapter {
        let mut adapter = BluetoothAdapter::new();
        let bluez_adapter = match BluetoothAdapterBluez::create_adapter(object_path) {
            Ok(a) => Some(a),
            Err(_) => None,
        };
        adapter.set_adapter(bluez_adapter);
        adapter
    }

    fn new() -> BluetoothAdapter {
        BluetoothAdapter {
            adapter: None,
            initialized: false,
            devices: Vec::new(),
        }
    }

    pub fn get_object_path(&self) -> String {
        self.get_adapter().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn set_adapter(&mut self, adapter: Option<BluetoothAdapterBluez>) {
        match adapter {
            Some(a) => self.adapter = Some(a),
            None => return,
        }

        self.initialized = true;

        self.update_device_list();
    }

    #[cfg(target_os = "linux")]
    fn get_adapter(&self) -> BluetoothAdapterBluez {
        self.adapter.clone().unwrap()
    }

    #[cfg(target_os = "linux")]
    fn add_device(&mut self, object_path: String) {
        self.devices.push(
            BluetoothDevice::new(BluetoothDeviceBluez::create_device(object_path)))
    }

    fn update_device_list(&mut self) {
        self.devices.clear();
        let devices = self.get_adapter().get_device_list().unwrap_or(vec!());
        for device in devices {
            self.add_device(device.clone());
        }
    }

    pub fn get_devices(&mut self, update_list: bool) -> Vec<BluetoothDevice>{
        if update_list {
            self.update_device_list();
        }
        self.devices.clone()
    }

    pub fn get_device(&mut self, address: String, update_list: bool) -> Option<BluetoothDevice> {
        let devices = self.get_devices(update_list);
        for device in devices {
            if device.get_address() == address {
                return Some(device.clone());
            }
        }
        None
    }

    pub fn get_address(&self) -> String {
        self.get_adapter().get_address().unwrap_or(String::new())
    }

    pub fn get_name(&self) -> String {
        self.get_adapter().get_name().unwrap_or(String::new())
    }

    pub fn get_alias(&self) -> String {
        self.get_adapter().get_alias().unwrap_or(String::new())
    }

    pub fn set_alias(&self, value: String) {
        self.get_adapter().set_alias(value).ok();
    }

    pub fn get_class(&self) -> u32 {
        self.get_adapter().get_class().unwrap_or(0)
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_powered(&self) -> bool {
        self.get_adapter().is_powered().unwrap_or(false)
    }

    pub fn set_powered(&self, value: bool) {
        self.get_adapter().set_powered(value).ok();
    }

    pub fn is_discoverable(&self) -> bool {
        self.get_adapter().is_discoverable().unwrap_or(false)
    }

    pub fn set_discoverable(&self, value: bool) {
        self.get_adapter().set_discoverable(value).ok();
    }

    pub fn is_pairable(&self) -> bool {
        self.get_adapter().is_pairable().unwrap_or(false)
    }

    pub fn set_pairable(&self, value: bool) {
        self.get_adapter().set_pairable(value).ok();
    }

    pub fn get_pairable_timeout(&self) -> u32 {
        self.get_adapter().get_pairable_timeout().unwrap_or(0)
    }

    pub fn set_pairable_timeout(&self, value: u32) {
        self.get_adapter().set_pairable_timeout(value).ok();
    }

    pub fn get_discoverable_timeout(&self) -> u32 {
        self.get_adapter().get_discoverable_timeout().unwrap_or(0)
    }

    pub fn set_discoverable_timeout(&self, value: u32) {
        self.get_adapter().set_discoverable_timeout(value).ok();
    }

    pub fn is_discovering(&self) -> bool {
        self.get_adapter().is_discovering().unwrap_or(false)
    }

    pub fn get_uuids(&self) -> Vec<String> {
        self.get_adapter().get_uuids().unwrap_or(vec!())
    }

    pub fn get_vendor_id_source(&self) -> String {
        self.get_adapter().get_vendor_id_source().unwrap_or(String::new())
    }

    pub fn get_vendor_id(&self) -> u32 {
        self.get_adapter().get_vendor_id().unwrap_or(0)
    }

    pub fn get_product_id(&self) -> u32 {
        self.get_adapter().get_product_id().unwrap_or(0)
    }

    pub fn get_device_id(&self) -> u32 {
        self.get_adapter().get_device_id().unwrap_or(0)
    }

    pub fn get_modalias(&self) -> (String, u32, u32, u32) {
        self.get_adapter().get_modalias().unwrap_or((String::new(),0,0,0))
    }
}

impl BluetoothDevice {
    #[cfg(target_os = "linux")]
    pub fn new(device: BluetoothDeviceBluez) -> BluetoothDevice {
        BluetoothDevice {
            device: device,
        }
    }

    pub fn get_object_path(&self) -> String {
        self.get_device().get_object_path()
    }

    #[cfg(target_os = "linux")]
    fn get_device(&self) -> BluetoothDeviceBluez {
        self.device.clone()
    }

    pub fn get_address(&self) -> String {
        self.get_device().get_address().unwrap_or(String::new())
    }

    pub fn get_name(&self) -> String {
        self.get_device().get_name().unwrap_or(String::new())
    }

    pub fn get_icon(&self) -> String {
        self.get_device().get_icon().unwrap_or(String::new())
    }

    pub fn get_class(&self) -> u32 {
        self.get_device().get_class().unwrap_or(0)
    }

    pub fn get_appearance(&self) -> u16 {
        self.get_device().get_appearance().unwrap_or(0)
    }

    pub fn get_uuids(&self) -> Vec<String> {
        self.get_device().get_uuids().unwrap_or(vec!())
    }

    pub fn is_paired(&self) -> bool {
        self.get_device().is_paired().unwrap_or(false)
    }

    pub fn is_connected(&self) -> bool {
        self.get_device().is_connected().unwrap_or(false)
    }

    pub fn is_trusted(&self) -> bool {
        self.get_device().is_trusted().unwrap_or(false)
    }

    pub fn is_blocked(&self) -> bool {
        self.get_device().is_blocked().unwrap_or(false)
    }

    pub fn get_alias(&self) -> String {
        self.get_device().get_alias().unwrap_or(String::new())
    }

    pub fn set_alias(&self, value: String) {
        self.get_device().set_alias(value).ok();
    }

    pub fn get_adapter(&self) -> String {
        self.get_device().get_adapter().unwrap_or(String::new())
    }

    pub fn is_legacy_pairing(&self) -> bool {
        self.get_device().is_legacy_pairing().unwrap_or(false)
    }

    pub fn get_vendor_id_source(&self) -> String {
        self.get_device().get_vendor_id_source().unwrap_or(String::new())
    }

    pub fn get_vendor_id(&self) -> u32 {
        self.get_device().get_vendor_id().unwrap_or(0)
    }

    pub fn get_product_id(&self) -> u32 {
        self.get_device().get_product_id().unwrap_or(0)
    }

    pub fn get_device_id(&self) -> u32 {
        self.get_device().get_device_id().unwrap_or(0)
    }

    pub fn get_modalias(&self) -> (String, u32, u32, u32) {
        self.get_device().get_modalias().unwrap_or((String::new(),0,0,0))
    }

    pub fn get_rssi(&self) -> i16 {
        self.get_device().get_rssi().unwrap_or(0)
    }

    pub fn get_tx_power(&self) -> i16 {
        self.get_device().get_tx_power().unwrap_or(0)
    }

    pub fn get_gatt_services(&self) -> Vec<BluetoothGATTService> {
        let services = self.get_device().get_gatt_services().unwrap_or(vec!());
        let mut v: Vec<BluetoothGATTService> = Vec::new();
        for service in services {
            v.push(BluetoothGATTService::create_service(service.clone()));
        }
        v
    }

    pub fn connect(&self) {
        self.get_device().connect().ok();
    }

    pub fn disconnect(&self) {
        self.get_device().disconnect().ok();
    }

    pub fn connect_profile(&self, uuid: String) {
        self.get_device().connect_profile(uuid).ok();
    }

    pub fn disconnect_profile(&self, uuid: String) {
        self.get_device().disconnect_profile(uuid).ok();
    }

    pub fn pair(&self) {
        self.get_device().pair().ok();
    }

    pub fn cancel_pairing(&self) {
        self.get_device().cancel_pairing().ok();
    }
}

impl BluetoothGATTService {
    #[cfg(target_os = "linux")]
    fn new(gatt_service: BluetoothGATTServiceBluez)
           -> BluetoothGATTService {
        BluetoothGATTService {
            gatt_service: gatt_service
        }
    }

    pub fn get_object_path(&self) -> String {
        self.get_gatt_service().get_object_path()
    }

    #[cfg(target_os = "linux")]
    pub fn create_service(service: String) -> BluetoothGATTService {
        BluetoothGATTService::new(
            BluetoothGATTServiceBluez::new(service.clone()))
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_service(&self) -> BluetoothGATTServiceBluez {
        self.gatt_service.clone()
    }

    pub fn get_uuid(&self) -> String {
        self.get_gatt_service().get_uuid().unwrap_or(String::new())
    }

    pub fn is_primary(&self) -> bool {
        self.get_gatt_service().is_primary().unwrap_or(false)
    }

    pub fn get_device(&self) -> String {
        self.get_gatt_service().get_device().unwrap_or(String::new())
    }

    pub fn get_characteristics(&self) -> Vec<BluetoothGATTCharacteristic> {
        let characteristics = self.get_gatt_service().get_characteristics().unwrap_or(vec!());
        let mut v: Vec<BluetoothGATTCharacteristic> = Vec::new();
        for characteristic in characteristics {
            v.push(BluetoothGATTCharacteristic::create_characteristic(characteristic.clone()));
        }
        v
    }
}

impl BluetoothGATTCharacteristic {
    #[cfg(target_os = "linux")]
    fn new(gatt_characteristic: BluetoothGATTCharacteristicBluez)
           -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            gatt_characteristic: gatt_characteristic
        }
    }

    pub fn get_object_path(&self) -> String {
        self.get_gatt_characteristic().get_object_path()
    }

    #[cfg(target_os = "linux")]
    pub fn create_characteristic(characteristic: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic::new(
            BluetoothGATTCharacteristicBluez::new(characteristic.clone()))
    }

    #[cfg(target_os = "linux")]
    fn get_gatt_characteristic(&self) -> BluetoothGATTCharacteristicBluez {
        self.gatt_characteristic.clone()
    }

    pub fn get_uuid(&self) -> String {
        self.get_gatt_characteristic().get_uuid().unwrap_or(String::new())
    }

    pub fn get_service(&self) -> String {
        self.get_gatt_characteristic().get_service().unwrap_or(String::new())
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.get_gatt_characteristic().get_value().unwrap_or(vec!())
    }

    pub fn is_notifying(&self) -> bool {
        self.get_gatt_characteristic().is_notifying().unwrap_or(false)
    }

    pub fn get_flags(&self) -> Vec<String> {
        self.get_gatt_characteristic().get_flags().unwrap_or(vec!())
    }

    pub fn get_descriptors(&self) -> Vec<String> {
        self.get_gatt_characteristic().get_descriptors().unwrap_or(vec!())
    }

    pub fn read_value(&self) -> Vec<u8> {
        self.get_gatt_characteristic().read_value().unwrap_or(vec!())
    }

    pub fn write_value(&self, values: Vec<u8>) {
        self.get_gatt_characteristic().write_value(values).ok();
    }

    pub fn start_notify(&self) {
        self.get_gatt_characteristic().start_notify().ok();
    }

    pub fn stop_notify(&self) {
        self.get_gatt_characteristic().stop_notify().ok();
    }
}

impl BluetoothGATTDescriptor {
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
    fn get_gatt_descriptor(&self) -> BluetoothGATTDescriptorBluez {
        self.gatt_descriptor.clone()
    }

    pub fn get_uuid(&self) -> String {
        self.get_gatt_descriptor().get_uuid().unwrap_or(String::new())
    }

    pub fn get_characteristic(&self) -> String {
        self.get_gatt_descriptor().get_characteristic().unwrap_or(String::new())
    }

    pub fn get_value(&self) -> Vec<u8> {
        self.get_gatt_descriptor().get_value().unwrap_or(vec!())
    }

    pub fn get_flags(&self) -> Vec<String> {
        self.get_gatt_descriptor().get_flags().unwrap_or(vec!())
    }

    pub fn read_value(&self) -> Vec<u8> {
        self.get_gatt_descriptor().read_value().unwrap_or(vec!())
    }

    pub fn write_value(&self, values: Vec<u8>) {
        self.get_gatt_descriptor().write_value(values).ok();
    }
}
