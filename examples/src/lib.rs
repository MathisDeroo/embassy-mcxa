#![no_std]
#![allow(clippy::missing_safety_doc)]

//! Shared board-specific helpers for the FRDM-MCXA276 examples.
//! These live with the examples so the HAL stays generic.

use hal::{clocks, pins};
use {defmt_rtt as _, embassy_mcxa as hal, panic_probe as _};

/// Initialize clocks and pin muxing for UART2 debug console.
/// Safe to call multiple times; writes are idempotent for our use.
pub unsafe fn init_uart2_pins(_p: &hal::pac::Peripherals) {
    // NOTE: Lpuart has been updated to properly enable + reset its own clocks.
    // GPIO has not.
    _ = clocks::enable_and_reset::<hal::peripherals::PORT2>(&clocks::periph_helpers::NoConfig);
    pins::configure_uart2_pins_port2();
}

/// Initialize clocks and reset for any GPIO/PORT.
#[allow(dead_code)]
pub unsafe fn init_gpio_pin<P: hal::gpio::GpioPin>(p: &hal::pac::Peripherals) {
    let port = 1;

    match port {
        0 => {
            _ = clocks::enable_and_reset::<hal::peripherals::PORT0>(&clocks::periph_helpers::NoConfig);
            _ = clocks::enable_and_reset::<hal::peripherals::GPIO0>(&clocks::periph_helpers::NoConfig);
        }
        1 => {
            _ = clocks::enable_and_reset::<hal::peripherals::PORT1>(&clocks::periph_helpers::NoConfig);
            _ = clocks::enable_and_reset::<hal::peripherals::GPIO1>(&clocks::periph_helpers::NoConfig);
        }
        2 => {
             _ = clocks::enable_and_reset::<hal::peripherals::PORT2>(&clocks::periph_helpers::NoConfig);
            _ = clocks::enable_and_reset::<hal::peripherals::GPIO2>(&clocks::periph_helpers::NoConfig);
        }
        3 => {
             _ = clocks::enable_and_reset::<hal::peripherals::PORT3>(&clocks::periph_helpers::NoConfig);
            _ = clocks::enable_and_reset::<hal::peripherals::GPIO3>(&clocks::periph_helpers::NoConfig);
        }
        4 => {
             _ = clocks::enable_and_reset::<hal::peripherals::PORT4>(&clocks::periph_helpers::NoConfig);
            _ = clocks::enable_and_reset::<hal::peripherals::GPIO4>(&clocks::periph_helpers::NoConfig);
        }
        _ => panic!("Unsupported GPIO port: {}", port),
    }
}

/// Initialize clocks and pin muxing for ADC.
pub unsafe fn init_adc_pins(_p: &hal::pac::Peripherals) {
    // NOTE: Lpuart has been updated to properly enable + reset its own clocks.
    // GPIO has not.
    _ = clocks::enable_and_reset::<hal::peripherals::PORT1>(&clocks::periph_helpers::NoConfig);
    pins::configure_adc_pins();
}
