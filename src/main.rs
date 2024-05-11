#![no_std]
#![no_main]

use microbit_neopixel::*;

use microbit::hal::gpio::Level;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    if let Some(board) = microbit::Board::take() {
        let np = Neopixel::new(board.edge.e00.into_push_pull_output(Level::Low), 241, 3);

        np.set_pixel(0, Color::new(255, 130, 0));
        np.show();

        loop { }
    }

    panic!("End of the program");
}
