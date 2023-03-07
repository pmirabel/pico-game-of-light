#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::adc::{Adc, Config};
use embassy_rp::interrupt;
use embassy_rp::pac::rosc::regs::Randombit;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
mod game_grid;
use {defmt_rtt as _, panic_probe as _};

use crate::game_grid::*;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // ---- ----SETUP ---- ---- ----
    // ---- ---- ---- ---- ---- ----
    // Analog input to randomize
    let p = embassy_rp::init(Default::default());

    // Create Game of life boardgame
    let mut gol_board: game_grid::GameGrid = Default::default();

    gol_board.randomize(1.0, 4);
    gol_board.display(true);

    // ---- ----"LOOP"---- ---- ----
    // ---- ---- ---- ---- ---- ----
    unwrap!(spawner.spawn(refresh_gol_board(gol_board, Duration::from_millis(10000))));
}

#[embassy_executor::task]
async fn refresh_gol_board(mut gg: GameGrid, interval: Duration) {
    loop {
        gg.update();
        info!("GOL board updated!");
        gg.display(true);

        Timer::after(interval).await;
    }
}

// async fn read_all_anaolg_inputs(p: embassy_rp::Peripherals)->[u16;4]{
//     let irq = interrupt::take!(ADC_IRQ_FIFO);
//     let mut adc = Adc::new(p.ADC, irq, Config::default());
//     let mut p26 = p.PIN_26;
//     let mut p27 = p.PIN_27;
//     let mut p28 = p.PIN_28;

//     [adc.read(&mut p26).await,
//     adc.read(&mut p27).await ,
//     adc.read(&mut p28).await,
//     adc.read_temperature().await]
// }
