#![no_std]
#![no_main]


use defmt::*;
use embassy_stm32::dac::{DacCh1, Value};
use embassy_stm32::dma::NoDma;
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::adc::{Adc, Resolution};
use embassy_stm32::Config;


// use alloc::string::String;
// extern crate alloc;

#[cortex_m_rt::entry]
fn main() -> ! {

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux.adcsel = mux::Adcsel::SYS;
    }


    let p = embassy_stm32::init(config);
    info!("Hello World!");
    
    let mut adc = Adc::new(p.ADC1);
    info!("ADC Init Done");

    let mut dac = DacCh1::new(p.DAC1, NoDma, p.PA4);
    info!("DAC Init Done");


    // adc.enable_vref();
    adc.set_resolution(Resolution::BITS8);

    
    let mut channel = p.PA0;

        loop {
            for dac_val in 0..=255 {
                dac.set(Value::Bit8(to_sine_wave(dac_val)));
                let adc_val = adc.blocking_read(&mut channel);
    
                // Print ASCII bar
                print_bar(adc_val);
                println!("");
                // println!("{}", adc_val); // Newline after each bar
            }
        }

    // loop {

    //     for v in 0..=255 {
    //         dac.set(Value::Bit8(to_sine_wave(v)));
    //         let v = adc.blocking_read(&mut channel);
    //         // info!("--> {}", v);
    //         println!("{}", v)
    //     }


    // }
}

use micromath::F32Ext;

fn to_sine_wave(v: u8) -> u8 {
    if v >= 128 {
        // top half
        let r = 3.14 * ((v - 128) as f32 / 128.0);
        (r.sin() * 128.0 + 127.0) as u8
    } else {
        // bottom half
        let r = 3.14 + 3.14 * (v as f32 / 128.0);
        (r.sin() * 128.0 + 127.0) as u8
    }
}



fn print_bar(value: u16) {
    let bar_length = (value as f32 / 255.0 * 20.0) as usize; // Scale to 20 characters wide

    let mut bar_string: [char; 20] = [' '; 20]; // Use String for building the bar

    // info!("{}", bar_length);

    for idx in 0..bar_length {
        // bar_string.push(); // Add block character
        *bar_string.get_mut(idx).unwrap() = '█';
    }
    for idx in bar_length..20 {
        // bar_string.push(' ');
        *bar_string.get_mut(idx).unwrap() = ' ';
    }

    

    println!("{:#}", bar_string);
}