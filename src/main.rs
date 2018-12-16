#![no_main]
#![no_std]
#![feature(alloc)]
#![feature(lang_items)]

extern crate cortex_m_rt;
extern crate alloc;
// #[cfg(not(test))]
extern crate panic_halt;
use core::alloc::Layout;

extern crate alloc_cortex_m;
extern crate cortex_m_rt as rt; // v0.5.x

#[macro_use(block)]
extern crate microbit;

extern crate cortex_m;

use cortex_m::asm;
use alloc_cortex_m::CortexMHeap;
use microbit::hal::prelude::*;
use microbit::hal::serial::BAUD115200;
use microbit::hal::serial;
extern crate bmlite;
use bmlite::*;
use microbit::hal::spi;
//use microbit::hal::hal::blocking::spi::Transfer;
extern crate embedded_hal;
use embedded_hal::blocking::spi::Transfer;
use microbit::hal;
use embedded_hal::spi::FullDuplex;


use cortex_m_rt::entry;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

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
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 2048 as usize) }

    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let mut gpio = p.GPIO.split();

      /* Set row of LED matrix to permanent high */
        let mut pin13 = gpio.pin13.into_push_pull_output();
        pin13.set_high();



        /* Initialise serial port on the micro:bit */

        /* Configure RX and TX pins accordingly */
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();

        /* Set up serial port using the prepared pins */
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

        let mut spix = hal::spi::Spi::spi0(p.SPI0,
                 gpio.pin23.into_push_pull_output().downgrade(),
                 gpio.pin21.into_push_pull_output().downgrade(),
                 gpio.pin22.into_floating_input().downgrade());

        //let mut spi = hal::
        /* Print a nice hello message */

        let s = b"Hello tell me what you doing Commands 1 2 3 :\r\n";
        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

        let s = b"Hej Knackt:\r\n";

    //         Get som debyg pins
        let mut led0 = gpio.pin8.into_push_pull_output();
        let mut led1 = gpio.pin4.into_push_pull_output();
        let mut led2 = gpio.pin6.into_push_pull_output();
        let mut led3 = gpio.pin9.into_push_pull_output();
        let mut led4 = gpio.pin12.into_push_pull_output();

        // Conect pins for reset and IRQ
        let mut spi_cs = gpio.pin16.into_push_pull_output();
        let mut spi_rst = gpio.pin30.into_push_pull_output();
        let mut spi_irq = gpio.pin0.into_pull_up_input();

        let mut  btn_a = gpio.pin17.into_pull_up_input();
        let mut  btn_b = gpio.pin26.into_pull_up_input();

        let mut bm = BmLite::new(spix, spi_cs,spi_rst,spi_irq);
        let _ans = bm.reset_start();
        asm::delay(100);
        let _ans = bm.reset_end();


        loop {
            let ans = bm.capture(0);
            match ans {
                Ok(_) => {},
                Err(_) => {
                    led1.set_high();
                    // let _ans = bm.reset();
                },
            } // The user interface touch the sensor and btn at the same time to ensoll
            // Extreemly secure
            if btn_a.is_low(){
                led0.set_high();
                led1.set_high();
                led2.set_high();
                led3.set_high();
                let _ans = bm.delete_all();
                let ans = bm.enroll();
                match ans {
                    Ok(_) => {
                        let s=b"Finger enrolled";
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                    
                        led0.set_low();
                        led1.set_low();
                        led2.set_low();
                        led3.set_low();
                        led3.set_high();
                    },
                    Err(_) => { 
                        loop{
                            led0.set_low();
                            led1.set_low();
                            led2.set_low();
                            led3.set_low();
                            led0.set_high();
                            led1.set_high();
                        }
                    }
                }
            }else{
                led0.set_low();
                led1.set_low();
                led2.set_low();
                led3.set_low();
                let ans= bm.identify();
                match ans {
                    Ok(id) => {
                        match id{
                            0 => {led2.set_high()}
                            1 => {led3.set_high()}
                            2 => {led3.set_high();led2.set_high()}
                            _ => {}
                        }
                        let s=b"Finger identifed as ";
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                        let s= hex(id as u8);
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

                        asm::delay(5000000);
                    }
                    Err(bmlite::Error::NoMatch) => {
                        led0.set_high();
                        let s=b"Finger Not known.";
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                    
                    }
                    Err(_) => {/*let _ans=bm.reset();*/}
                }
            }
        }
    }
    loop {
        continue;
    }
}
// required: define how Out Of Memory (OOM) conditions should be handled
// *if* no other crate has already defined `oom`
#[lang = "oom"]
#[no_mangle]

pub fn rust_oom(_layout: Layout) -> ! {
   // trap here for the debuger to find
   asm::bkpt();
   loop {
   }
}