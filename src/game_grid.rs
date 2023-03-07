const WIDTH: usize = 18;
const HEIGHT: usize = 8;

use core::default;

use defmt::*;
use embassy_rp::{clocks::RoscRng, pac::common::W};
use rand_core::RngCore;

pub(crate) struct GameGrid {
    cells: [[bool; WIDTH]; HEIGHT],
    width_i32: i32,
    heigth_i32: i32,
}
impl GameGrid {
    pub(crate) fn update(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];
        (0..HEIGHT).for_each(|y| {
            for x in 0..WIDTH {
                let neighbors = self.count_alive_neighbors(x, y);
                new_cells[y][x] =
                    matches!((self.cells[y][x], neighbors), (true, 2..=3) | (false, 3));
            }
        });
        self.cells = new_cells;
    }

    pub(crate) fn randomize(&mut self, probability_to_live: f32, max_neighbors: u8) {
        debug!(
            "randomize with probability_to_live = {} and max_neighbors = {}",
            probability_to_live, max_neighbors
        );
        let mut random: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
        let mut rng: RoscRng = RoscRng;
        rng.fill_bytes(&mut random);
        let thresh = probability_to_live * u8::MAX as f32;
        (0..HEIGHT).for_each(|y| {
            for x in 0..WIDTH {
                let neighbors = self.count_alive_neighbors(x, y);
                if neighbors > 8 {
                    warn!("alive neighbors > 8 at [y{}][x{}]", y, x);
                }
                self.cells[y][x] = neighbors < max_neighbors && random[y * x] < thresh as u8;
            }
        });
    }

    pub(crate) fn display(&self, display_neighboor: bool) {
        (0..HEIGHT).for_each(|y| {
            let mut tmp : [u8; WIDTH]= [0;WIDTH];
            let mut tmp_ngh : [u8; WIDTH]= [0;WIDTH];
            if display_neighboor {
                (0..WIDTH).for_each(|x| {
                    tmp[x]=if self.cells[y][x] { 1 } else { 0 };
                    tmp_ngh[x] = self.count_alive_neighbors(x, y);
                });
                debug!("{}| NGHB :{}|", tmp,tmp_ngh);

            } else {
                let mut line: [bool; WIDTH] = Default::default();
                line[..WIDTH].copy_from_slice(&self.cells[y][..WIDTH]);
                let tmp: [u8; WIDTH] = line.map(|v| if v { 1 } else { 0 });
                debug!("{}|", tmp);
            }
        });
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in [0, 1, 2] {
            for dx in [0, 1, 2] {
                if dx == 1 && dy == 1 {
                    continue;
                }
                // if let Some(nx) = x + dx.checked_sub(1);
                let nx = i32::try_from(x).unwrap() + dx - 1;
                let ny = i32::try_from(y).unwrap() + dy - 1;
                if nx >= 0
                    && ny >= 0
                    && nx < self.width_i32
                    && ny < self.heigth_i32
                    && self.cells[usize::try_from(ny).unwrap()][usize::try_from(nx).unwrap()]
                {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Default for GameGrid {
    fn default() -> Self {
        let width_i32 = i32::try_from(WIDTH).unwrap();
        let heigth_i32 = i32::try_from(HEIGHT).unwrap();
        GameGrid {
            cells: [[false; WIDTH]; HEIGHT],
            width_i32,
            heigth_i32,
        }
    }
}
