use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::{thread, time};

use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    for n in 0..=100 {
        println!("Hello, world {n}x!", n = n);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
