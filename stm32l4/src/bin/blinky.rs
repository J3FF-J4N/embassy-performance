#![no_std]
#![no_main]

// use cortex_m_semihosting::hprintln;
use defmt::{dbg, expect, info, unwrap};
// use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Speed};
use embassy_stm32::{bind_interrupts, mode::Blocking, peripherals, usart::{self, Config as UsartConfig, Uart}};
use embassy_time::Timer;
// extern crate panic_semihosting;
use {defmt_rtt as _, panic_probe as _};
//use panic_semihosting as _;
use defmt_rtt as _;
// use embassy_executor::Spawner;
// use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
// use embassy_nrf::Peripherals;

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

// Declare async tasks
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led: Output<'_> = Output::new(pin, Level::High, Speed::Low);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
        // hprintln!("Test Task").unwrap();
    }
}

#[embassy_executor::task]
async fn blink2(pin: AnyPin) {
    let mut led: Output<'_> = Output::new(pin, Level::High, Speed::Low);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(250).await;
        led.set_low();
        Timer::after_millis(250).await;
        // hprintln!("Test Task").unwrap();
    }
}

#[embassy_executor::task]
async fn uart_task(mut usart: Uart<'static, Blocking>) {
    loop {

        // format_args!("{}", "test");
        
        unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
        unwrap!(usart.blocking_flush());
        // info!("{}", "Test");
        // dbg!(&usart);
        Timer::after_millis(300).await;
    }
}

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let config = UsartConfig::default();
    let usart = unwrap!(Uart::new_blocking(p.USART1, p.PA10, p.PA9, config));

    

    // usart.blocking_write(b"Hello Embassy World!\r\n").unwrap();

    // Spawned tasks run in the background, concurrently.
    unwrap!(spawner.spawn(blink(p.PB3.degrade())));
    unwrap!(spawner.spawn(blink2(p.PB4.degrade())));
    unwrap!(spawner.spawn(uart_task(usart)));

    info!("Init done");

    // let mut button = Input::new(p.P0_11, Pull::Up);
    loop {
        // Asynchronously wait for GPIO events, allowing other tasks
        // to run, or the core to sleep.
        // button.wait_for_low().await;
        // info!("Button pressed!");
        // button.wait_for_high().await;
        
        // hprintln!("Test").unwrap();
        Timer::after_millis(150).await;
    }
}