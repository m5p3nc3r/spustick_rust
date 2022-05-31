use bsp::hal::{
    gpio::{Function, FunctionConfig, Pin, PinId, ValidPinMode},
    pio::{PIOExt, UninitStateMachine, PIO, SM0},
    timer::CountDown,
    Timer,
};
use embedded_time::rate::Hertz;
//use embedded_hal::timer::CountDown;
use rp_pico as bsp;

// Import the actual crate to handle the Ws2812 protocol:
use ws2812_pio::Ws2812;
// Import useful traits to handle the ws2812 LEDs:
use smart_leds::{brightness, SmartLedsWrite, RGB8};

// Currently 3 consecutive LEDs are driven by this example
// to keep the power draw compatible with USB:
const STRIP_LEN: usize = 2;

pub struct Leds<'a, I, P>
where
    I: PinId,
    P: PIOExt + FunctionConfig,
    Function<P>: ValidPinMode<I>,
    Ws2812<P, SM0, CountDown<'a>, I>:,
{
    leds: [RGB8; STRIP_LEN],
    //    pin: Pin<I, Function<P>>,
    ws: Ws2812<P, SM0, CountDown<'a>, I>,
    animation_speed: f32,
    brightness: u8,
    t: f32,
}

impl<'a, I, P> Leds<'a, I, P>
where
    I: PinId,
    P: PIOExt + FunctionConfig,
    Function<P>: ValidPinMode<I>,
{
    pub fn new(
        mut pio: PIO<P>,
        sm0: UninitStateMachine<(P, SM0)>,
        timer: &'a Timer,
        freq: Hertz,
        pin: Pin<I, Function<P>>,
    ) -> Self {
        let ws = Ws2812::new(
            // Use pin 6 on the Raspberry Pi Pico (which is GPIO4 of the rp2040 chip)
            // for the LED data output:
            pin,
            &mut pio,
            sm0,
            freq,
            timer.count_down(),
        );

        Self {
            leds: [(0, 0, 0).into(); STRIP_LEN],
            //            pin,
            ws,
            animation_speed: 0.1,
            brightness: 64u8,
            t: 0.0,
        }
    }

    pub fn update(&mut self) {
        let sin = rp_pico::hal::rom_data::float_funcs::fsin::ptr();

        for (i, led) in self.leds.iter_mut().enumerate() {
            // An offset to give 3 consecutive LEDs a different color:
            let hue_offs = match i % 3 {
                1 => 0.25,
                2 => 0.5,
                _ => 0.0,
            };

            let sin_11 = sin((self.t + hue_offs) * 2.0 * core::f32::consts::PI);
            // Bring -1..1 sine range to 0..1 range:
            let sin_01 = (sin_11 + 1.0) * 0.5;

            let hue = 360.0 * sin_01;
            let sat = 1.0;
            let val = 1.0;

            let rgb = hsv2rgb_u8(hue, sat, val);
            *led = rgb.into();
        }

        // Here the magic happens and the `leds` buffer is written to the
        // ws2812 LEDs:
        self.ws
            .write(brightness(self.leds.iter().copied(), self.brightness))
            .unwrap();

        // Increase the time counter variable and make sure it
        // stays inbetween 0.0 to 1.0 range:
        self.t += (16.0 / 1000.0) * self.animation_speed;
        while self.t > 1.0 {
            self.t -= 1.0;
        }
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
    )
}
