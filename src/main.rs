#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

#[macro_use(block)]
extern crate microbit;

use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;
use microbit::hal::serial;
use microbit::hal;



use cortex_m_rt::entry;


fn hex_nible(i:u8)-> u8{
    if i&0xF <10 {
       (i&0xf)+0x30
    }else{
       (i & 0xf) + 0x40
    }
}

fn hex(a:u8) -> [u8;2] {
    [hex_nible(a>>4),hex_nible(a)]
}


#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let mut gpio = p.GPIO.split();

      /* Set row of LED matrix to permanent high */
        let mut pin13 = gpio.pin13.into_push_pull_output();
        pin13.set_high();

        /* Set 2 columns to output to control LED states */
        let mut led1 = gpio.pin4.into_push_pull_output();
        let mut led2 = gpio.pin6.into_push_pull_output();

        /* Initialise serial port on the micro:bit */

        /* Configure RX and TX pins accordingly */
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();

        /* Set up serial port using the prepared pins */
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();


        let mut spi = hal::
        /* Print a nice hello message */

        let s = b"Commands 1 2 3 :\r\n";
        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

        let s = b"Hej Knackt:\r\n";
        /* Endless loop */
        loop {
            /* Read and echo back */
            if let Ok(c) = block!(rx.read()) {
                let s=hex(c);
                let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                if c == 0x33 {
                    led1.set_high();
                    led2.set_low();
                }else{
                    led2.set_high();
                    led1.set_low();
                    let c = c & 0xDF;
                    let _ = block!(tx.write(c));
                }
            }
        }
    }

    loop {
        continue;
    }
}
