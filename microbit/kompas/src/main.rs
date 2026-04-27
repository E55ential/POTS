#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use heapless::String;
use libm::atan2f;
use lsm303agr::{Lsm303agr, MagMode, MagOutputDataRate};
use microbit::{
    display::blocking::Display,
    hal::{
        twim::{Frequency, Twim},
        uarte::{Baudrate, Parity, Uarte},
        Delay, Timer,
    },
    Board,
};
use panic_halt as _;

type Image = [[u8; 5]; 5];

// ... (ваши константы N, NE, E и т.д. остаются без изменений)
const N: Image = [
[0, 0, 1, 0, 0],
[0, 1, 1, 1, 0],
[1, 0, 1, 0, 1],
[0, 0, 1, 0, 0],
[0, 0, 1, 0, 0],
];
const NE: Image = [
[0, 1, 1, 1, 1],
[0, 0, 0, 1, 1],
[0, 0, 1, 0, 1],
[0, 1, 0, 0, 1],
[1, 0, 0, 0, 0],
];
const E: Image = [
[0, 0, 1, 0, 0],
[0, 0, 0, 1, 0],
[1, 1, 1, 1, 1],
[0, 0, 0, 1, 0],
[0, 0, 1, 0, 0],
];
const SE: Image = [
[1, 0, 0, 0, 0],
[0, 1, 0, 0, 1],
[0, 0, 1, 0, 1],
[0, 0, 0, 1, 1],
[0, 1, 1, 1, 1],
];
const S: Image = [
[0, 0, 1, 0, 0],
[0, 0, 1, 0, 0],
[1, 0, 1, 0, 1],
[0, 1, 1, 1, 0],
[0, 0, 1, 0, 0],
];
const SW: Image = [
[0, 0, 0, 0, 1],
[1, 0, 0, 1, 0],
[1, 0, 1, 0, 0],
[1, 1, 0, 0, 0],
[1, 1, 1, 1, 0],
];
const W: Image = [
[0, 0, 1, 0, 0],
[0, 1, 0, 0, 0],
[1, 1, 1, 1, 1],
[0, 1, 0, 0, 0],
[0, 0, 1, 0, 0],
];
const NW: Image = [
[1, 1, 1, 1, 0],
[1, 1, 0, 0, 0],
[1, 0, 1, 0, 0],
[1, 0, 0, 1, 0],
[1, 0, 0, 0, 1],
];

fn arrow_for_heading(deg: i32) -> Image {
    match ((deg + 22) % 360) / 45 {
        0 => N,
        1 => NE,
        2 => E,
        3 => SE,
        4 => S,
        5 => SW,
        6 => W,
        _ => NW,
    }
}

fn heading_deg(x: i32, y: i32) -> i32 {
    // Применяем вычисленные корректировки
    let x_corr = x as f32 - (-162675.0);
    let y_corr = y as f32 - (-2925.0);

    // Используем atan2f(y, x) — это стандарт
    // В зависимости от того, как вы держите плату, 
    // возможно придется поменять x и y местами или инвертировать знак
    let mut angle = atan2f(y_corr, x_corr) * 180.0 / core::f32::consts::PI;

    if angle < 0.0 {
        angle += 360.0;
    }
    angle as i32
}


#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut delay = Delay::new(board.SYST);
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // Оставляем только ОДИН экземпляр UART
    let mut uart = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    let i2c = Twim::new(
        board.TWIM0, // Или TWIM1, на v2 обычно используется TWIM0 для внутренних сенсоров
        board.i2c_internal.into(),
        Frequency::K100,
    );


    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_mag_mode_and_odr(&mut delay, MagMode::HighResolution, MagOutputDataRate::Hz10)
        .unwrap();

    let _ = writeln!(uart, "LSM303AGR compass started\r\n");

    let mut heading = 0i32;

    loop {
    // Временно убираем проверку xyz_new_data()
    if let Ok(mag) = sensor.magnetic_field() {
        let x = mag.x_nt();
        let y = mag.y_nt();
        let z = mag.z_nt();
        
        heading = heading_deg(x, y);

        let mut msg: String<96> = String::new();
        let _ = writeln!(&mut msg, "X:{} Y:{} Z:{}\r\n", x, y, z);
        let _ = uart.write(msg.as_bytes());
        }
    
    display.show(&mut timer, arrow_for_heading(heading), 100);
    delay.delay_ms(100); // Уменьшите задержку для теста
    }

}
