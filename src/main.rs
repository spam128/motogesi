#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    gpio::{gpioc::PC15, Output, PushPull},
//    timer::SysTimerExt,
//    timer::Timer,

};

#[entry]
fn main() -> ! {
    // Pobranie dostępu do peryferiów
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Konfiguracja zegarów
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Konfiguracja GPIO dla PC15 (dioda)
    let mut gpioc = dp.GPIOC.split();
    let mut led: PC15<Output<PushPull>> = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

    // Konfiguracja opóźnienia
  //  let mut delay = dp.SYST.delay(&clocks);
    let mut delay = cp.SYST.delay(&clocks);

    // Tworzymy sprzętowy timer opóźnień (TIM3)
//    let mut timer = Timer::tim3(dp.TIM3, &clocks, &mut rcc.apb1).start_count_down(500.millis());

    loop {
        led.toggle();
        delay.delay_ms(1_000_u16);
//        timer.wait().unwrap(); // Czekamy 500ms
    }
}

