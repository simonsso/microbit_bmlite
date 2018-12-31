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
use microbit::led::Display;
extern crate bmlite;
use bmlite::*;
extern crate embedded_hal;
use microbit::hal;
use hal::spi::SpiExt;


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

pub mod bitmaps;
// use microbit::microbit_bitmaps as bitmaps;
#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, 2048 as usize) }

    if let Some(p) = microbit::Peripherals::take() {
        /* Split GPIO pins */
        let mut gpio = p.GPIO.split();
        let mut delay = hal::delay::Delay::new(p.TIMER0);

      /* Set row of // LED matrix to permanent high */

        let mut display = Display::new(
            gpio.pin4.into_push_pull_output(),
            gpio.pin5.into_push_pull_output(),
            gpio.pin6.into_push_pull_output(),
            gpio.pin7.into_push_pull_output(),
            gpio.pin8.into_push_pull_output(),
            gpio.pin9.into_push_pull_output(),
            gpio.pin10.into_push_pull_output(),
            gpio.pin11.into_push_pull_output(),
            gpio.pin12.into_push_pull_output(),
            gpio.pin13.into_push_pull_output(),
            gpio.pin14.into_push_pull_output(),
            gpio.pin15.into_push_pull_output());

        display.display_pre_u32(&mut delay, bitmaps::img::square_image , 300);
        display.display_pre_u32(&mut delay, bitmaps::img::square_small_image, 300);
        display.display_pre_u32(&mut delay, bitmaps::img::dot33 , 300);
        display.display_pre_u32(&mut delay, bitmaps::img::diamond_small_image , 300);
        display.display_pre_u32(&mut delay, bitmaps::img::diamond_image , 300);

        /* Initialise serial port on the micro:bit */

        /* Configure RX and TX pins accordingly */
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();

        /* Set up serial port using the prepared pins */
        let (mut tx, mut rx) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();

        let mut spix = p.SPI1.constrain( hal::spi::Pins{
                sck: gpio.pin23.into_push_pull_output().downgrade(),
                mosi: gpio.pin21.into_push_pull_output().downgrade(),
                miso: gpio.pin22.into_floating_input().downgrade()});

        //let mut spi = hal::
        /* Print a nice hello message */

        let s = b"Hello connect BM Lite and start\r\n";
        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();


        // Conect pins for reset and IRQ
        let mut spi_cs = gpio.pin16.into_push_pull_output();
        let mut spi_rst = gpio.pin30.into_push_pull_output();
        let mut spi_irq = gpio.pin0.into_pull_up_input();

        let mut  btn_a = gpio.pin17.into_pull_up_input();
        let mut  btn_b = gpio.pin26.into_pull_up_input();

        let mut bm = BmLite::new(spix, spi_cs,spi_rst,spi_irq);

        let _ans = bm.reset(||{asm::delay(100);});

        loop {
            let mut fingerpresent = false;
            display.display_pre_u32(&mut delay,bitmaps::img::minus,30);
            let ans = bm.capture(1000);
            match ans {
                Ok(present) => { fingerpresent = present==0 },
                Err(_) => {
                    // let _ans = bm.reset();
                },
            } // The user interface touch the sensor and btn at the same time to ensoll
            // Extreemly secure
            if btn_a.is_low(){
                display.display_pre_u32(&mut delay,bitmaps::img::question_mark,5000);
                if btn_a.is_low(){
                    let _ans = bm.delete_all();
                }
            }
            if btn_b.is_low(){
                display.display_pre_u32(&mut delay,bitmaps::img::hbars_top_botom,1000);

                let ans = bm.enroll(|progress|
                                        {
                                            let pat = match progress{
                                                0 => 0x3070,
                                                1 => 0x30f0,
                                                2 => 0x31f0,
                                                3 => 0x33f0,
                                                _ => 0x37f0,
                                            };
                                            display.display_static_raw(pat);
                                         });
                match ans {
                    Ok(_) => {
                        let s=b"Finger enrolled\r\n";
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();

                        display.display_pre_u32(&mut delay, bitmaps::img::full_square,1000);
                        display.display_pre_u32(&mut delay, bitmaps::img::square_image,200);
                        display.display_pre_u32(&mut delay, bitmaps::img::square_small_image,200);
                        display.display_pre_u32(&mut delay, bitmaps::img::dot33,200);
                    },
                    Err(_) => {
                        loop{
                          display.display_pre_u32(&mut delay,bitmaps::img::x_big,3000);
                        }
                    }
                }
            }else if fingerpresent{
                let ans= bm.identify();
                match ans {
                    Ok(id) => {
                        display.display_pre_u32(&mut delay,bitmaps::img::circle,300);
                        match id{
                            0 => {display.display_pre_u32(&mut delay, bitmaps::img::sword_image ,1400);
                            }
                            1 => {display.display_pre_u32(&mut delay, bitmaps::img::pacman_image  ,1400);
                            }
                            2 => {display.display_pre_u32(&mut delay, bitmaps::img::pitchfork_image ,1400);
                            }
                            _ => {}
                        }
                        let s=b"Finger identifed as ";
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                        let s= hex(id as u8);
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                        display.display_pre_u32(&mut delay,bitmaps::img::circle,300);
                    }
                    Err(bmlite::Error::NoMatch) => {
                        // led0.set_high();
                        let s=b"Finger Not known.\r\n";
                        let _ = s.into_iter().map(|c| block!(tx.write(*c))).last();
                        display.display_pre_u32(&mut delay,bitmaps::img::x_big, 300);
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
