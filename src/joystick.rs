use usbd_hid::descriptor::generator_prelude::*;

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
    pub buttons: i8,
}
