#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    gpio::{gpioc::PC15, Output, PushPull},
    timer::SysTimerExt,
};

#[entry]
fn main() -> ! {
    // Pobranie dostępu do peryferiów
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Konfiguracja zegarów
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Konfiguracja GPIO dla PC15 (dioda)
    let mut gpioc = dp.GPIOC.split();
    let mut led: PC15<Output<PushPull>> = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

    // Konfiguracja opóźnienia
    let mut delay = dp.SYST.delay(&clocks);

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}

