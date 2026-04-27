#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    hal::uarte::{self, Baudrate, Parity},
    Board,
};
use panic_halt as _;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    // Настройка UART (USB-разъем micro:bit)
    let mut serial = uarte::Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    writeln!(serial, "Hello World!").unwrap();

    loop {
        // Программа завершена, уходим в бесконечный цикл
    }
}
