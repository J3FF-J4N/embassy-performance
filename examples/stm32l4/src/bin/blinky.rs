#![no_std]
#![no_main]


use defmt::{dbg, expect, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::{bind_interrupts, mode::Blocking, peripherals, usart::{self, Config as UsartConfig, Uart}};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
use defmt_rtt as _;


bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
    USART2 => usart::InterruptHandler<peripherals::USART2>;
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
async fn uart_task(mut usart: Uart<'static, Async>) {
    loop {

        // format_args!("{}", "test");
        
        unwrap!(usart.write(b"Hello Embassy World!\r\n").await);
        unwrap!(usart.flush().await);
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
    // let usart = unwrap!(Uart::new_blocking(p.USART1, p.PA10, p.PA9, config));
    // let usart = Uart::new(p.USART1, p.PA10, p.PA9, Irqs, p.DMA1_CH4, p.DMA1_CH5, config).unwrap();
    let usart = Uart::new(p.USART2, p.PA15, p.PA2, Irqs, p.DMA1_CH7, p.DMA1_CH6, config).unwrap();

    

    // usart.blocking_write(b"Hello Embassy World!\r\n").unwrap();

    // Spawned tasks run in the background, concurrently.
    unwrap!(spawner.spawn(blink(p.PB3.degrade())));
    unwrap!(spawner.spawn(blink2(p.PB4.degrade())));
    unwrap!(spawner.spawn(uart_task(usart)));

    info!("Init done");

    // let mut button = Input::new(p.P0_11, Pull::Up);
    loop {
        Timer::after_millis(150).await;
    }
}