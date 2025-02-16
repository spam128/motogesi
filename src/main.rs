#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::fmt::Write;
use embedded_hal::serial::{Read, Write as SerialWrite};
use nb::block;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    gpio::{gpioc::PC15, Output, PushPull},
    serial::{Config, Serial},
};
use heapless::String;
// Debuging
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

/// Sends an AT command and waits for "OK" response
fn send_at_command<UART: SerialWrite<u8> + Read<u8>>(
    uart: &mut UART,
    command: &str,
) -> Result<(), &'static str> {
    for byte in command.as_bytes() {
        block!(uart.write(*byte)).map_err(|_| "Write error")?;
    }
    block!(uart.write(b'\r')).map_err(|_| "Write error")?;
    block!(uart.write(b'\n')).map_err(|_| "Write error")?;
    
    let mut buffer = [0u8; 64]; // Buffer to store response
    let mut index = 0;
    
    // Read response and check for "OK"
    loop {
        match uart.read() {
            Ok(byte) => {
                buffer[index] = byte;
                index += 1;
                if index >= buffer.len() {
                    return Err("Buffer overflow");
                }
                if buffer.windows(2).any(|w| w == b"OK") {
                    return Ok(());
                }
            }
            Err(nb::Error::WouldBlock) => continue,
            Err(_) => return Err("Read error"),
        }
    }
}

// Configures the ESP8266 baud rate
fn configure_esp8266<UART: SerialWrite<u8> + Read<u8>>(uart: &mut UART) -> Result<(), &'static str> {
    send_at_command(uart, "AT").map_err(|_| "ESP8266 not responding")?;
    send_at_command(uart, "AT+UART_DEF=9600,8,1,0,0").map_err(|_| "Failed to set baud rate")?;
    Ok(())
}

// Connects to a WiFi network
fn connect_to_wifi<UART: SerialWrite<u8> + Read<u8>>(
    uart: &mut UART,
    ssid: &str,
    password: &str,
) -> Result<(), &'static str> {
    send_at_command(uart, "AT+CWMODE=1").map_err(|_| "Failed to set mode")?;

    let mut connect_cmd = String::<64>::new();
    write!(&mut connect_cmd, "AT+CWJAP=\"{}\",\"{}\"", ssid, password)
        .map_err(|_| "String format error")?;

    send_at_command(uart, &connect_cmd).map_err(|_| "Failed to connect to WiFi")?;
    Ok(())
}
// ) -> Result<(), &'static str> {
//     send_at_command(uart, "AT").map_err(|_| "ESP8266 not responding")?;
//     send_at_command(uart, "AT+CWMODE=1").map_err(|_| "Failed to set mode")?;
//
// //     let mut connect_cmd = heapless::String::<64>::new();
//     let mut connect_cmd = String::<64>::new();
//     write!(&mut connect_cmd, "AT+CWJAP=\"{}\",\"{}\"", ssid, password)
//         .map_err(|_| "String format error")?;
//
//     send_at_command(uart, &connect_cmd).map_err(|_| "Failed to connect to WiFi")?;
//     Ok(())
// }

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting Blue Pill debug session");

    // Get access to peripherals
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut afio = dp.AFIO.constrain();

    // clocks configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // Configuration for GPIO
    let mut gpioc = dp.GPIOC.split();
    let mut gpioa = dp.GPIOA.split();

    // PC15 Diod
    let mut led: PC15<Output<PushPull>> = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);
    // Delay configuration 
    let mut delay = cp.SYST.delay(&clocks);

    //Configure UART2 TX and RX
    let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx = gpioa.pa3;
    let mut serial = Serial::new(dp.USART2, (tx, rx), &mut afio.mapr, Config::default().baudrate(115_200.bps()), &clocks);
    //     let (mut tx, mut rx) = serial.split();
    // wifi config
    let ssid = "wifi_name";
    let password = "wifi_password";
    if let Err(e) = configure_esp8266(&mut serial) {
        led.set_high();
        delay.delay_ms(10_000_u16);
        panic!("Failed to configure ESP8266: {}", e);
    }
    //     if let Err(e) = connect_to_wifi(&mut uart, ssid, password) {
    if let Err(e) = connect_to_wifi(&mut serial, ssid, password) {
        led.set_high();
        loop {
            led.toggle();
            delay.delay_ms(5_000_u16);
        //        timer.wait().unwrap(); // Czekamy 500ms
        }        panic!("Failed to connect to WiFi: {}", e);
    }
    
    loop {
        led.toggle();
        delay.delay_ms(1_000_u16);
    //        timer.wait().unwrap(); // Czekamy 500ms
    }
}


