use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::{thread, time};

use esp_idf_hal::peripherals::Peripherals;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio4.into_output().unwrap();

    for n in 0..=100 {
        println!("Hello, world {n}x!", n = n);
        thread::sleep(time::Duration::from_millis(100));
    }
}
