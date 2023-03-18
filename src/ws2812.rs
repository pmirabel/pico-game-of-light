use embassy_rp::gpio::{self};
use embassy_rp::pio::{
    FifoJoin, PioInstance, PioStateMachine, PioStateMachineInstance, ShiftDirection, SmInstance,
};
use embassy_rp::pio_instr_util;
use embassy_rp::relocate::RelocatedProgram;
use cichlid::ColorRGB;

use {defmt_rtt as _, panic_probe as _};

/// The number of LEDs in the strip
pub(crate) const NUM_LEDS: usize = 136;

/// Represents a RP2040 PIO controlled WS2812 LED strip
pub struct Ws2812<P: PioInstance, S: SmInstance> {
    sm: PioStateMachineInstance<P, S>,
}

impl<P: PioInstance, S: SmInstance> Ws2812<P, S> {
    /// Creates a new WS2812 LED strip.
    ///
    /// # Arguments
    ///
    /// * `sm`: The PIO state machine to use for controlling the strip.
    /// * `pin`: The GPIO pin that the strip is connected to.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = embassy_rp::init(Default::default());
    /// let (_pio0, sm0, _sm1, _sm2, _sm3) = p.PIO0.split();
    /// // Create Ledstrip
    /// let ws2812 = Ws2812::new(sm0, p.PIN_8.degrade());
    /// ```
    pub fn new(mut sm: PioStateMachineInstance<P, S>, pin: gpio::AnyPin) -> Self {
        // prepare the PIO program
        let side_set = pio::SideSet::new(false, 1, false);
        let mut a: pio::Assembler<32> = pio::Assembler::new_with_side_set(side_set);

        const T1: u8 = 2; // start bit
        const T2: u8 = 5; // data bit
        const T3: u8 = 3; // stop bit
        const CYCLES_PER_BIT: u32 = (T1 + T2 + T3) as u32;

        let mut wrap_target = a.label();
        let mut wrap_source = a.label();
        let mut do_zero = a.label();
        a.set_with_side_set(pio::SetDestination::PINDIRS, 1, 0);
        a.bind(&mut wrap_target);
        // Do stop bit
        a.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 - 1, 0);
        // Do start bit
        a.jmp_with_delay_and_side_set(pio::JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);
        // Do data bit = 1
        a.jmp_with_delay_and_side_set(pio::JmpCondition::Always, &mut wrap_target, T2 - 1, 1);
        a.bind(&mut do_zero);
        // Do data bit = 0
        a.nop_with_delay_and_side_set(T2 - 1, 0);
        a.bind(&mut wrap_source);

        let prg = a.assemble_with_wrap(wrap_source, wrap_target);

        let relocated = RelocatedProgram::new(&prg);
        sm.write_instr(relocated.origin() as usize, relocated.code());
        pio_instr_util::exec_jmp(&mut sm, relocated.origin());

        // Pin config
        let out_pin = sm.make_pio_pin(pin);
        sm.set_set_pins(&[&out_pin]);
        sm.set_sideset_base_pin(&out_pin);
        sm.set_sideset_count(1);

        // Clock config
        // TODO CLOCK_FREQ should come from embassy_rp
        const CLOCK_FREQ: u32 = 125_000_000;
        const WS2812_FREQ: u32 = 800_000;

        let bit_freq = WS2812_FREQ * CYCLES_PER_BIT;
        let mut int = CLOCK_FREQ / bit_freq;
        let rem = CLOCK_FREQ - (int * bit_freq);
        let frac = (rem * 256) / bit_freq;
        // 65536.0 is represented as 0 in the pio's clock divider
        if int == 65536 {
            int = 0;
        }

        sm.set_clkdiv((int << 8) | frac);
        let pio::Wrap { source, target } = relocated.wrap();
        sm.set_wrap(source, target);

        // FIFO config
        sm.set_autopull(true);
        sm.set_fifo_join(FifoJoin::TxOnly);
        sm.set_pull_threshold(24);
        sm.set_out_shift_dir(ShiftDirection::Left);

        sm.set_enable(true);

        Self { sm }
    }

    /// This method writes the provided sequence of RGB colors to the LED strip.
    ///
    /// # Arguments
    ///
    /// * `colors`: The sequence of RGB colors to write.
    pub async fn write(&mut self, colors: &[ColorRGB]) {
        for color in colors {
            let word =
                (u32::from(color.g) << 24) | (u32::from(color.r) << 16) | (u32::from(color.b) << 8);

            self.sm.wait_push(word).await;
        }

        // I missed a bit from your first post, this doesn't implement the SmartLedsWrite trait.
        // I have a function called write that does (nearly*) the same thing,
        // but if you're using some other crate on top, this won't meet the trait bounds. The
        // fundamental problem is that the trait is blocking, and this is async.

        // If you wanted to brute force it, you could impl the trait with block_on around the async bits.
        // Something like this:
        // for color in colors {
        //     let word = (u32::from(color.g) << 24) | (u32::from(color.r) << 16) | (u32::from(color.b) << 8);
        //     block_on(self.sm.wait_push(word).await);
        // }

        // In that case, if the only thing you're doing is rendering something and writing it to a string of ws2812s,
        // then you'll only block if the render is faster than the pio.
        // Of course if you're doing other tasks then you'll also stall them whenever the renderer outpaces the pio.
        // A workaround in the case of the 2040 would be to make use of the second core to handle rendering.

        // The better path forward would be to introduce an async version of the SmartLedsWrite trait.
        // The downside being the function color problem meaning that anything using said trait also needs to be async.
    }
}
