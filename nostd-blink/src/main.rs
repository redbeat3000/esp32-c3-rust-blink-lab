#![no_std]
#![no_main]

use esp_hal::{
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    time::{Duration, Instant},
};
use esp_backtrace as _;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

    loop {
        if button.is_low() {
            led.toggle();
            let start = Instant::now();
            while start.elapsed() < Duration::from_millis(500) {}
        } else {
            led.set_low();
        }
    }
}
