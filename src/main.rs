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
mod ledstrip_effect;
mod ws2812;
use {defmt_rtt as _, panic_probe as _};

use cichlid::ColorRGB;
use ledstrip_effect::LedstripColors;
use ledstrip_effect::TRANSITION_STEPS;

use crate::ws2812::*;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

struct GameGridMessage {
    game_grid_update: [bool; NUM_LEDS],
}

struct LedStripMessage {
    led_strip_update: [ColorRGB; NUM_LEDS],
}

static GG_SIGNAL: Signal<CriticalSectionRawMutex, GameGridMessage> = Signal::new();
static LED_SIGNAL: Signal<CriticalSectionRawMutex, LedStripMessage> = Signal::new();

pub(crate) const LEDSTRIP_REFRESH_DELAY: Duration = Duration::from_millis(10);

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
    unwrap!(spawner.spawn(refresh_gol_board(Duration::from_millis(10000))));
    unwrap!(spawner.spawn(animate_ledstrip()));
    unwrap!(spawner.spawn(refresh_ledstrip(ws2812)));
}

#[embassy_executor::task]
async fn refresh_gol_board(interval: Duration) {
    // Create Game of life boardgame
    let mut gg: game_grid::GameGrid = Default::default();
    gg.randomize(0.42);
    gg.display(true);

    GG_SIGNAL.signal(GameGridMessage {
        game_grid_update: gg.to_bool_arrray(),
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
            game_grid_update: gg.to_bool_arrray(),
        });
        Timer::after(interval).await;
    }
}

#[embassy_executor::task]
async fn animate_ledstrip() {
    let ledstrip_colors = LedstripColors::new();
    // plumbing
    let mut gamegrid_msg: GameGridMessage = GameGridMessage {
        game_grid_update: [false; NUM_LEDS],
    };
    let mut prev_gamegrid_msg;

    loop {
        // receive gg update and store last one
        prev_gamegrid_msg = gamegrid_msg;
        gamegrid_msg = GG_SIGNAL.wait().await;

        // test purpose
        let tmp: [u8; NUM_LEDS] = gamegrid_msg.game_grid_update.map(|v| if v { 1 } else { 0 });
        debug!("RECEIVED update of game grid:\n\t\t{}", tmp);

        while !GG_SIGNAL.signaled() {
            // compute new colors
            for cpt in 0..TRANSITION_STEPS {
                let mut ledstrip_msg: LedStripMessage = LedStripMessage {
                    led_strip_update: [ColorRGB::default(); NUM_LEDS],
                };
                for led in 0..NUM_LEDS {
                    match (
                        prev_gamegrid_msg.game_grid_update[led],
                        gamegrid_msg.game_grid_update[led],
                    ) {
                        // Alive --> Alive
                        (true, true) => {
                            ledstrip_msg.led_strip_update[led] =
                                ledstrip_colors.get_color_at(cpt).current_still_alive;
                        }
                        // Alive --> Dead
                        (true, false) => {
                            ledstrip_msg.led_strip_update[led] =
                                ledstrip_colors.get_color_at(cpt).current_alive_to_dead;
                        }
                        // Dead --> Alive
                        (false, true) => {
                            ledstrip_msg.led_strip_update[led] =
                                ledstrip_colors.get_color_at(cpt).current_dead_to_alive;
                        }
                        // Dead --> Dead
                        (false, false) => {
                            ledstrip_msg.led_strip_update[led] =
                                ledstrip_colors.get_color_at(cpt).current_still_dead;
                        }
                    }
                }
                //signal light ledstrip
                LED_SIGNAL.signal(ledstrip_msg);
                Timer::after(LEDSTRIP_REFRESH_DELAY).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn refresh_ledstrip(mut ws2812: Ws2812<PioInstanceBase<0>, SmInstanceBase<0>>) {
    // Loop forever making RGB values and pushing them out to the WS2812.
    loop {
        // light ledstrip with received value
        ws2812
            .write(&LED_SIGNAL.wait().await.led_strip_update)
            .await;
        //TODO: define minimal sleep value
        // Timer::after(Duration::from_millis(500)).await;
    }
}
