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
mod ws2812;
use {defmt_rtt as _, panic_probe as _};

use smart_leds::RGB8;

use crate::game_grid::*;
use crate::ws2812::*;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

struct GameGridMessage {
    GameGridUpdate: [bool; NUM_LEDS],
}

struct LedStripMessage {
    LedStripUpdate: [RGB8; NUM_LEDS],
}

static GG_SIGNAL: Signal<CriticalSectionRawMutex, GameGridMessage> = Signal::new();
static LED_SIGNAL: Signal<CriticalSectionRawMutex, LedStripMessage> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // ---- ----SETUP ---- ---- ----
    // ---- ---- ---- ---- ---- ----
    let p = embassy_rp::init(Default::default());
    let (_pio0, sm0, _sm1, _sm2, _sm3) = p.PIO0.split();

    // Create Ledstrip
    let ws2812 = Ws2812::new(sm0, p.PIN_8.degrade());

    // ---- ----"LOOP"---- ---- ----
    // ---- ---- ---- ---- ---- ----
    unwrap!(spawner.spawn(refresh_gol_board(Duration::from_millis(5000))));
    unwrap!(spawner.spawn(animate_ledstrip()));
    unwrap!(spawner.spawn(refresh_ledstrip(ws2812)));
}

#[embassy_executor::task]
async fn refresh_gol_board(interval: Duration) {
    // Create Game of life boardgame
    let mut gg: game_grid::GameGrid = Default::default();
    gg.randomize(1.0);
    gg.display(true);

    GG_SIGNAL.signal(GameGridMessage {
        GameGridUpdate: gg.to_bool_arrray(),
    });
    Timer::after(interval).await;

    loop {
        if !gg.update() {
            info!("GOL board updated!");
        } else {
            gg.display(true);
            info!("GOL did not evolve... Randomize it again :)");
            // TODO: store hash and detect cycle through
            gg.randomize(0.3);
        }

        gg.display(false);
        GG_SIGNAL.signal(GameGridMessage {
            GameGridUpdate: gg.to_bool_arrray(),
        });
        Timer::after(interval).await;
    }
}
#[embassy_executor::task]
async fn animate_ledstrip() {
    loop {
        let mut ledstrip_msg: LedStripMessage = LedStripMessage {
        LedStripUpdate: [RGB8::default(); NUM_LEDS],
        };
        // receive gg update
        let  gamegrid_msg = GG_SIGNAL.wait().await;
        // test purpose
        let tmp: [u8; NUM_LEDS] = gamegrid_msg.GameGridUpdate.map(|v| if v { 1 } else { 0 });
        debug!("RECEIVED update of game grid:\n\t\t{}", tmp);

        // do stuff with
        (0..NUM_LEDS).for_each(|led| {
            if gamegrid_msg.GameGridUpdate[led] {
                ledstrip_msg.LedStripUpdate[led] = RGB8::from((15, 0, 0));
            } else {
                ledstrip_msg.LedStripUpdate[led] = RGB8::from((0, 15, 0));
            }
        });
        //signal light ledstrip
        LED_SIGNAL.signal(ledstrip_msg);
    }
}

#[embassy_executor::task]
async fn refresh_ledstrip(mut ws2812: Ws2812<PioInstanceBase<0>, SmInstanceBase<0>>) {
    // Loop forever making RGB values and pushing them out to the WS2812.
    loop {
        // light ledstrip with received value
        ws2812.write(&LED_SIGNAL.wait().await.LedStripUpdate).await;
        //TODO: define minimal sleep value
        Timer::after(Duration::from_millis(500)).await;
    }
}
