/* This Source Code Form is subject to the terms of the Mozilla Public
 *  * License, v. 2.0. If a copy of the MPL was not distributed with this
 *   * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::error::Error;

const NOT_SUPPORTED_ON_REAL_ERROR: &'static str = "Error! Test functions are not supported on real devices!";

pub trait BluetoothAdapter<B: Bluetooth> {
    fn init() -> Result<B::Adapter, Box<Error>>;
    fn init_mock() -> Result<B::Adapter, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_id(&self) -> String;
    fn set_id(&self, id: String) {
        ()
    }
    fn get_devices(&self) -> Result<Vec<B::Device>, Box<Error>>;
    fn get_device(&self, address: String) -> Result<Option<B::Device>, Box<Error>>;
    fn get_address(&self) -> Result<String, Box<Error>>;
    fn set_address(&self, address: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_name(&self) -> Result<String, Box<Error>>;
    fn set_name(&self, name: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_alias(&self) -> Result<String, Box<Error>>;
    fn set_alias(&self, alias: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_class(&self) -> Result<u32, Box<Error>>;
    fn set_class(&self, class: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_powered(&self) -> Result<bool, Box<Error>>;
    fn set_powered(&self, powered: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_present(&self) -> Result<bool, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_present(&self, present: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_discoverable(&self) -> Result<bool, Box<Error>>;
    fn set_discoverable(&self, discoverable: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_pairable(&self) -> Result<bool, Box<Error>>;
    fn set_pairable(&self, pairable: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_pairable_timeout(&self) -> Result<u32, Box<Error>>;
    fn set_pairable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_discoverable_timeout(&self) -> Result<u32, Box<Error>>;
    fn set_discoverable_timeout(&self, timeout: u32) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn is_discovering(&self) -> Result<bool, Box<Error>>;
    fn set_discovering(&self, discovering: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_can_start_discovery(&self, can_start_discovery: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_can_stop_discovery(&self, can_stop_discovery: bool) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn create_discovery_session(&self) -> Result<B::DiscoverySession, Box<Error>>;
    fn get_uuids(&self) -> Result<Vec<String>, Box<Error>>;
    fn set_uuids(&self, uuids: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_vendor_id_source(&self) -> Result<String, Box<Error>>;
    fn get_vendor_id(&self) -> Result<u32, Box<Error>>;
    fn get_product_id(&self) -> Result<u32, Box<Error>>;
    fn get_device_id(&self) -> Result<u32, Box<Error>>;
    fn get_modalias(&self) -> Result<(String, u32, u32, u32), Box<Error>>;
    fn set_modalias(&self, modalias: String) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn get_ad_datas(&self) -> Result<Vec<String>, Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
    fn set_ad_datas(&self, ad_datas: Vec<String>) -> Result<(), Box<Error>> {
        Err(Box::from(NOT_SUPPORTED_ON_REAL_ERROR))
    }
}

pub trait BluetoothDiscoverySession<B: Bluetooth> {}
pub trait BluetoothDevice<B: Bluetooth> {}
pub trait GATTService<B: Bluetooth> {}
pub trait GATTCaharcteristic<B: Bluetooth> {}
pub trait GATTDescriptor<B: Bluetooth> {}

pub trait Bluetooth: Sized {
    type Adapter: BluetoothAdapter<Self>;
    type DiscoverySession: BluetoothDiscoverySession<Self>;
    type Device: BluetoothDevice<Self>;
    type GATTService: GATTService<Self>;
    type GATTCaharcteristic: GATTCaharcteristic<Self>;
    type GATTDescriptor: GATTDescriptor<Self>;
}
