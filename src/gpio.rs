// TODO - clean these up
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

use core::marker::PhantomData;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

pub struct DI;
pub struct DO;
// /// Alternate function
// pub struct Alternate<MODE> {
//     _mode: PhantomData<MODE>,
// }

/// Represents a digital input or output level
pub enum Level {
    Low,
    High,
}

// ===============================================================
// Implement Generic Pins for this port, which allows you to use
// other peripherals without having to be completely rust-generic
// across all of the possible pins
// ===============================================================
/// Generic $PX pin
pub struct Pin<MODE> {
    pub pin: u8,
    pub port: bool,
    _mode: PhantomData<MODE>,
}

use crate::hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin};
use void::Void;

impl<MODE> Pin<MODE> {
    /// Convert the pin to be a floating input
    pub fn into_floating_input(self) -> Pin<Input<Floating>> {
        // unsafe {
        //     &(*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .pin_cnf[self.pin as usize]
        // }
        // .write(|w| {
        //     w.dir().input();
        //     w.input().connect();
        //     w.pull().disabled();
        //     w.drive().s0s1();
        //     w.sense().disabled();
        //     w
        // });

        Pin {
            _mode: PhantomData,
            port: self.port,
            pin: self.pin,
        }
    }
    pub fn into_pullup_input(self) -> Pin<Input<PullUp>> {
        // unsafe {
        //     &(*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .pin_cnf[self.pin as usize]
        // }
        // .write(|w| {
        //     w.dir().input();
        //     w.input().connect();
        //     w.pull().pullup();
        //     w.drive().s0s1();
        //     w.sense().disabled();
        //     w
        // });

        Pin {
            _mode: PhantomData,
            port: self.port,
            pin: self.pin,
        }
    }
    pub fn into_pulldown_input(self) -> Pin<Input<PullDown>> {
        // unsafe {
        //     &(*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .pin_cnf[self.pin as usize]
        // }
        // .write(|w| {
        //     w.dir().input();
        //     w.input().connect();
        //     w.pull().pulldown();
        //     w.drive().s0s1();
        //     w.sense().disabled();
        //     w
        // });

        Pin {
            _mode: PhantomData,
            port: self.port,
            pin: self.pin,
        }
    }

    /// Convert the pin to be a push-pull output with normal drive
    pub fn into_push_pull_output(self, initial_output: Level) -> Pin<Output<PushPull>> {
        let mut pin = Pin {
            _mode: PhantomData,
            port: self.port,
            pin: self.pin,
        };

        match initial_output {
            Level::Low => pin.set_low().unwrap(),
            Level::High => pin.set_high().unwrap(),
        }

        // unsafe {
        //     &(*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .pin_cnf[self.pin as usize]
        // }
        // .write(|w| {
        //     w.dir().output();
        //     w.input().connect(); // AJM - hack for SPI
        //     w.pull().disabled();
        //     w.drive().s0s1();
        //     w.sense().disabled();
        //     w
        // });

        pin
    }

    /// Convert the pin to be an open-drain output
    ///
    /// This method currently does not support configuring an
    /// internal pull-up or pull-down resistor.
    pub fn into_open_drain_output(
        self,
        config: OpenDrainConfig,
        initial_output: Level,
    ) -> Pin<Output<OpenDrain>> {
        let mut pin = Pin {
            _mode: PhantomData,
            port: self.port,
            pin: self.pin,
        };

        match initial_output {
            Level::Low => pin.set_low().unwrap(),
            Level::High => pin.set_high().unwrap(),
        }

        // This is safe, as we restrict our access to the dedicated
        // register for this pin.
        // let pin_cnf = unsafe {
        //     &(*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .pin_cnf[self.pin as usize]
        // };
        // pin_cnf.write(|w| {
        //     w.dir().output();
        //     w.input().disconnect();
        //     w.pull().disabled();
        //     w.drive().variant(config.variant());
        //     w.sense().disabled();
        //     w
        // });

        pin
    }
}

impl<MODE> InputPin for Pin<Input<MODE>> {
    type Error = Void;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.is_low().map(|v| !v)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        // Ok(unsafe {
        //     ((*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .in_
        //     .read()
        //     .bits()
        //         & (1 << self.pin))
        //         == 0
        // })
        Ok(self.port)
    }
}

impl<MODE> OutputPin for Pin<Output<MODE>> {
    type Error = Void;

    /// Set the output as high
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register - TODO(AJM) verify?
        // TODO - I wish I could do something like `.pins$i()`...
        // unsafe {
        //     (*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .outset
        //     .write(|w| w.bits(1u32 << self.pin));
        // }
        self.port = true;
        Ok(())
    }

    /// Set the output as low
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // NOTE(unsafe) atomic write to a stateless register - TODO(AJM) verify?
        // TODO - I wish I could do something like `.pins$i()`...
        // unsafe {
        //     (*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .outclr
        //     .write(|w| w.bits(1u32 << self.pin));
        // }
        self.port = false;
        Ok(())
    }
}

impl<MODE> StatefulOutputPin for Pin<Output<MODE>> {
    /// Is the output pin set as high?
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        self.is_set_low().map(|v| !v)
    }

    /// Is the output pin set as low?
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        // NOTE(unsafe) atomic read with no side effects - TODO(AJM) verify?
        // TODO - I wish I could do something like `.pins$i()`...
        // Ok(unsafe {
        //     ((*{
        //         #[cfg(not(feature = "52840"))]
        //         {
        //             P0::ptr()
        //         }
        //         #[cfg(feature = "52840")]
        //         {
        //             if !self.port {
        //                 P0::ptr()
        //             } else {
        //                 P1::ptr()
        //             }
        //         }
        //     })
        //     .out
        //     .read()
        //     .bits()
        //         & (1 << self.pin))
        //         == 0
        // })
        Ok(self.port)
    }
}

/// Pin configuration for open-drain mode
pub enum OpenDrainConfig {
    Disconnect0Standard1,
    Disconnect0HighDrive1,
    Standard0Disconnect1,
    HighDrive0Disconnect1,
}

// #[cfg(feature = "9160")]
// use crate::target::p0_ns::{pin_cnf, PIN_CNF};

// #[cfg(not(feature = "9160"))]
// use crate::target::p0::{pin_cnf, PIN_CNF};

// impl OpenDrainConfig {
//     fn variant(self) -> pin_cnf::DRIVEW {
//         use self::OpenDrainConfig::*;

//         match self {
//             Disconnect0Standard1 => pin_cnf::DRIVEW::D0S1,
//             Disconnect0HighDrive1 => pin_cnf::DRIVEW::D0H1,
//             Standard0Disconnect1 => pin_cnf::DRIVEW::S0D1,
//             HighDrive0Disconnect1 => pin_cnf::DRIVEW::H0D1,
//         }
//     }
// }

macro_rules! gpio {
    (
        $PX:ident, $pxsvd:ident, $px:ident, $port_value:expr, [
            $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
        ]
    ) => {
        /// GPIO
        pub mod $px {
            use super::{
                Pin,

                // Alternate,
                Floating,
                Input,
                Level,
                OpenDrain,
                OpenDrainConfig,
                Output,
                PullDown,
                PullUp,
                PushPull,

                PhantomData,
                $PX
            };

            // use crate::target;
            use crate::hal::digital::v2::{OutputPin, StatefulOutputPin, InputPin};
            use void::Void;



            // ===============================================================
            // This chunk allows you to obtain an nrf52-hal gpio from the
            // upstream nrf52 gpio definitions by defining a trait
            // ===============================================================
            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl Parts {
                pub fn new(_gpio: $PX) -> Self {
                    Self {
                        $(
                            $pxi: $PXi {
                                _mode: PhantomData,
                            },
                        )+
                    }
                }
            }

            // ===============================================================
            // Implement each of the typed pins usable through the nrf52-hal
            // defined interface
            // ===============================================================
            $(
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }


                impl<MODE> $PXi<MODE> {
                    /// Convert the pin to be a floating input
                    pub fn into_floating_input(self) -> $PXi<Input<Floating>> {
                        // unsafe { &(*$PX::ptr()).pin_cnf[$i] }.write(|w| {
                        //     w.dir().input();
                        //     w.input().connect();
                        //     w.pull().disabled();
                        //     w.drive().s0s1();
                        //     w.sense().disabled();
                        //     w
                        // });

                        $PXi {
                            _mode: PhantomData,
                        }
                    }
                    pub fn into_pulldown_input(self) -> $PXi<Input<PullDown>> {
                        // unsafe { &(*$PX::ptr()).pin_cnf[$i] }.write(|w| {
                        //     w.dir().input();
                        //     w.input().connect();
                        //     w.pull().pulldown();
                        //     w.drive().s0s1();
                        //     w.sense().disabled();
                        //     w
                        // });

                        $PXi {
                            _mode: PhantomData,
                        }
                    }
                    pub fn into_pullup_input(self) -> $PXi<Input<PullUp>> {
                        // unsafe { &(*$PX::ptr()).pin_cnf[$i] }.write(|w| {
                        //     w.dir().input();
                        //     w.input().connect();
                        //     w.pull().pullup();
                        //     w.drive().s0s1();
                        //     w.sense().disabled();
                        //     w
                        // });

                        $PXi {
                            _mode: PhantomData,
                        }
                    }

                    /// Convert the pin to bepin a push-pull output with normal drive
                    pub fn into_push_pull_output(self, initial_output: Level)
                        -> $PXi<Output<PushPull>>
                    {
                        let mut pin = $PXi {
                            _mode: PhantomData,
                        };

                        match initial_output {
                            Level::Low  => pin.set_low().unwrap(),
                            Level::High => pin.set_high().unwrap(),
                        }

                        // unsafe { &(*$PX::ptr()).pin_cnf[$i] }.write(|w| {
                        //     w.dir().output();
                        //     w.input().disconnect();
                        //     w.pull().disabled();
                        //     w.drive().s0s1();
                        //     w.sense().disabled();
                        //     w
                        // });

                        pin
                    }

                    /// Convert the pin to be an open-drain output
                    ///
                    /// This method currently does not support configuring an
                    /// internal pull-up or pull-down resistor.
                    pub fn into_open_drain_output(self,
                        config:         OpenDrainConfig,
                        initial_output: Level,
                    )
                        -> $PXi<Output<OpenDrain>>
                    {
                        let mut pin = $PXi {
                            _mode: PhantomData,
                        };

                        match initial_output {
                            Level::Low  => pin.set_low().unwrap(),
                            Level::High => pin.set_high().unwrap(),
                        }

                        // This is safe, as we restrict our access to the
                        // dedicated register for this pin.
                        // let pin_cnf = unsafe {
                            // &(*$PX::ptr()).pin_cnf[$i]
                        // // };
                        // pin_cnf.write(|w| {
                        //     w.dir().output();
                        //     w.input().disconnect();
                        //     w.pull().disabled();
                        //     w.drive().variant(config.variant());
                        //     w.sense().disabled();
                        //     w
                        // });

                        pin
                    }

                    /// Degrade to a generic pin struct, which can be used with peripherals
                    pub fn degrade(self) -> Pin<MODE> {
                        Pin {
                            _mode: PhantomData,
                            port: $port_value,
                            pin: $i
                        }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    type Error = Void;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        self.is_low().map(|v| !v)
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        // Ok(unsafe { ((*$PX::ptr()).in_.read().bits() & (1 << $i)) == 0 })
                        Ok(false)
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    type Error = Void;

                    /// Set the output as high
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        // NOTE(unsafe) atomic write to a stateless register - TODO(AJM) verify?
                        // TODO - I wish I could do something like `.pins$i()`...
                        // unsafe { (*$PX::ptr()).outset.write(|w| w.bits(1u32 << $i)); }
                        Ok(())
                    }

                    /// Set the output as low
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        // NOTE(unsafe) atomic write to a stateless register - TODO(AJM) verify?
                        // TODO - I wish I could do something like `.pins$i()`...
                        // unsafe { (*$PX::ptr()).outclr.write(|w| w.bits(1u32 << $i)); }
                        Ok(())
                    }
                }

                impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
                    /// Is the output pin set as high?
                    fn is_set_high(&self) -> Result<bool, Self::Error> {
                        self.is_set_low().map(|v| !v)
                    }

                    /// Is the output pin set as low?
                    fn is_set_low(&self) -> Result<bool, Self::Error> {
                        // NOTE(unsafe) atomic read with no side effects - TODO(AJM) verify?
                        // TODO - I wish I could do something like `.pins$i()`...
                        // Ok(unsafe { ((*$PX::ptr()).out.read().bits() & (1 << $i)) == 0 })
                        Ok(false)
                    }
                }
            )+
        }
    }
}

// ===========================================================================
gpio!(DI, di, di, false, [
    DI_00: (di_00,  0, Input<Floating>),
    DI_01: (di_01,  1, Input<Floating>),
    DI_02: (di_02,  2, Input<Floating>),
    DI_03: (di_03,  3, Input<Floating>),
    DI_04: (di_04,  4, Input<Floating>),
    DI_05: (di_05,  5, Input<Floating>),
    DI_06: (di_06,  6, Input<Floating>),
    DI_07: (di_07,  7, Input<Floating>),
    DI_08: (di_08,  8, Input<Floating>),
    DI_09: (di_09,  9, Input<Floating>),
    DI_10: (di_10, 10, Input<Floating>),
    DI_11: (di_11, 11, Input<Floating>),
    DI_12: (di_12, 12, Input<Floating>),
    DI_13: (di_13, 13, Input<Floating>),
    DI_14: (di_14, 14, Input<Floating>),
    DI_15: (di_15, 15, Input<Floating>),
    DI_16: (di_16, 16, Input<Floating>),
    DI_17: (di_17, 17, Input<Floating>),
    DI_18: (di_18, 18, Input<Floating>),
    DI_19: (di_19, 19, Input<Floating>),
    DI_20: (di_20, 20, Input<Floating>),
    DI_21: (di_21, 21, Input<Floating>),
    DI_22: (di_22, 22, Input<Floating>),
    DI_23: (di_23, 23, Input<Floating>),
    DI_24: (di_24, 24, Input<Floating>),
    DI_25: (di_25, 25, Input<Floating>),
    DI_26: (di_26, 26, Input<Floating>),
    DI_27: (di_27, 27, Input<Floating>),
    DI_28: (di_28, 28, Input<Floating>),
    DI_29: (di_29, 29, Input<Floating>),
    DI_30: (di_30, 30, Input<Floating>),
    DI_31: (di_31, 31, Input<Floating>),
]);

gpio!(DO, dout, dout, false, [
    DO_00: (dout_00,  0, Output<PushPull>),
    DO_01: (dout_01,  1, Output<PushPull>),
    DO_02: (dout_02,  2, Output<PushPull>),
    DO_03: (dout_03,  3, Output<PushPull>),
    DO_04: (dout_04,  4, Output<PushPull>),
    DO_05: (dout_05,  5, Output<PushPull>),
    DO_06: (dout_06,  6, Output<PushPull>),
    DO_07: (dout_07,  7, Output<PushPull>),
    DO_08: (dout_08,  8, Output<PushPull>),
    DO_09: (dout_09,  9, Output<PushPull>),
    DO_10: (dout_10, 10, Output<PushPull>),
    DO_11: (dout_11, 11, Output<PushPull>),
    DO_12: (dout_12, 12, Output<PushPull>),
    DO_13: (dout_13, 13, Output<PushPull>),
    DO_14: (dout_14, 14, Output<PushPull>),
    DO_15: (dout_15, 15, Output<PushPull>),
    DO_16: (dout_16, 16, Output<PushPull>),
    DO_17: (dout_17, 17, Output<PushPull>),
    DO_18: (dout_18, 18, Output<PushPull>),
    DO_19: (dout_19, 19, Output<PushPull>),
    DO_20: (dout_20, 20, Output<PushPull>),
    DO_21: (dout_21, 21, Output<PushPull>),
    DO_22: (dout_22, 22, Output<PushPull>),
    DO_23: (dout_23, 23, Output<PushPull>),
    DO_24: (dout_24, 24, Output<PushPull>),
    DO_25: (dout_25, 25, Output<PushPull>),
    DO_26: (dout_26, 26, Output<PushPull>),
    DO_27: (dout_27, 27, Output<PushPull>),
    DO_28: (dout_28, 28, Output<PushPull>),
    DO_29: (dout_29, 29, Output<PushPull>),
    DO_30: (dout_30, 30, Output<PushPull>),
    DO_31: (dout_31, 31, Output<PushPull>),
]);
