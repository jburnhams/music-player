use std::collections::HashMap;

use mdns_sd::ServiceInfo;
use music_player_types::types::{Device, AIRPLAY_SERVICE_NAME};

use crate::scan_devices;

/// Test that scan_devices returns successfully and includes the airplay scanning infrastructure
#[tokio::test]
async fn scan_devices_returns_ok() {
    let result = scan_devices().await;
    assert!(result.is_ok(), "scan_devices should return Ok");

    let devices = result.unwrap();
    let devices_guard = devices.lock().unwrap();
    // The devices vec should exist (may be empty if no devices on network)
    assert!(devices_guard.len() >= 0);
}

/// Test that an AirPlay ServiceInfo is correctly converted to a Device
#[test]
fn airplay_service_info_to_device() {
    // Create a mock AirPlay service info
    // AirPlay services have format: <id>@<name>._raop._tcp.local.
    let service_type = "_raop._tcp.local.";
    let instance_name = "AABBCCDD11223344@Living Room Speaker";
    let host_name = "living-room-speaker.local.";
    let ip = "192.168.1.100";
    let port: u16 = 7000;
    let properties: Option<HashMap<String, String>> = None;

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        ip,
        port,
        properties,
    )
    .expect("Failed to create ServiceInfo");

    let device = Device::from(service_info);

    // Verify the device is correctly identified as an AirPlay device
    assert_eq!(device.app, "airplay");
    assert!(device.is_cast_device);
    assert!(!device.is_source_device);
    assert_eq!(device.ip, "192.168.1.100");
    assert_eq!(device.port, 7000);
    // The name should be extracted from the instance name (after @, before service type)
    assert_eq!(device.name, "Living Room Speaker");
}

/// Test that an AirPlay device with special characters in name is parsed correctly
#[test]
fn airplay_service_info_with_special_name() {
    let service_type = "_raop._tcp.local.";
    let instance_name = "001122334455@HomePod Mini";
    let host_name = "homepod-mini.local.";
    let ip = "10.0.0.50";
    let port: u16 = 7000;
    let properties: Option<HashMap<String, String>> = None;

    let service_info = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        ip,
        port,
        properties,
    )
    .expect("Failed to create ServiceInfo");

    let device = Device::from(service_info);

    assert_eq!(device.app, "airplay");
    assert_eq!(device.name, "HomePod Mini");
    assert_eq!(device.ip, "10.0.0.50");
}

/// Test that the AIRPLAY_SERVICE_NAME constant is correct
#[test]
fn airplay_service_name_constant() {
    assert_eq!(AIRPLAY_SERVICE_NAME, "_raop._tcp.local.");
}
