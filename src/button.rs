use atmega_hal::port::{Pin, mode};

#[derive(PartialEq, Eq)]
pub enum ButtonState {
    Inactive, // button is not pressed and the state is same from last time
    Active, // button is pressed and the state is same from last time
    Pressed, // The button was just pressed
    Released // The button was just released
}

const DEBOUNCECYCLES: u8 = 50;

pub struct Button {
    input: Pin<mode::Input>,
    active_high: bool,
    last_active: bool,
    active: bool,
    integrator: u8
}

impl Button {
    pub fn create(input: Pin<mode::Input>, active_high: bool) -> Button {
        Button {
            input,
            active_high,
            last_active: false,
            active: false,
            integrator: 0,
        }
    }

    pub fn step(&mut self) {
        let mut btn_active = self.input.is_low();
        if self.active_high {
            btn_active = !btn_active;
        }

        if !btn_active {
            if self.integrator > 0 {
                self.integrator -= 1;
            }
        } else if self.integrator < DEBOUNCECYCLES {
            self.integrator += 1;
        }

        self.active = self.pressed();
    }

    fn pressed(&mut self) -> bool{
        if self.integrator == 0 {
            self.last_active = self.active;
            self.active = false;
        } else if self.integrator >= DEBOUNCECYCLES {
            self.integrator = DEBOUNCECYCLES;
            self.last_active = self.active;
            self.active = true;
        }

        self.active
    }

    pub fn state(&self) -> ButtonState {
        if self.active {
            if self.last_active {
                return ButtonState::Active;
            }

            return ButtonState::Pressed;
        }

        if !self.last_active {
            return ButtonState::Inactive;
        }

        return ButtonState::Released;
    }
}
