#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0b0001_1000;
const WHOAMI: u8 = 0b0000_0111;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH7,
        Hertz(100_000),
        Default::default(),
    );

    let mut data = [0u8; 1];
    unwrap!(i2c.write_read(ADDRESS, &[WHOAMI], &mut data).await);
    info!("Whoami: {}", data[0]);
}
