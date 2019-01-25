
// Derived based on data from https://github.com/bbcmicrobit/micropython/blob/master/source/microbit/microbitconstimage.cpp
// Which require this licence.

/*
 * This file is part of the MicroPython project, http://micropython.org/
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Damien P. George
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */


pub mod img {
   #![allow(non_upper_case_globals)]

   ///    *** 
   ///   *   *
   ///     ** 
   ///        
   ///     *  
   ///
   pub const question_mark:u32 = 0x01dbc7fd;

   ///   *   *
   ///    * * 
   ///     *  
   ///    * * 
   ///   *   *
   ///
   pub const x_big:u32 = 0x06a7f7aa;

   ///    *** 
   ///   *   *
   ///   *   *
   ///   *   *
   ///    *** 
   ///
   pub const circle:u32 = 0x05db0975;

   ///   *****
   ///        
   ///        
   ///        
   ///   *****
   ///
   pub const hbars_top_botom:u32 = 0x07e30ff8;

   ///        
   ///   *****
   ///        
   ///   *****
   ///        
   ///
   pub const hbars_center:u32 = 0x041fff07;

   ///   *****
   ///   *****
   ///   *****
   ///   *****
   ///   *****
   ///
   pub const full_square:u32 = 0x00030000;

   ///        
   ///        
   ///     *  
   ///        
   ///        
   ///
   pub const dot33:u32 = 0x07fff7ff;

   ///        
   ///        
   ///    *** 
   ///        
   ///        
   ///
   pub const minus:u32 = 0x03fff6ff;

   ///        
   ///        
   ///   *****
   ///        
   ///        
   ///
   pub const longminus:u32 = 0x03fff0ff;

   ///    * * 
   ///   *****
   ///   *****
   ///    *** 
   ///     *  
   ///
   pub const heart_image:u32 = 0x001bc08f;

   ///        
   ///    * * 
   ///    *** 
   ///     *  
   ///        
   ///
   pub const heart_small_image:u32 = 0x02bff6df;

   ///        
   ///    * * 
   ///        
   ///   *   *
   ///    *** 
   ///
   pub const happy_image:u32 = 0x06bb3f77;

   ///        
   ///        
   ///        
   ///   *   *
   ///    *** 
   ///
   pub const smile_image:u32 = 0x07fb3f77;

   ///        
   ///    * * 
   ///        
   ///    *** 
   ///   *   *
   ///
   pub const sad_image:u32 = 0x06a7ff8f;

   ///        
   ///    * * 
   ///        
   ///    * * 
   ///   * * *
   ///
   pub const confused_image:u32 = 0x06a3ffaf;

   ///   *   *
   ///    * * 
   ///        
   ///   *****
   ///   * * *
   ///
   pub const angry_image:u32 = 0x06a3ff02;

   ///        
   ///   ** **
   ///        
   ///    *** 
   ///        
   ///
   pub const asleep_image:u32 = 0x049fff8f;

   ///    * * 
   ///        
   ///     *  
   ///    * * 
   ///     *  
   ///
   pub const surprised_image:u32 = 0x07fbc7af;

   ///   *   *
   ///        
   ///   *****
   ///     * *
   ///     ***
   ///
   pub const silly_image:u32 = 0x03f3b0d2;

   ///   *****
   ///   ** **
   ///        
   ///    * * 
   ///    *** 
   ///
   pub const fabulous_image:u32 = 0x049b0fa8;

   ///    * * 
   ///        
   ///      * 
   ///     *  
   ///    *   
   ///
   pub const meh_image:u32 = 0x03ff4fdf;

   ///        
   ///       *
   ///      * 
   ///   * *  
   ///    *   
   ///
   pub const yes_image:u32 = 0x01ff7f5f;

   ///   *   *
   ///    * * 
   ///     *  
   ///    * * 
   ///   *   *
   ///
   pub const no_image:u32 = 0x06a7f7aa;

   ///     *  
   ///     *  
   ///     *  
   ///        
   ///        
   ///
   pub const clock12_image:u32 = 0x077ff7fd;

   ///      * 
   ///      * 
   ///     *  
   ///        
   ///        
   ///
   pub const clock1_image:u32 = 0x06ffd7ff;

   ///        
   ///      **
   ///     *  
   ///        
   ///        
   ///
   pub const clock2_image:u32 = 0x04fff7ff;

   ///        
   ///        
   ///     ***
   ///        
   ///        
   ///
   pub const clock3_image:u32 = 0x03fff5ff;

   ///        
   ///        
   ///     *  
   ///      **
   ///        
   ///
   pub const clock4_image:u32 = 0x07fff7e7;

   ///        
   ///        
   ///     *  
   ///      * 
   ///      * 
   ///
   pub const clock5_image:u32 = 0x07ffb7ef;

   ///        
   ///        
   ///     *  
   ///     *  
   ///     *  
   ///
   pub const clock6_image:u32 = 0x07fbf7df;

   ///        
   ///        
   ///     *  
   ///    *   
   ///    *   
   ///
   pub const clock7_image:u32 = 0x07ff77bf;

   ///        
   ///        
   ///     *  
   ///   **   
   ///        
   ///
   pub const clock8_image:u32 = 0x07fff73f;

   ///        
   ///        
   ///   ***  
   ///        
   ///        
   ///
   pub const clock9_image:u32 = 0x07fff2ff;

   ///        
   ///   **   
   ///     *  
   ///        
   ///        
   ///
   pub const clock10_image:u32 = 0x079ff7ff;

   ///    *   
   ///    *   
   ///     *  
   ///        
   ///        
   ///
   pub const clock11_image:u32 = 0x07bfe7ff;

   ///     *  
   ///    *** 
   ///   * * *
   ///     *  
   ///     *  
   ///
   pub const arrow_n_image:u32 = 0x063bf1dd;

   ///     ***
   ///      **
   ///     * *
   ///    *   
   ///   *    
   ///
   pub const arrow_ne_image:u32 = 0x04efd5b9;

   ///     *  
   ///      * 
   ///   *****
   ///      * 
   ///     *  
   ///
   pub const arrow_e_image:u32 = 0x02fbf0ed;

   ///   *    
   ///    *   
   ///     * *
   ///      **
   ///     ***
   ///
   pub const arrow_se_image:u32 = 0x07b3b5e6;

   ///     *  
   ///     *  
   ///   * * *
   ///    *** 
   ///     *  
   ///
   pub const arrow_s_image:u32 = 0x077bf18d;

   ///       *
   ///      * 
   ///   * *  
   ///   **   
   ///   ***  
   ///
   pub const arrow_sw_image:u32 = 0x06eb733b;

   ///     *  
   ///    *   
   ///   *****
   ///    *   
   ///     *  
   ///
   pub const arrow_w_image:u32 = 0x03bbf0bd;

   ///   ***  
   ///   **   
   ///   * *  
   ///      * 
   ///       *
   ///
   pub const arrow_nw_image:u32 = 0x0797e3ec;

   ///        
   ///     *  
   ///    * * 
   ///   *****
   ///        
   ///
   pub const letriangle:u32 = 0x037ffe07;

   ///   *    
   ///   **   
   ///   * *  
   ///   *  * 
   ///   *****
   ///
   pub const letriangle_left:u32 = 0x0783336e;

   ///    * * 
   ///   * * *
   ///    * * 
   ///   * * *
   ///    * * 
   ///
   pub const chessboard_image:u32 = 0x015f0e57;

   ///     *  
   ///    * * 
   ///   *   *
   ///    * * 
   ///     *  
   ///
   pub const diamond_image:u32 = 0x06bbf9ad;

   ///        
   ///     *  
   ///    * * 
   ///     *  
   ///        
   ///
   pub const diamond_small_image:u32 = 0x037ffedf;

   ///   *****
   ///   *   *
   ///   *   *
   ///   *   *
   ///   *****
   ///
   pub const square_image:u32 = 0x05c30970;

   ///        
   ///    *** 
   ///    * * 
   ///    *** 
   ///        
   ///
   pub const square_small_image:u32 = 0x023ffe8f;

   ///   * *  
   ///   * *  
   ///   **** 
   ///   ** * 
   ///   **** 
   ///
   pub const image_rabbit:u32 = 0x034b322c;

   ///   *   *
   ///   *   *
   ///   *****
   ///    *** 
   ///     *  
   ///
   pub const image_cow:u32 = 0x01dbf08a;

   ///     *  
   ///     *  
   ///     *  
   ///   ***  
   ///   ***  
   ///
   pub const music_crotchet_image:u32 = 0x076b771d;

   ///     *  
   ///     ** 
   ///     * *
   ///   ***  
   ///   ***  
   ///
   pub const music_quaver_image:u32 = 0x066b751d;

   ///    ****
   ///    *  *
   ///    *  *
   ///   ** **
   ///   ** **
   ///
   pub const music_quavers_image:u32 = 0x05a70c21;

   ///   * * *
   ///   * * *
   ///   *****
   ///     *  
   ///     *  
   ///
   pub const pitchfork_image:u32 = 0x015bf0d8;

   ///     *  
   ///    *** 
   ///     *  
   ///    *** 
   ///   *****
   ///
   pub const xmas_image:u32 = 0x0623378d;

   ///    ****
   ///   ** * 
   ///   ***  
   ///   **** 
   ///    ****
   ///
   pub const pacman_image:u32 = 0x06930209;

   ///     *  
   ///    *** 
   ///   ** **
   ///    *** 
   ///     *  
   ///
   pub const letarget:u32 = 0x023bf88d;

   ///   ** **
   ///   *****
   ///    *** 
   ///    *** 
   ///    *** 
   ///
   pub const letshirt:u32 = 0x001b068a;

   ///      **
   ///      **
   ///   *****
   ///   *****
   ///    * * 
   ///
   pub const rollerskate_image:u32 = 0x00ff1003;

   ///    **  
   ///   ***  
   ///    ****
   ///    *** 
   ///        
   ///
   pub const duck_image:u32 = 0x031fe48d;

   ///     *  
   ///    *** 
   ///   *****
   ///    *** 
   ///    * * 
   ///
   pub const house_image:u32 = 0x023f308d;

   ///        
   ///    *** 
   ///   *****
   ///    * * 
   ///        
   ///
   pub const letortoise:u32 = 0x023ff0af;

   ///   ** **
   ///   *****
   ///     *  
   ///   *****
   ///   ** **
   ///
   pub const butterfly_image:u32 = 0x04070702;

   ///     *  
   ///   *****
   ///     *  
   ///    * * 
   ///   *   *
   ///
   pub const stickfigure_image:u32 = 0x0407f7ad;

   ///   *****
   ///   * * *
   ///   *****
   ///   *****
   ///   * * *
   ///
   pub const ghost_image:u32 = 0x0143c000;

   ///     *  
   ///     *  
   ///     *  
   ///    *** 
   ///     *  
   ///
   pub const sword_image:u32 = 0x077bf78d;

   ///   **   
   ///    *   
   ///    *   
   ///    *** 
   ///    * * 
   ///
   pub const giraffe_image:u32 = 0x07bf2e8e;

   ///    *** 
   ///   * * *
   ///   *****
   ///    *** 
   ///    *** 
   ///
   pub const skull_image:u32 = 0x015b008d;

   ///    *** 
   ///   *****
   ///     *  
   ///   * *  
   ///    **  
   ///
   pub const umbrella_image:u32 = 0x041b475d;

   ///   **   
   ///   ** **
   ///    * * 
   ///    *** 
   ///        
   ///
   pub const snake_image:u32 = 0x009fee8e;
}