#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

#[macro_use(block)]
extern crate microbit;

use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;

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

        /* Initialise serial port on the micro:bit */
        let (mut tx, mut rx) = microbit::serial_port(gpio, p.UART0, BAUD115200);

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
                }else{
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
