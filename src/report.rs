use usbd_hid::descriptor::generator_prelude::*;

#[derive(Default)]
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (usage_page = GENERIC_DESKTOP,) = {
                (usage = X,) = {
                    #[item_settings data,variable,absolute] x=input;
                };
                (usage = Y,) = {
                    #[item_settings data,variable,absolute] y=input;
                }
            };
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
            };
        }
    }
)]
pub struct SpustickReport {
    pub x: i8,
    pub y: i8,
    buttons: u8,
}

impl SpustickReport {
    pub fn set_button(&mut self, i: u8, val: bool) {
        self.buttons |= (val as u8) << i;
    }

    // pub fn get_button(self, i: u8) -> bool {
    //     (self.buttons >> i & 0x1) == 0x01
    // }
}
