use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

fn main() {
    // Link patches needed for ESP-IDF
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio8).unwrap();
    let mut button = PinDriver::input(peripherals.pins.gpio9).unwrap();
    button.set_pull(Pull::Up).unwrap();

    loop {
        if button.is_low() {
            // Button pressed → blink
            led.set_high().unwrap();
            FreeRtos::delay_ms(300);
            led.set_low().unwrap();
            FreeRtos::delay_ms(300);
        } else {
            // Released → LED off
            led.set_low().unwrap();
        }
    }
}
