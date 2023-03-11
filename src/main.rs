#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_rp::pio::PioInstanceBase;
use embassy_rp::pio::PioPeripheral;
use embassy_rp::pio::SmInstanceBase;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
mod game_grid;
mod ws2812_pio;
use {defmt_rtt as _, panic_probe as _};

use smart_leds::RGB8;

use crate::game_grid::*;
use crate::ws2812_pio::*;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // ---- ----SETUP ---- ---- ----
    // ---- ---- ---- ---- ---- ----
    // Analog input to randomize
    let p = embassy_rp::init(Default::default());

    // Create Game of life boardgame
    let mut gol_board: game_grid::GameGrid = Default::default();
    gol_board.randomize(0.3);
    gol_board.display(true);

    // ---- ----"LOOP"---- ---- ----
    // ---- ---- ---- ---- ---- ----
    unwrap!(spawner.spawn(refresh_gol_board(gol_board, Duration::from_millis(1000))));

    // LED STUFF

    let (_pio0, sm0, _sm1, _sm2, _sm3) = p.PIO0.split();
    let ws2812 = Ws2812::new(sm0, p.PIN_8.degrade());
    unwrap!(spawner.spawn(refresh_ledstrip(ws2812)));
}

#[embassy_executor::task]
async fn refresh_gol_board(mut gg: GameGrid, interval: Duration) {
    loop {
        if !gg.update() {
            info!("GOL board updated!");
        } else {
            gg.display(true);
            info!("GOL did not evolve... Randomize it again :)");
            gg.randomize(0.3);
        }

        gg.display(true);
        Timer::after(interval).await;
    }
}

#[embassy_executor::task]
async fn refresh_ledstrip(mut ws2812: Ws2812<PioInstanceBase<0>, SmInstanceBase<0>>) {
    let mut data = [RGB8::default(); NUM_LEDS];

    // Loop forever making RGB values and pushing them out to the WS2812.
    loop {
        for j in 0..(256 * 5) {
            debug!("New Colors:");
            (0..NUM_LEDS).for_each(|i| {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                debug!("R: {} G: {} B: {}", data[i].r, data[i].g, data[i].b);
            });
            ws2812.write(&data).await;

            Timer::after(Duration::from_micros(5)).await;
        }
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
