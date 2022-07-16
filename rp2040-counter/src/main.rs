#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;

use embedded_time::duration::*;
use embedded_time::rate::Extensions;

use embedded_hal::timer::CountDown;

use panic_halt as _;

use rp_pico::hal;
use rp_pico::hal::pac;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use ssd1306::{prelude::*, Ssd1306};

pub mod smalltact;

use fmt_buf::FmtBuf;
use smalltact::DirectButton;

/// Entry point to our bare-metal application.
///
#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let inc_btn = DirectButton::new(pins.gpio14.into_pull_up_input());
    let dec_btn = DirectButton::new(pins.gpio15.into_pull_up_input());

    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio16.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio17.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    // Create the I²C display interface:
    let interface = ssd1306::I2CDisplayInterface::new(i2c);

    // Create a driver instance and initialize:
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Create a text style for drawing the font:
    let char_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut delay = timer.count_down();

    let texts: &[(&str, Point)] = &[
        ("Push the ", Point::zero()),
        ("2 buttons ", Point::new(0, 16)),
        ("to start", Point::new(0, 32)),
    ];

    for (text, coord) in texts {
        Text::with_baseline(text, *coord, char_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
    }

    loop {
        if inc_btn.pushed() && dec_btn.pushed() {
            break;
        }
        delay.start(100.milliseconds());
        let _ = nb::block!(delay.wait());

        display.flush().unwrap();
    }

    let mut counter: u16 = 0;

    display.clear();

    display.flush().unwrap();

    loop {
        if inc_btn.pushed() {
            counter += 1;
        }
        if dec_btn.pushed() && counter != 0 {
            counter -= 1;
        }

        let mut fmt_buf = FmtBuf::from(counter);
        fmt_buf.write_str(" ticks").unwrap();

        Text::new(fmt_buf.as_str(), Point::new(20, 35), char_style)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        display.clear();

        delay.start(95.milliseconds());
        let _ = nb::block!(delay.wait());
    }
}
