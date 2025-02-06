#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    gpio::{gpioc::PC15, Output, PushPull},
    timer::SysDelay,
};

#[entry]
fn main() -> ! {
    // Pobranie dostępu do urządzeń peryferyjnych
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Konfiguracja zegarów
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);

    // Konfiguracja opóźnień na podstawie zegara systemowego
    let mut delay = cp.SYST.delay(&clocks);

    // Podział GPIOC (split() nie przyjmuje argumentów w nowej wersji HAL)
    let mut gpioc = dp.GPIOC.split();

    // Konfiguracja diody LED na PC15 jako wyjście push-pull
    let mut led: PC15<Output<PushPull>> = gpioc.pc15.into_push_pull_output();

    // Główna pętla programu (mruganie LED)
    loop {
        led.set_high(); // Włączenie LED
        delay.delay_ms(500);
        led.set_low();  // Wyłączenie LED
        delay.delay_ms(500);
    }
}

