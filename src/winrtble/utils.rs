// btleplug Source Code File
//
// Copyright 2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.
//
// Some portions of this file are taken and/or modified from Rumble
// (https://github.com/mwylde/rumble), using a dual MIT/Apache License under the
// following copyright:
//
// Copyright (c) 2014 The Rust Project Developers

use super::bindings;
use crate::{
    api::{BDAddr, CharPropFlags},
    Error, Result,
};
use bindings::windows::{
    devices::bluetooth::generic_attribute_profile::{
        GattCharacteristicProperties, GattCommunicationStatus,
    },
    storage::streams::{DataReader, IBuffer},
};
use std::str::FromStr;
use uuid::Uuid;
use windows::Guid;

pub fn to_error(status: GattCommunicationStatus) -> Result<()> {
    if status == GattCommunicationStatus::AccessDenied {
        Err(Error::PermissionDenied)
    } else if status == GattCommunicationStatus::Unreachable {
        Err(Error::NotConnected)
    } else if status == GattCommunicationStatus::Success {
        Ok(())
    } else if status == GattCommunicationStatus::ProtocolError {
        Err(Error::NotSupported("ProtocolError".to_string()))
    } else {
        Err(Error::Other(format!("Communication Error:")))
    }
}

pub fn to_uuid(uuid: &Guid) -> Uuid {
    let guid_s = format!("{:?}", uuid);
    Uuid::from_str(&guid_s).unwrap()
}

pub fn to_vec(buffer: &IBuffer) -> Vec<u8> {
    let reader = DataReader::from_buffer(buffer).unwrap();
    let len = reader.unconsumed_buffer_length().unwrap() as usize;
    let mut data = vec![0u8; len];
    reader.read_bytes(&mut data).unwrap();
    data
}

pub fn to_guid(uuid: &Uuid) -> Guid {
    let (data1, data2, data3, data4) = uuid.as_fields();
    Guid::from_values(data1, data2, data3, data4.to_owned())
}

pub fn to_char_props(_: &GattCharacteristicProperties) -> CharPropFlags {
    CharPropFlags::from_bits_truncate(0 as u8)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// }
