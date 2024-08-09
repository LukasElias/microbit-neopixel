#![no_std]
#![no_main]

use microbit_neopixel as neopixel;

use microbit::hal::gpio::Level;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { neopixel::HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    if let Some(board) = microbit::Board::take() {
        let mut np = neopixel::Neopixel::new(board.edge.e00.into_push_pull_output(Level::Low).into(), 241);

        np.clear_color(&neopixel::Color::new(0, 0, 255));
        np.show();
    }

    panic!("End of the program");
}
