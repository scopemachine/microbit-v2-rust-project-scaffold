#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
use defmt::info;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use microbit::{board::Board, hal::timer::Timer};

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    info!("Hello, micro:bit v2!");

    // The 5x5 LED matrix is scanned: a column pin sinks current (active low)
    // while a row pin sources it (active high). Light just the top-left LED.
    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;

    loop {
        let _ = row1.set_high();
        timer.delay_ms(500);
        let _ = row1.set_low();
        timer.delay_ms(500);
        info!("blink");
    }
}
