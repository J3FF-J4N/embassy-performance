#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};

const ADDRESS: u8 = 0b0001_1000;
const WHOAMI: u8 = 0b0000_0111;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut i2c = I2c::new_blocking(p.I2C1, p.PB6, p.PB7, Hertz(100_000), Default::default());

    let mut data = [0u8; 1];
    unwrap!(i2c.blocking_write_read(ADDRESS, &[WHOAMI], &mut data));
   
    

    info!("Whoami: {:#010b}", data[0]);
}
