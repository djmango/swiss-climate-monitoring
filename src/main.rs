use anyhow::bail;
use dht_sensor::DhtReading;
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::peripherals::Peripherals;

use embedded_svc::wifi::*;

// use esp_idf_svc::ping;

use esp_idf_svc::{
    httpd::Configuration, netif::EspNetifStack, nvs::EspDefaultNvs, ping::EspPing,
    sysloop::EspSysLoopStack, wifi::EspWifi,
};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::{error, info};
use std::{sync::Arc, thread, time::Duration};

const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> Result<Box<EspWifi>, anyhow::Error> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    wifi.set_configuration(&embedded_svc::wifi::Configuration::Mixed(
        ClientConfiguration {
            ssid: SSID.into(),
            password: PASS.into(),
            channel,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    info!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Started(ApIpStatus::Done),
    ) = status
    {
        info!("Wifi connected. Status: {:?}", status);

        // esp_idf_svc::ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let dht22_1 = pins.gpio21;
    let mut sensor = dht22_1.into_input_output().unwrap();

    let mut delay = esp_idf_hal::delay::Ets;

    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());

    let wifi = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    );

    if wifi.is_err() {
        error!("Wifi is error: {:?}", wifi.err());
    } else {
        info!("Wifi is okay");
    }

    sensor.set_high().ok();
    println!("Waiting on the sensor...");
    // delay.delay_ms(1000_u16); this one sems to be causing task watchdog got triggered
    // thread::sleep(Duration::from_millis(1000));

    for n in 0..=100 {
        println!("Hello, world {n}x!", n = n);

        match dht_sensor::dht22::Reading::read(&mut delay, &mut sensor) {
            Ok(dht_sensor::dht22::Reading {
                temperature,
                relative_humidity,
            }) => println!("{}°, {}% RH", temperature, relative_humidity),
            Err(e) => println!("Error {:?}", e),
        }

        // delay.delay_ms(1000_u16);
        thread::sleep(Duration::from_millis(100));
    }
}
