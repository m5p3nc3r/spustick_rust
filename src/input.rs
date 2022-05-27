use bsp::hal::gpio::{Pin, PinId, PullUpInput};
use embedded_hal::digital::v2::InputPin;
use rp_pico as bsp;

use crate::report::SpustickReport;
pub struct JoystickInput<UP, DOWN, LEFT, RIGHT, B1, B2>
where
    UP: PinId,
    DOWN: PinId,
    LEFT: PinId,
    RIGHT: PinId,
    B1: PinId,
    B2: PinId,
{
    up: Pin<UP, PullUpInput>,
    down: Pin<DOWN, PullUpInput>,
    left: Pin<LEFT, PullUpInput>,
    right: Pin<RIGHT, PullUpInput>,
    b1: Pin<B1, PullUpInput>,
    b2: Pin<B2, PullUpInput>,
}

impl<UP, DOWN, LEFT, RIGHT, B1, B2> JoystickInput<UP, DOWN, LEFT, RIGHT, B1, B2>
where
    UP: PinId,
    DOWN: PinId,
    LEFT: PinId,
    RIGHT: PinId,
    B1: PinId,
    B2: PinId,
{
    pub fn new(
        up: Pin<UP, PullUpInput>,
        down: Pin<DOWN, PullUpInput>,
        left: Pin<LEFT, PullUpInput>,
        right: Pin<RIGHT, PullUpInput>,
        b1: Pin<B1, PullUpInput>,
        b2: Pin<B2, PullUpInput>,
    ) -> Self {
        JoystickInput {
            up: up,
            down: down,
            left: left,
            right: right,
            b1: b1,
            b2: b2,
        }
    }

    pub fn process(&self, report: &mut SpustickReport) {
        // Initialse the report to default values
        // TODO: Should be able to use Default::default()?
        report.x = 0;
        report.y = 0;
        report.buttons = 0;

        // Update the report based on input switches
        if self.up.is_low().unwrap() {
            report.y = i8::MAX;
        }
        if self.down.is_low().unwrap() {
            report.y = i8::MIN + 1;
        }
        if self.left.is_low().unwrap() {
            report.x = i8::MIN + 1;
        }
        if self.right.is_low().unwrap() {
            report.x = i8::MAX;
        }
        if self.b1.is_low().unwrap() {
            report.buttons |= 1 << 0;
        }
        if self.b2.is_low().unwrap() {
            report.buttons |= 1 << 1;
        }
    }
}
