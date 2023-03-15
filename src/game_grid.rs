pub(crate) const WIDTH: usize = 17;
pub(crate) const HEIGHT: usize = 8;

use core::hash::{Hash, Hasher};

use defmt::*;
use embassy_rp::clocks::RoscRng;
use rand_core::RngCore;

pub(crate) struct GameGrid {
    cells: [[bool; WIDTH]; HEIGHT],
    width_i32: i32,
    heigth_i32: i32,
}

impl GameGrid {
    pub(crate) fn update(&mut self) -> bool {
        let mut new_cells = [[false; WIDTH]; HEIGHT];
        (0..HEIGHT).for_each(|y| {
            for x in 0..WIDTH {
                let neighbors = self.count_alive_neighbors(x, y);
                new_cells[y][x] =
                    matches!((self.cells[y][x], neighbors), (true, 2..=3) | (false, 3));
            }
        });

        // Iterates over the rows of both arrays using the iter method and compares each element of the rows using the all method, which returns true if all elements of the row are equal. Finally, the function returns true if all rows are equal.
        let changes = new_cells
            .iter()
            .zip(self.cells.iter())
            .all(|(row1, row2)| row1.iter().zip(row2.iter()).all(|(a, b)| a == b));

        self.cells = new_cells;
        changes
    }

    pub(crate) fn randomize(&mut self, probability_to_live: f32) {
        debug!(
            "randomize with probability_to_live = {}",
            probability_to_live
        );
        let mut random: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
        let mut rng: RoscRng = RoscRng;
        rng.fill_bytes(&mut random);
        let thresh = probability_to_live * u8::MAX as f32;
        (0..HEIGHT).for_each(|y| {
            (0..WIDTH).for_each(|x| {
                // let neighbors = self.count_alive_neighbors(x, y);
                self.cells[y][x] = random[y * WIDTH + x] < thresh as u8;
            });
        });
    }

    pub(crate) fn get_hash(&self) -> u64 {
        hash_array(&self.cells)
    }

    pub(crate) fn display(&self, display_neighboor: bool) {
        (0..HEIGHT).for_each(|y| {
            let mut tmp: [u8; WIDTH] = [0; WIDTH];
            let mut tmp_ngh: [u8; WIDTH] = [0; WIDTH];
            if display_neighboor {
                (0..WIDTH).for_each(|x| {
                    tmp[x] = if self.cells[y][x] { 1 } else { 0 };
                    tmp_ngh[x] = self.count_alive_neighbors(x, y);
                });
                debug!("{}| NGHB :{}|", tmp, tmp_ngh);
            } else {
                let mut line: [bool; WIDTH] = Default::default();
                line[..WIDTH].copy_from_slice(&self.cells[y][..WIDTH]);
                let tmp: [u8; WIDTH] = line.map(|v| if v { 1 } else { 0 });
                debug!("{}|", tmp);
            }
        });
        debug!("HASH:{}", self.get_hash())
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

fn hash_array<T: Hash>(arr: &[T]) -> u64 {
    let mut hasher = ArrayHasher::new();
    arr.hash(&mut hasher);
    hasher.finish()
}

struct ArrayHasher {
    state: u64,
}

impl ArrayHasher {
    fn new() -> Self {
        ArrayHasher { state: 0 }
    }
}

impl Hasher for ArrayHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.state = self.state.wrapping_mul(31).wrapping_add(*byte as u64);
        }
    }
}
