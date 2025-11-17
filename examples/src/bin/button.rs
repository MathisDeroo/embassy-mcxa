#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mcxa::bind_interrupts;
use embassy_time::Timer;
use hal::gpio::{Pull, Level, SlewRate, DriveStrength, Input};
use {defmt_rtt as _, embassy_mcxa as hal, panic_probe as _};

// Bind only OS_EVENT for timer interrupts
bind_interrupts!(struct Irqs {
    OS_EVENT => hal::ostimer::time_driver::OsEventHandler;
});

#[used]
#[no_mangle]
static KEEP_OS_EVENT: unsafe extern "C" fn() = OS_EVENT;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(hal::config::Config::default());

    defmt::info!("Button example");

   // Initialize embassy-time global driver backed by OSTIMER0
    hal::ostimer::time_driver::init(hal::config::Config::default().time_interrupt_priority, 1_000_000);

    let mut monitor = Input::new(p.P1_7, Pull::None, DriveStrength::Normal, SlewRate::Slow);

    loop {
        defmt::info!("Pin level is {:?}", monitor.get_level());
        Timer::after_millis(1000).await;
    }
}
