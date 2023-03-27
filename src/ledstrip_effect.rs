use cichlid::{prelude::*, ColorRGB};

pub(crate) const TRANSITION_STEPS: usize = 50;
pub(crate) const MAX_BRIGHTNESS: u8 = 30;

pub(crate) struct LedstripColors {
    alive_to_dead: [ColorRGB; TRANSITION_STEPS],
    dead_to_alive: [ColorRGB; TRANSITION_STEPS],
    still_alive: [ColorRGB; TRANSITION_STEPS],
    still_dead: [ColorRGB; TRANSITION_STEPS],
}
pub(crate) struct LedstripColor {
    pub(crate) current_alive_to_dead: ColorRGB,
    pub(crate) current_dead_to_alive: ColorRGB,
    pub(crate) current_still_alive: ColorRGB,
    pub(crate) current_still_dead: ColorRGB,
}

impl LedstripColors {
    pub(crate) fn new() -> Self {
        let mut alive_to_dead = [ColorRGB::default(); TRANSITION_STEPS];
        let mut dead_to_alive = [ColorRGB::default(); TRANSITION_STEPS];

        let alive_color = ColorRGB::BlueViolet;
        let dead_color = ColorRGB::Black;
        let start_alive = alive_color;
        let start_dead = dead_color;

        // TODO: animate still cells
        let mut still_alive = [alive_color; TRANSITION_STEPS];
        let mut still_dead = [dead_color; TRANSITION_STEPS];

        alive_to_dead.gradient_fill_rgb_to_inclusive(start_alive, dead_color);
        dead_to_alive.gradient_fill_rgb_to_inclusive(start_dead, alive_color);

        for i in 0..TRANSITION_STEPS {
            alive_to_dead[i].scale(MAX_BRIGHTNESS);
            dead_to_alive[i].scale(MAX_BRIGHTNESS);
            still_alive[i].scale(MAX_BRIGHTNESS);
            still_dead[i].scale(MAX_BRIGHTNESS);
        }
        Self {
            alive_to_dead,
            dead_to_alive,
            still_alive,
            still_dead,
        }
    }

    pub(crate) fn get_color_at(&self, index: usize) -> LedstripColor {
        // TODO: improve this : if we reached end of animation, return last color
        LedstripColor {
            current_alive_to_dead: *self
                .alive_to_dead
                .get(index)
                .unwrap_or(self.alive_to_dead.get(TRANSITION_STEPS - 1).unwrap()),
            current_dead_to_alive: *self
                .dead_to_alive
                .get(index)
                .unwrap_or(self.dead_to_alive.get(TRANSITION_STEPS - 1).unwrap()),
            current_still_alive: *self
                .still_alive
                .get(index)
                .unwrap_or(self.still_alive.get(TRANSITION_STEPS - 1).unwrap()),
            current_still_dead: *self
                .still_dead
                .get(index)
                .unwrap_or(self.still_dead.get(TRANSITION_STEPS - 1).unwrap()),
        }
    }
}

impl Default for LedstripColors {
    fn default() -> Self {
        Self::new()
    }
}
