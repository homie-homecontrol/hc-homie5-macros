# hc-homie5-proc

`hc-homie5-proc` is a helper crate for [`hc-homie5`](https://github.com/your-repo/hc-homie5) that provides procedural macros to simplify the creation of Homie-compliant devices in Rust.

## Overview

This crate offers a procedural macro, `homie_device`, which automatically adds the necessary fields and implements the `HomieDeviceCore` trait for structs representing Homie devices.

## Usage

You will most likely never use this create directly. It will be installed alongside `hc-homie5` and reexported from there.

### Example

```rust
use hc_homie5_proc::homie_device;
use hc_homie5::{HomieDeviceCore, HomieMQTTClient};
use homie5::{
    device_description::HomieDeviceDescription, Homie5DeviceProtocol, HomieDeviceStatus,
    HomieDomain, HomieID, DeviceRef,
};

#[homie_device]
pub struct MyDevice {
    some_field: String,
}
```

### What the Macro Does

Applying `#[homie_device]` to a struct:

- Ensures the struct has named fields.
- Adds the following fields automatically:
    - `device_ref: DeviceRef`
    - `status: HomieDeviceStatus`
    - `device_desc: HomieDeviceDescription`
    - `homie_proto: Homie5DeviceProtocol`
    - `homie_client: HomieMQTTClient`
- Implements the `HomieDeviceCore` trait for the struct with default implementations for required methods.

## Features

- Reduces boilerplate code for Homie-compliant devices.
- Ensures all required fields and trait implementations are correctly added.
- Compatible with `hc-homie5` for seamless MQTT-based home automation device creation.

## License

This project is licensed under the MIT License.
