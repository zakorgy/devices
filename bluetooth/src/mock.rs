/* This Source Code Form is subject to the terms of the Mozilla Public
 *  * License, v. 2.0. If a copy of the MPL was not distributed with this
 *   * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use blurmock::fake_adapter::FakeBluetoothAdapter;
use blurmock::fake_device::FakeBluetoothDevice;
use blurmock::fake_characteristic::FakeBluetoothGATTCharacteristic;
use blurmock::fake_descriptor::FakeBluetoothGATTDescriptor;
use blurmock::fake_service::FakeBluetoothGATTService;
use blurmock::fake_discovery_session::FakeBluetoothDiscoverySession;

use std::collections::HashMap;
use std::sync::Arc;
use std::error::Error;
use Bluetooth as BluetoothTrait;
use {Adapter, Characteristic, Descriptor, Device, DiscoverySession, Service};

impl Adapter<Bluetooth> for Arc<FakeBluetoothAdapter> {}
impl Device<Bluetooth> for Arc<FakeBluetoothDevice> {}
impl DiscoverySession<Bluetooth> for Arc<FakeBluetoothDiscoverySession> {}
impl Service<Bluetooth> for Arc<FakeBluetoothGATTService> {}
impl Characteristic<Bluetooth> for Arc<FakeBluetoothGATTCharacteristic> {}
impl Descriptor<Bluetooth> for Arc<FakeBluetoothGATTDescriptor> {
    fn create_descriptor(
        characteristic: Arc<FakeBluetoothGATTCharacteristic>,
        descriptor: String,
    ) -> Arc<FakeBluetoothGATTDescriptor> {
        FakeBluetoothGATTDescriptor::new_empty(characteristic, descriptor)
    }
    fn get_id(&self) -> String {
        FakeBluetoothGATTDescriptor::get_id(self)
    }
    fn set_id(&self, id: String) {
        FakeBluetoothGATTDescriptor::set_id(self, id)
    }
    fn get_uuid(&self) -> Result<String, Box<Error>> {
        FakeBluetoothGATTDescriptor::get_uuid(self)
    }
    fn set_uuid(&self, uuid: String) -> Result<(), Box<Error>> {
        FakeBluetoothGATTDescriptor::set_uuid(self, uuid)
    }
    fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        FakeBluetoothGATTDescriptor::get_value(self)
    }
    fn set_value(&self, value: Vec<u8>) -> Result<(), Box<Error>> {
        FakeBluetoothGATTDescriptor::set_value(self, Some(value))
    }
    fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        FakeBluetoothGATTDescriptor::get_flags(self)
    }
    fn set_flags(&self, flags: Vec<String>) -> Result<(), Box<Error>> {
        FakeBluetoothGATTDescriptor::set_flags(self, flags)
    }
    fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        FakeBluetoothGATTDescriptor::read_value(self)
    }
    fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        FakeBluetoothGATTDescriptor::write_value(self, values)
    }
}

enum Bluetooth {}
impl BluetoothTrait for Bluetooth {
    type Adapter = Arc<FakeBluetoothAdapter>;
    type DiscoverySession = Arc<FakeBluetoothDiscoverySession>;
    type Device = Arc<FakeBluetoothDevice>;
    type GATTService = Arc<FakeBluetoothGATTService>;
    type GATTCharacteristic = Arc<FakeBluetoothGATTCharacteristic>;
    type GATTDescriptor = Arc<FakeBluetoothGATTDescriptor>;
}