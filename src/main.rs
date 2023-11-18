#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{
    Input,
    Level,
    Output,
    Pull,
    Speed,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PB3, Level::High, Speed::Low);
    let mut button = ExtiInput::new(
        Input::new(p.PB4, Pull::Down), p.EXTI4
    );

    loop {
        button.wait_for_rising_edge().await;
        match led.get_output_level() {
            Level::High => led.set_low(),
            Level::Low => led.set_high()
        }
    }
}