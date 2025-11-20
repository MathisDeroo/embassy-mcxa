#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mcxa::bind_interrupts;
use embassy_time::Timer;
use {defmt_rtt as _, embassy_mcxa as hal, panic_probe as _};

use hal::gpio::Input;
use hal::pac::port0::pcr0::{Ps, Pe, Mux, Sre, Dse};

use hal::interrupt::{InterruptExt};

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

    defmt::info!("GPIO interrupt example");

   // Initialize embassy-time global driver backed by OSTIMER0
    hal::ostimer::time_driver::init(hal::config::Config::default().time_interrupt_priority, 1_000_000);

    let mut pin = Input::new(p.P1_7, Pe::Pe1, Ps::Ps1, Dse::Dse0, Sre::Sre0);

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
