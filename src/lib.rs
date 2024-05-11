#![no_std]

extern crate alloc;

use embedded_alloc::Heap;
use defmt_rtt as _;
use panic_halt as _;


use alloc::vec::Vec;
use microbit::hal::gpio::{
    Pin,
    Output,
};

use microbit::hal::prelude::OutputPin;

#[global_allocator]
static HEAP: Heap = Heap::empty();


extern "C" {
    fn sendNeopixelBuffer(pin: u32, data_address: &u8, num_bytes: u16);
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    white: u8,
}

pub struct Neopixel<MODE> {
    pin: Pin<Output<MODE>>,
    num_pixels: u16,
    bytes_per_pixel: u8,
    data: Vec<u8>,
}

impl<MODE> Neopixel<MODE> {
    pub fn new(pin: Pin<Output<MODE>>, num_pixels: u16, bytes_per_pixel: u8) -> Self {
        Self {
            pin,
            num_pixels,
            bytes_per_pixel,
            data: {
                let mut data_buffer: Vec<u8> = Vec::new();

                for _ in 0..num_pixels * bytes_per_pixel as u16 {
                    data_buffer.push(0);
                }

                data_buffer
            },
        }
    }

    pub fn set_pixel(&mut self, pixel_index: u16, color: Color) {
        let byte_index = pixel_index as usize * self.bytes_per_pixel as usize;

        for bytes_per_pixel in 0..self.bytes_per_pixel {
            match bytes_per_pixel {
                0 => self.data[byte_index] = color.red,
                1 => self.data[byte_index + 1] = color.green,
                2 => self.data[byte_index + 2] = color.blue,
                _ => self.data[byte_index + 3] = color.white,
            }
        }
    }
    
    pub fn show(&mut self) {
        let pin_mask: u32 = 1u32 << self.pin.pin();

        let _ = self.pin.set_low();

        unsafe {
            sendNeopixelBuffer(pin_mask, &self.data[0], self.bytes_per_pixel as u16 * self.num_pixels);
        }
    }
}
