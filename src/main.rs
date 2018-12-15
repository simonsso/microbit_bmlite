#![no_main]
#![no_std]

extern crate cortex_m_rt;
// #[cfg(not(test))]
extern crate panic_halt;

#[macro_use(block)]

extern crate microbit;

extern crate cortex_m;
use cortex_m::asm::{delay,nop};

use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;
use microbit::hal::serial;
use microbit::hal::spi;
//use microbit::hal::hal::blocking::spi::Transfer;
extern crate embedded_hal;
use embedded_hal::blocking::spi::Transfer;
use microbit::hal;
use embedded_hal::spi::FullDuplex;


use cortex_m_rt::entry;


fn hex_nible(i:u8)-> u8{
    if i&0xF <10 {
       (i&0xf)+0x30
    }else{
       (i & 0xf) + 0x40 - 10
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
      //  let mut pin23 = gpio.pin23.into_push_pull_output();
      //  pin23.set_high();

        /* Set 2 columns to output to control LED states */
        let mut led1 = gpio.pin4.into_push_pull_output();
        let mut led2 = gpio.pin6.into_push_pull_output();

        /* Initialise serial port on the micro:bit */

        /* Configure RX and TX pins accordingly */
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();

        /* Set up serial port using the prepared pins */
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

        let mut spix = hal::spi::Spi::spi0(p.SPI0,
                 gpio.pin23.into_floating_input().downgrade(),
                 gpio.pin21.into_push_pull_output().downgrade(),
                 gpio.pin22.into_floating_input().downgrade());

        //let mut spi = hal::
        /* Print a nice hello message */

        let s = b"Hello tell me what you doing Commands 1 2 3 :\r\n";
        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

        let s = b"Hej Knackt:\r\n";
        /* Endless loop */
        let mut buf:[u8;2] = [0x1c,0];
        let mut rbuf:[u8;2] = [0,0];

        let mut p23 = gpio.pin13.into_push_pull_output();
        let mut p21 = gpio.pin14.into_push_pull_output();
        let mut p22 = gpio.pin12.into_push_pull_output();

        let mut cs = gpio.pin16.into_push_pull_output();
        let mut reset = gpio.pin20.into_push_pull_output();

        cs.set_low();
        reset.set_high();
        p23.set_low();
        p22.set_low();
        p21.set_low(); 
        loop {
            /* Read and echo back */
            let c = 0x34;          
            match c {
                0x30 => {
                    reset.set_low();
                    p23.set_low();
                    p22.set_low();
                    p21.set_low();
                    reset.set_high();
                }
                0x31 => {
                    let s = b"You presss 1:";
                    let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                    p23.set_high();

                }
                0x32 => {
                    // Transfer::transfer(&mut spix,&mut buf);
                    let _ = hal::hal::spi::FullDuplex::send(&mut spix, 0x1c).unwrap();              
                    let ans = hal::hal::spi::FullDuplex::read(&mut spix).unwrap();
                    let s=hex(ans);
                    let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                    p22.set_high();
                }
                0x33 => {
                    p21.set_high();
                    // Transfer::transfer(&mut spix,&mut buf);
                    let _ = hal::hal::spi::FullDuplex::send(&mut spix, buf[0]).unwrap();

                    rbuf[0] = hal::hal::spi::FullDuplex::read(&mut spix).unwrap();

                    let _ = hal::hal::spi::FullDuplex::send(&mut spix, buf[1]).unwrap();

                    rbuf[1] = hal::hal::spi::FullDuplex::read(&mut spix).unwrap();

                    led1.set_high();
                    led2.set_low();
                    for i in buf.iter(){
                        let s=hex(*i);
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                    }
                }
                0x34 =>{
                    let mut mybuf:[u8;3] = [0xfC,0,0];
                    led2.set_high();
                    spix.transfer(&mut mybuf);

                    for i in mybuf.iter(){
                        let s=hex(*i);
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                    }
                }
                _ => {
                    led2.set_high();
                    led1.set_low();
                    // let c = c & 0xDF;
                    let _ = block!(tx.write(c));
                }
            }
        }
    }

    loop {
        continue;
    }
}


// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(hex_nible(0), 0x30);
//         assert_eq!(hex_nible(10), 0x41);
//         assert_eq!(hex_nible(16), 0x41);
//         assert_eq!(hex_nible(15), 0x46);
//         assert_eq!(hex_nible(7), 0x37);
//         assert_eq!(hex_nible(0x66), 0x36);
//     }
// }