#![no_std]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
pub extern crate nrf51_hal as hal;

pub extern crate cortex_m;
pub extern crate nb;

extern crate cortex_m_rt;

pub use nb::*;

pub use cortex_m_rt::*;
pub use hal::nrf51;
pub use nrf51::interrupt;
pub use nrf51::interrupt::*;
pub use nrf51::*;

use hal::gpio::gpio::Parts;
use hal::serial::{Rx, Serial, Tx, BAUDRATEW};

pub mod led;
