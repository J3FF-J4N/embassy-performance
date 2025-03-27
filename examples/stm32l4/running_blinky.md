# Master Project

## Dependencies

Rust (Obviously the compiler)

```bash
curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh
```

Probe-rs (Flashing, Debugging, etc)

```bash
curl --proto '=https' --tlsv1.2 -LsSf <https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh> | sh
```

MCU Target (Can be anything supported by embassy)

```bash
rustup target add thumbv7em-none-eabi
```

## Running Blinky

1. Navigate to embassy/examples/stm32l4/
2. After installing all the dependencies ensure to restart the terminal (and Code if it was running already) to update the environment variables
3. Connect MCU to Host device
4. Run <cargo run --bin blinky> for debug build or <cargo run --bin blinky --release> for the significantly faster and smaller debug build
5. The programming should now start flashing to the device

### Planned Improvment

As of right now only the codebase provided by me should be used as I have not figured out a way to use cargo to automatically gather the required embassy components. This is a big TODO on my list but is super low priority as it does not improve or help me with my thesis. Once I achieve this all the requirements are significantly easier to rettrieve and the latest version of embassy can always be used.

## Modules tested

* ~~ADC~~
* ~~Blinky (LED)~~
* ~~Button ~~
* ~~Button Exti~~
* CAN
* ~~DAC DMA~~
* ~~DAC~~
* ~~I2C Blocking Async~~
* ~~I2C DMA~~
* ~~I2C~~
* MCO
* ~~RNG~~
* ~~RTC~~
* SPE ADIN1110
* SPI Blocking Async
* SPI DMA
* SPI
* TSC Async
* TSC Blocking
* TSC Multipin
* ~~USART DMA~~
* ~~USART~~

## Examples for STM32L4 family

Run individual examples with

```bash
cargo run --bin <module-name>
```

for example

```bash
cargo run --bin blinky
```

![alt text](image.png)

Useful commands for later:

rust-objdump -h target/thumbv7em-none-eabi/release/blinky

cargo size --bin blinky
