#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mcxa as hal;
use embassy_mcxa::bind_interrupts;
use embassy_mcxa_examples::{init_gpio_pin, init_ostimer0};
use embassy_time::{Duration, Timer};
use embassy_mcxa::gpio;
use hal::gpio::{Level, SlewRate, DriveStrength, Input};

use embassy_mcxa_examples::init_uart2;
use hal::{uart, InterruptExt};

use core::fmt::Write;
use heapless::String;

// Bind only OS_EVENT for timer interrupts
bind_interrupts!(struct Irqs {
    OS_EVENT => hal::ostimer::time_driver::OsEventHandler;
});

#[used]
#[no_mangle]
static KEEP_OS_EVENT: unsafe extern "C" fn() = OS_EVENT;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = hal::init(hal::config::Config::default());

    unsafe {
        init_uart2(hal::pac());
        init_gpio_pin::<hal::peripherals::P1_7>(hal::pac());
        init_ostimer0(hal::pac());
    }

    let src = unsafe { hal::clocks::uart2_src_hz(hal::pac()) };
    let uart = uart::Uart::<uart::Lpuart2>::new(_p.LPUART2, uart::Config::new(src));

    uart.write_str_blocking("\r\n=== Button interrupt Example ===\r\n");

    // Initialize embassy-time global driver backed by OSTIMER0
    hal::ostimer::time_driver::init(hal::config::Config::default().time_interrupt_priority, 1_000_000);

    let mut monitor = Input::new(_p.P1_7, Level::High, DriveStrength::Normal, SlewRate::Slow);

    loop {
        let mut buf: String<20> = String::new();
        write!(buf, "\r\nPin level is {:?}\r\n", monitor.get_level());
        uart.write_str_blocking(&buf);
        Timer::after_millis(1000).await;
    }
}
