#![no_std]
#![no_main]


use defmt::{info, println, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::rcc::{Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv, PllRDiv, PllSource, Sysclk};
use embassy_stm32::Config;
use embassy_stm32::{bind_interrupts, peripherals, usart::{self, Config as UsartConfig, Uart}};
use embassy_time::Timer;
use heapless::Vec;
use panic_probe as _;
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
    let mut led: Output<'static> = Output::new(pin, Level::High, Speed::Low);

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
        
        unwrap!(usart.write(b"Hello Embassy World!\r\n").await);
        unwrap!(usart.flush().await);
        // info!("{}", "Test");
        // dbg!(&usart);
        Timer::after_millis(2000).await;
    }
}

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    //Set up the system clock for 80MHz
    let mut config = Config::default();
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.pll = Some(Pll{source: PllSource::MSI, prediv: PllPreDiv::DIV1, mul: PllMul::MUL40, divp: Some(PllPDiv::DIV7), divq: Some(PllQDiv::DIV2), divr: Some(PllRDiv::DIV2)});
    //---------------------------------

    let p = embassy_stm32::init(config);

    let config = UsartConfig::default();



    

    let mut usart = Uart::new(p.USART2, p.PA15, p.PA2, Irqs, p.DMA1_CH7, p.DMA1_CH6, config).unwrap();




    
    let mut x: [u8; 5] = [0; 5]; //Size fixed all data initialised
    *x.last_mut().unwrap() = 1; //Modify element at index 41

    let mut y: Vec<u8, 5> = Vec::new(); //Size fixed to a given maximum, data initialised once appended
    y.push(1).unwrap(); //Add an element to the end of the collection
    *y.last_mut().unwrap() = 2; //Modify element at index 0


    println!("Array length: {}", x.len());
    println!("Array content: {}", x);
    println!("Vector length: {}", y.len());
    println!("Vector content: {}", y);

    // Array length: 5
    // Array content: [0, 0, 0, 0, 1]
    // Vector length: 1
    // Vector content: [2]

    loop {
        
    }

    // Spawned tasks run in the background, concurrently.
    unwrap!(spawner.spawn(blink(p.PB3.degrade())));
    unwrap!(spawner.spawn(blink2(p.PB4.degrade())));
    unwrap!(spawner.spawn(uart_task(usart)));

    info!("Init done");

    // let mut button = Input::new(p.P0_11, Pull::Up);
    let mut idx: u16 = 0;
    loop {
        // Timer::after_millis(5000).await;
        idx = idx.overflowing_add(1).0;
        info!("{}", idx);
    }
}