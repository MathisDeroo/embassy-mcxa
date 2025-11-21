#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::gpio::{DriveStrength, Input, Pull, SlewRate};
use hal::interrupt::InterruptExt;
use {defmt_rtt as _, embassy_mcxa as hal, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(hal::config::Config::default());

    defmt::info!("GPIO interrupt example");

    let mut pin = Input::new(p.P1_7, Pull::Up, DriveStrength::Normal, SlewRate::Fast);

    unsafe {
        hal::interrupt::GPIO1.enable();
    }

    unsafe {
        cortex_m::interrupt::enable();
    }

    let mut press_count = 0u32;

    loop {
        pin.wait_for_falling_edge().await;

        press_count += 1;

        defmt::info!("Button pressed! Count: {}", press_count);
        Timer::after_millis(50).await;
    }
}
