//! GP driver built around a type-erased `Flex` pin, similar to other Embassy HALs.
//! The exported `Output`/`Input` drivers own a `Flex` so they no longer depend on the
//! concrete pin type.

use core::convert::Infallible;
use core::marker::PhantomData;

use embassy_hal_internal::{Peri, PeripheralType};
use paste::paste;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Function {
    /// Function 0
    F0,
    /// Function 1
    F1,
    /// Function 2
    F2,
    /// Function 3
    F3,
    /// Function 4
    F4,
    /// Function 5
    F5,
    /// Function 6
    F6,
    /// Function 7
    F7,
}

/// Logical level for GP pins.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Level {
    Low,
    High,
}

impl From<bool> for Level {
    fn from(val: bool) -> Self {
        match val {
            true => Self::High,
            false => Self::Low,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Pull {
    None,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SlewRate {
    Fast,
    Slow,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DriveStrength {
    Normal,
    Double,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Inverter {
    Disabled,
    Enabled,
}

pub type Gpio = crate::peripherals::GPIO0;

/// Type-erased representation of a GP pin.
pub struct AnyPin {
    port: usize,
    pin: usize,
    gpio: &'static crate::pac::gpio0::RegisterBlock,
}

impl AnyPin {
    /// Create an `AnyPin` from raw components.
    pub fn new(port: usize, pin: usize, gpio: &'static crate::pac::gpio0::RegisterBlock) -> Self {
        Self { port, pin, gpio }
    }

    #[inline(always)]
    fn mask(&self) -> u32 {
        1 << self.pin
    }

    #[inline(always)]
    fn gpio(&self) -> &'static crate::pac::gpio0::RegisterBlock {
        self.gpio
    }

    #[inline(always)]
    pub fn port_index(&self) -> usize {
        self.port
    }

    #[inline(always)]
    pub fn pin_index(&self) -> usize {
        self.pin
    }
}

/// Type-level trait implemented by concrete pin ZSTs.
embassy_hal_internal::impl_peripheral!(AnyPin);

trait SealedPin {
    fn pin_port(&self) -> usize;

    fn port(&self) -> usize {
        self.pin_port() / 32
    }

    fn pin(&self) -> usize {
        self.pin_port() % 32
    }

    fn gpio(&self) -> &'static crate::pac::gpio0::RegisterBlock;

    fn port_reg(&self) -> &'static crate::pac::port0::RegisterBlock;

    fn pcr_reg(&self) -> &'static crate::pac::port0::Pcr0;

    fn set_function(&self, function: Function);

    fn set_pull(&self, pull: Pull);

    fn set_drive_strength(&self, strength: DriveStrength);

    fn set_slew_rate(&self, slew_rate: SlewRate);

    fn set_enable_input_buffer(&self);
}

/// GP pin trait.
#[allow(private_bounds)]
pub trait GpioPin: SealedPin + Sized + PeripheralType + Into<AnyPin> + 'static {
    /// Type-erase the pin.
    fn degrade(self) -> AnyPin {
        // SAFETY: This is only called within the GpioPin trait, which is only
        // implemented within this module on valid pin peripherals and thus
        // has been verified to be correct.
        AnyPin::new(self.port(), self.pin(), self.gpio())
    }
}

impl SealedPin for AnyPin {
    fn pin_port(&self) -> usize {
        self.port * 32 + self.pin
    }

    fn gpio(&self) -> &'static crate::pac::gpio0::RegisterBlock {
        self.gpio()
    }

    fn port_reg(&self) -> &'static crate::pac::port0::RegisterBlock {
        match self.port() {
            0 => unsafe { &*crate::pac::Port0::ptr() },
            1 => unsafe { &*crate::pac::Port1::ptr() },
            2 => unsafe { &*crate::pac::Port2::ptr() },
            3 => unsafe { &*crate::pac::Port3::ptr() },
            4 => unsafe { &*crate::pac::Port4::ptr() },
            _ => panic!("Invalid port: {}", self.port),
        }
    }

    fn pcr_reg(&self) -> &'static crate::pac::port0::Pcr0 {
        let port_reg = self.port_reg();
        match self.pin() {
            0 => port_reg.pcr0(),
            1 => port_reg.pcr1(),
            2 => port_reg.pcr2(),
            3 => port_reg.pcr3(),
            4 => port_reg.pcr4(),
            5 => port_reg.pcr5(),
            6 => port_reg.pcr6(),
            7 => port_reg.pcr7(),
            8 => port_reg.pcr8(),
            9 => port_reg.pcr9(),
            10 => port_reg.pcr10(),
            11 => port_reg.pcr11(),
            12 => port_reg.pcr12(),
            13 => port_reg.pcr13(),
            14 => port_reg.pcr14(),
            15 => port_reg.pcr15(),
            16 => port_reg.pcr16(),
            17 => port_reg.pcr17(),
            18 => port_reg.pcr18(),
            19 => port_reg.pcr19(),
            20 => port_reg.pcr20(),
            21 => port_reg.pcr21(),
            22 => port_reg.pcr22(),
            23 => port_reg.pcr23(),
            24 => port_reg.pcr24(),
            25 => port_reg.pcr25(),
            26 => port_reg.pcr26(),
            27 => port_reg.pcr27(),
            28 => port_reg.pcr28(),
            29 => port_reg.pcr29(),
            30 => port_reg.pcr30(),
            31 => port_reg.pcr31(),
            _ => panic!("Invalid pin: {}", self.pin),
        }
    }


    fn set_function(&self, function: Function) {
        let mux_value = match function {
            Function::F0 => 0,
            Function::F1 => 1,
            Function::F2 => 2,
            Function::F3 => 3,
            Function::F4 => 4,
            Function::F5 => 5,
            Function::F6 => 6,
            Function::F7 => 7,
        };

        unsafe {
            //let pcr = &port_reg.pcr[pin_index];
            self.pcr_reg().modify(|_, w| {
                w.mux().bits(mux_value)
            });
        }
    }

    fn set_pull(&self, pull: Pull) {
        match pull {
            Pull::None => {
                self.pcr_reg().modify(|_, w| w.pe().pe0());
            },
            Pull::Up => {
                self.pcr_reg().modify(|_, w| {
                    w.pe().pe1();
                    w.ps().ps1()
                });
            },
            Pull::Down => {
                self.pcr_reg().modify(|_, w| {
                    w.pe().pe1();
                    w.ps().ps0()
                });
            }
        }
    }

    fn set_drive_strength(&self, strength: DriveStrength) {
        match strength {
            DriveStrength::Normal => {
                self.pcr_reg().modify(|_, w| w.dse().dse0());

            }
            DriveStrength::Double => {
                self.pcr_reg().modify(|_, w| w.dse().dse1());
            }
        }
    }

    fn set_slew_rate(&self, slew_rate: SlewRate) {
        match slew_rate {
            SlewRate::Slow => {
                self.pcr_reg().modify(|_, w| w.sre().sre0());
            }
            SlewRate::Fast => {
                self.pcr_reg().modify(|_, w| w.sre().sre1());
            }
        }
    }

    fn set_enable_input_buffer(&self) {
        self.pcr_reg().modify(|_, w| w.ibe().ibe1());
    }
}

impl GpioPin for AnyPin {}

macro_rules! impl_pin {
    ($peri:ident, $port:expr, $pin:expr, $block:ident) => {
        paste! {
            impl SealedPin for crate::peripherals::$peri {
                fn pin_port(&self) -> usize {
                    $port * 32 + $pin
                }

                fn gpio(&self) -> &'static crate::pac::gpio0::RegisterBlock {
                    unsafe { &*crate::pac::$block::ptr() }
                }

                fn port_reg(&self) -> &'static crate::pac::port0::RegisterBlock {
                    unsafe { &*crate::pac::[<Port $port>]::ptr() }
                }

                fn pcr_reg(&self) -> &'static crate::pac::port0::Pcr0 {
                    self.port_reg().[<pcr $pin>]()
                }

                fn set_function(&self, function: Function) {
                    let mux_value = match function {
                        Function::F0 => 0,
                        Function::F1 => 1,
                        Function::F2 => 2,
                        Function::F3 => 3,
                        Function::F4 => 4,
                        Function::F5 => 5,
                        Function::F6 => 6,
                        Function::F7 => 7,
                    };
                    
                    unsafe {
                        let port_reg = &*crate::pac::[<Port $port>]::ptr();
                        port_reg.[<pcr $pin>]().modify(|_, w| {
                            w.mux().bits(mux_value)
                        });
                    }
                }

                fn set_pull(&self, pull: Pull) {
                    let port_reg = unsafe {&*crate::pac::[<Port $port>]::ptr()};
                    match pull {
                        Pull::None => {
                            port_reg.[<pcr $pin>]().modify(|_, w| w.pe().pe0());
                        },
                        Pull::Up => {
                            port_reg.[<pcr $pin>]().modify(|_, w| {
                                w.pe().pe1();
                                w.ps().ps1()
                            });
                        },
                        Pull::Down => {
                            port_reg.[<pcr $pin>]().modify(|_, w| {
                                w.pe().pe1();
                                w.ps().ps0()
                            });
                        }
                    }
                }

                fn set_drive_strength(&self, strength: DriveStrength) {
                    let port_reg = unsafe {&*crate::pac::[<Port $port>]::ptr()};
                    match strength {
                        DriveStrength::Normal => {
                            port_reg.[<pcr $pin>]().modify(|_, w| w.dse().dse0());

                        }
                        DriveStrength::Double => {
                            port_reg.[<pcr $pin>]().modify(|_, w| w.dse().dse1());
                        }
                    }
                }

                fn set_slew_rate(&self, slew_rate: SlewRate) {
                    let port_reg = unsafe {&*crate::pac::[<Port $port>]::ptr()};
                    match slew_rate {
                        SlewRate::Slow => {
                            port_reg.[<pcr $pin>]().modify(|_, w| w.sre().sre0());
                        }
                        SlewRate::Fast => {
                            port_reg.[<pcr $pin>]().modify(|_, w| w.sre().sre1());
                        }
                    }
                }

                fn set_enable_input_buffer(&self) {
                    let port_reg = unsafe {&*crate::pac::[<Port $port>]::ptr()};
                    port_reg.[<pcr $pin>]().modify(|_, w| w.ibe().ibe1());
                }
            }

            impl GpioPin for crate::peripherals::$peri {}

            impl From<crate::peripherals::$peri> for AnyPin {
                fn from(value: crate::peripherals::$peri) -> Self {
                    value.degrade()
                    }
                }

                impl crate::peripherals::$peri {
                /// Convenience helper to obtain a type-erased handle to this pin.
                pub fn degrade(&self) -> AnyPin {
                    AnyPin::new(self.port(), self.pin(), self.gpio())
                }
            }
        }
    };
}

impl_pin!(P0_0, 0, 0, Gpio0);
impl_pin!(P0_1, 0, 1, Gpio0);
impl_pin!(P0_2, 0, 2, Gpio0);
impl_pin!(P0_3, 0, 3, Gpio0);
impl_pin!(P0_4, 0, 4, Gpio0);
impl_pin!(P0_5, 0, 5, Gpio0);
impl_pin!(P0_6, 0, 6, Gpio0);
impl_pin!(P0_7, 0, 7, Gpio0);
impl_pin!(P0_12, 0, 12, Gpio0);
impl_pin!(P0_13, 0, 13, Gpio0);
impl_pin!(P0_14, 0, 14, Gpio0);
impl_pin!(P0_15, 0, 15, Gpio0);
impl_pin!(P0_16, 0, 16, Gpio0);
impl_pin!(P0_17, 0, 17, Gpio0);
impl_pin!(P0_18, 0, 18, Gpio0);
impl_pin!(P0_19, 0, 19, Gpio0);
impl_pin!(P0_20, 0, 20, Gpio0);
impl_pin!(P0_21, 0, 21, Gpio0);
impl_pin!(P0_22, 0, 22, Gpio0);
impl_pin!(P0_23, 0, 23, Gpio0);
impl_pin!(P0_24, 0, 24, Gpio0);
impl_pin!(P0_25, 0, 25, Gpio0);
impl_pin!(P0_26, 0, 26, Gpio0);
impl_pin!(P0_27, 0, 27, Gpio0);
    
impl_pin!(P1_0, 1, 0, Gpio1);
impl_pin!(P1_1, 1, 1, Gpio1);
impl_pin!(P1_2, 1, 2, Gpio1);
impl_pin!(P1_3, 1, 3, Gpio1);
impl_pin!(P1_4, 1, 4, Gpio1);
impl_pin!(P1_5, 1, 5, Gpio1);
impl_pin!(P1_6, 1, 6, Gpio1);
impl_pin!(P1_7, 1, 7, Gpio1);
impl_pin!(P1_8, 1, 8, Gpio1);
impl_pin!(P1_9, 1, 9, Gpio1);
impl_pin!(P1_10, 1, 10, Gpio1);
impl_pin!(P1_11, 1, 11, Gpio1);
impl_pin!(P1_12, 1, 12, Gpio1);
impl_pin!(P1_13, 1, 13, Gpio1);
impl_pin!(P1_14, 1, 14, Gpio1);
impl_pin!(P1_15, 1, 15, Gpio1);
impl_pin!(P1_16, 1, 16, Gpio1);
impl_pin!(P1_17, 1, 17, Gpio1);
impl_pin!(P1_18, 1, 18, Gpio1);
impl_pin!(P1_19, 1, 19, Gpio1);
impl_pin!(P1_29, 1, 29, Gpio1);
impl_pin!(P1_30, 1, 30, Gpio1);
impl_pin!(P1_31, 1, 31, Gpio1);

impl_pin!(P2_0, 2, 0, Gpio2);
impl_pin!(P2_1, 2, 1, Gpio2);
impl_pin!(P2_2, 2, 2, Gpio2);
impl_pin!(P2_3, 2, 3, Gpio2);
impl_pin!(P2_4, 2, 4, Gpio2);
impl_pin!(P2_5, 2, 5, Gpio2);
impl_pin!(P2_6, 2, 6, Gpio2);
impl_pin!(P2_7, 2, 7, Gpio2);
impl_pin!(P2_8, 2, 8, Gpio2);
impl_pin!(P2_9, 2, 9, Gpio2);
impl_pin!(P2_10, 2, 10, Gpio2);
impl_pin!(P2_11, 2, 11, Gpio2);
impl_pin!(P2_12, 2, 12, Gpio2);
impl_pin!(P2_13, 2, 13, Gpio2);
impl_pin!(P2_14, 2, 14, Gpio2);
impl_pin!(P2_15, 2, 15, Gpio2);
impl_pin!(P2_16, 2, 16, Gpio2);
impl_pin!(P2_17, 2, 17, Gpio2);
impl_pin!(P2_18, 2, 18, Gpio2);
impl_pin!(P2_19, 2, 19, Gpio2);
impl_pin!(P2_20, 2, 20, Gpio2);
impl_pin!(P2_21, 2, 21, Gpio2);
impl_pin!(P2_22, 2, 22, Gpio2);
impl_pin!(P2_23, 2, 23, Gpio2);
impl_pin!(P2_24, 2, 24, Gpio2);
impl_pin!(P2_25, 2, 25, Gpio2);
impl_pin!(P2_26, 2, 26, Gpio2);

impl_pin!(P3_0, 3, 0, Gpio3);
impl_pin!(P3_1, 3, 1, Gpio3);
impl_pin!(P3_2, 3, 2, Gpio3);
impl_pin!(P3_3, 3, 3, Gpio3);
impl_pin!(P3_4, 3, 4, Gpio3);
impl_pin!(P3_5, 3, 5, Gpio3);
impl_pin!(P3_6, 3, 6, Gpio3);
impl_pin!(P3_7, 3, 7, Gpio3);
impl_pin!(P3_8, 3, 8, Gpio3);
impl_pin!(P3_9, 3, 9, Gpio3);
impl_pin!(P3_10, 3, 10, Gpio3);
impl_pin!(P3_11, 3, 11, Gpio3);
impl_pin!(P3_12, 3, 12, Gpio3);
impl_pin!(P3_13, 3, 13, Gpio3);
impl_pin!(P3_14, 3, 14, Gpio3);
impl_pin!(P3_15, 3, 15, Gpio3);
impl_pin!(P3_16, 3, 16, Gpio3);
impl_pin!(P3_17, 3, 17, Gpio3);
impl_pin!(P3_18, 3, 18, Gpio3);
impl_pin!(P3_19, 3, 19, Gpio3);
impl_pin!(P3_20, 3, 20, Gpio3);
impl_pin!(P3_21, 3, 21, Gpio3);
impl_pin!(P3_22, 3, 22, Gpio3);
impl_pin!(P3_23, 3, 23, Gpio3);
impl_pin!(P3_24, 3, 24, Gpio3);
impl_pin!(P3_25, 3, 25, Gpio3);
impl_pin!(P3_26, 3, 26, Gpio3);
impl_pin!(P3_27, 3, 27, Gpio3);
impl_pin!(P3_28, 3, 28, Gpio3);
impl_pin!(P3_29, 3, 29, Gpio3);
impl_pin!(P3_30, 3, 30, Gpio3);
impl_pin!(P3_31, 3, 31, Gpio3);

impl_pin!(P4_0, 4, 0, Gpio4);
impl_pin!(P4_1, 4, 1, Gpio4);
impl_pin!(P4_2, 4, 2, Gpio4);
impl_pin!(P4_3, 4, 3, Gpio4);
impl_pin!(P4_4, 4, 4, Gpio4);
impl_pin!(P4_5, 4, 5, Gpio4);
impl_pin!(P4_6, 4, 6, Gpio4);
impl_pin!(P4_7, 4, 7, Gpio4);

/// A flexible pin that can be configured as input or output.
pub struct Flex<'d> {
    pin: Peri<'d, AnyPin>,
    _marker: PhantomData<&'d mut ()>,
}

impl<'d> Flex<'d> {
    /// Wrap the pin in a `Flex`.
    ///
    /// The pin remains unmodified. The initial output level is unspecified, but
    /// can be changed before the pin is put into output mode.
    pub fn new(pin: Peri<'d, impl GpioPin>) -> Self {
        pin.set_function(Function::F0);
        Self {
            pin: pin.into(),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn gpio(&self) -> &'static crate::pac::gpio0::RegisterBlock {
        self.pin.gpio()
    }

    #[inline]
    fn mask(&self) -> u32 {
        self.pin.mask()
    }

    /// Put the pin into input mode.
    ///
    /// The pull setting is left unchanged.
    pub fn set_as_input(&mut self, strength: DriveStrength, slew_rate: SlewRate) {
        let mask = self.mask();
        let gpio = self.gpio();

        self.set_pull(Pull::None);
        self.set_drive_strength(strength);
        self.set_slew_rate(slew_rate);
        self.set_enable_input_buffer();
        
        gpio.pddr().modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
    }

    /// Put the pin into output mode.
    ///
    /// The initial output level is left unchanged.
    pub fn set_as_output(&mut self, strength: DriveStrength, slew_rate: SlewRate) {
        let mask = self.mask();
        let gpio = self.gpio();

        self.set_pull(Pull::None);
        self.set_drive_strength(strength);
        self.set_slew_rate(slew_rate);

        gpio.pddr().modify(|r, w| unsafe { w.bits(r.bits() | mask) });
    }

    /// Set output level to High.
    #[inline]
    pub fn set_high(&mut self) {
        self.gpio().psor().write(|w| unsafe { w.bits(self.mask()) });
    }

    /// Set output level to Low.
    #[inline]
    pub fn set_low(&mut self) {
        self.gpio().pcor().write(|w| unsafe { w.bits(self.mask()) });
    }

    /// Set output level to the given `Level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }

    /// Toggle output level.
    #[inline]
    pub fn toggle(&mut self) {
        self.gpio().ptor().write(|w| unsafe { w.bits(self.mask()) });
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        (self.gpio().pdir().read().bits() & self.mask()) != 0
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Is the output pin set as high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.is_high()
    }

    /// Is the output pin set as low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    pub fn set_pull(&mut self, pull: Pull) {
        self.pin.set_pull(pull);
    }

    pub fn set_drive_strength(&mut self, strength: DriveStrength) {
        self.pin.set_drive_strength(strength);
    }

    pub fn set_slew_rate(&mut self, slew_rate: SlewRate) {
        self.pin.set_slew_rate(slew_rate);
    }

    pub fn set_enable_input_buffer(&mut self) {
        self.pin.set_enable_input_buffer();
    }

    pub fn get_level(&self) -> Level {
        self.is_high().into()
    }
}

/// GP output driver that owns a `Flex` pin.
pub struct Output<'d> {
    flex: Flex<'d>,
}

impl<'d> Output<'d> {
    /// Create a GPIO output driver for a [GpioPin] with the provided [Level].
    pub fn new(pin: Peri<'d, impl GpioPin>, 
        initial: Level,
        strength: DriveStrength,
        slew_rate: SlewRate,
        ) -> Self {
        let mut flex = Flex::new(pin);
        flex.set_level(initial);
        flex.set_as_output(strength, slew_rate);
        Self { flex }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.flex.set_high();
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.flex.set_low();
    }

    /// Set the output level.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.flex.set_level(level);
    }

    /// Toggle the output level.
    #[inline]
    pub fn toggle(&mut self) {
        self.flex.toggle();
    }

    /// Is the output pin set as high?
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.flex.is_high()
    }

    /// Is the output pin set as low?
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// Expose the inner `Flex` if callers need to reconfigure the pin.
    #[inline]
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }
}

/// GP input driver that owns a `Flex` pin.
pub struct Input<'d> {
    flex: Flex<'d>,
}

impl<'d> Input<'d> {
    /// Create a GPIO input driver for a [GpioPin].
    pub fn new(pin: Peri<'d, impl GpioPin>, 
        initial: Level,
        strength: DriveStrength,
        slew_rate: SlewRate,
        ) -> Self {
        let mut flex = Flex::new(pin);
        flex.set_level(initial);
        flex.set_as_input(strength, slew_rate);
        Self { flex }
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        self.flex.is_high()
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        self.flex.is_low()
    }

    /// Expose the inner `Flex` if callers need to reconfigure the pin.
    #[inline]
    pub fn into_flex(self) -> Flex<'d> {
        self.flex
    }

    pub fn get_level(&self) -> Level {
        self.flex.get_level()
    }
}

// Both embedded_hal 0.2 and 1.0 must be supported by embassy HALs.
impl embedded_hal_02::digital::v2::InputPin for Flex<'_> {
    // GPIO operations on this block cannot fail, therefor we set the error type
    // to Infallible to guarantee that we can only produce Ok variants.
    type Error = Infallible;

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl embedded_hal_02::digital::v2::InputPin for Input<'_> {
    type Error = Infallible;

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

impl embedded_hal_02::digital::v2::OutputPin for Flex<'_> {
    type Error = Infallible;

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl embedded_hal_02::digital::v2::StatefulOutputPin for Flex<'_> {
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl embedded_hal_02::digital::v2::ToggleableOutputPin for Flex<'_> {
    type Error = Infallible;

    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

impl embedded_hal_1::digital::ErrorType for Flex<'_> {
    type Error = Infallible;
}

impl embedded_hal_1::digital::ErrorType for Input<'_> {
    type Error = Infallible;
}

impl embedded_hal_1::digital::ErrorType for Output<'_> {
    type Error = Infallible;
}

impl embedded_hal_1::digital::InputPin for Input<'_> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_high())
    }

    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_low())
    }
}

impl embedded_hal_1::digital::OutputPin for Flex<'_> {
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl embedded_hal_1::digital::StatefulOutputPin for Flex<'_> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_high())
    }

    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok((*self).is_set_low())
    }
}
