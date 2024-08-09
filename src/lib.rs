#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use embedded_alloc::Heap;
use defmt_rtt as _;
use panic_halt as _;
use embedded_hal::digital::OutputPin;
use microbit::hal::gpio::{
    Pin,
    Output,
    PushPull,
};

#[global_allocator]
pub static HEAP: Heap = Heap::empty();

static BYTES_PER_PIXEL: u8 = 3;

extern "C" {
    fn sendNeopixelBuffer(pin: u32, data_address: &u8, num_bytes: u16);
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Neopixel {
    pin: Pin<Output<PushPull>>,
    num_pixels: u16,
    data: Vec<u8>,
}

impl Neopixel {
    pub fn new(pin: Pin<Output<PushPull>>, num_pixels: u16) -> Self {

        Self {
            pin,
            num_pixels,
            data: {
                let mut data_buffer: Vec<u8> = vec![0; num_pixels as usize];

                for _ in 0..num_pixels * BYTES_PER_PIXEL as u16 {
                    data_buffer.push(0);
                }

                data_buffer
            },
        }
    }

    pub fn set_pixel(&mut self, pixel_index: u16, color: &Color) {
        let byte_index = pixel_index as usize * BYTES_PER_PIXEL as usize;

        // The reason green is first is because the neopixel strip is grb not rgb
        self.data[byte_index] = color.green;
        self.data[byte_index + 1] = color.red;
        self.data[byte_index + 2] = color.blue;
    }

    pub fn clear_color(&mut self, color: &Color) {
        for pixel in 0..self.num_pixels {
            self.set_pixel(pixel, color);
        }
    }
    
    pub fn show(&mut self) {
        let pin_mask: u32 = 1u32 << self.pin.pin();

        let _ = self.pin.set_low();

        unsafe {
            sendNeopixelBuffer(pin_mask, &self.data[0], BYTES_PER_PIXEL as u16 * self.num_pixels);
        }
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}
