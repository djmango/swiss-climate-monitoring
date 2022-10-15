use dht_sensor::DhtReading;
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{thread, time};

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

    sensor.set_high().ok();
    println!("Waiting on the sensor...");
    thread::sleep(time::Duration::from_millis(1000));

    for n in 0..=100 {
        println!("Hello, world {n}x!", n = n);

        match dht_sensor::dht22::Reading::read(&mut delay, &mut sensor) {
            Ok(dht_sensor::dht22::Reading {
                temperature,
                relative_humidity,
            }) => println!("{}°, {}% RH", temperature, relative_humidity),
            Err(e) => println!("Error {:?}", e),
        }

        thread::sleep(time::Duration::from_millis(1000));
    }
}
