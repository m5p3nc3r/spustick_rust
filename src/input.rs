use bsp::hal::gpio::{Pin, PinId, PullUpInput};
use embedded_hal::digital::v2::InputPin;
use rp_pico as bsp;

use crate::report::SpustickReport;
pub struct JoystickInput<UP, DOWN, LEFT, RIGHT>
where
    UP: PinId,
    DOWN: PinId,
    LEFT: PinId,
    RIGHT: PinId,
{
    up: Pin<UP, PullUpInput>,
    down: Pin<DOWN, PullUpInput>,
    left: Pin<LEFT, PullUpInput>,
    right: Pin<RIGHT, PullUpInput>,
}

impl<UP, DOWN, LEFT, RIGHT> JoystickInput<UP, DOWN, LEFT, RIGHT>
where
    UP: PinId,
    DOWN: PinId,
    LEFT: PinId,
    RIGHT: PinId,
{
    pub fn new(
        up: Pin<UP, PullUpInput>,
        down: Pin<DOWN, PullUpInput>,
        left: Pin<LEFT, PullUpInput>,
        right: Pin<RIGHT, PullUpInput>,
    ) -> Self {
        JoystickInput {
            up: up,
            down: down,
            left: left,
            right: right,
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
    }
}
