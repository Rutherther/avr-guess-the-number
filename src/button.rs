use atmega_hal::port::{Pin, mode};

#[derive(PartialEq, Eq)]
pub enum ButtonState {
    Inactive, // button is not pressed and the state is same from last time
    Active, // button is pressed and the state is same from last time
    Pressed, // The button was just pressed
    Released // The button was just released
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ButtonEvent {
    Click,
    LongClick,
    LongClickContinuous,
    DoubleClick,
    None
}

const DEBOUNCECYCLES: u8 = 50;

const PRESSED_FOR_MAX: u16 = 65000;
const RELEASED_FOR_MAX: u16 = 65000;

const PRESSED_FOR_LONG: u16 = 1000;
const RELEASED_FOR_DOUBLE_CLICK: u16 = 1000;

pub struct Button {
    input: Pin<mode::Input>,
    active_high: bool,
    last_active: bool,
    active: bool,
    integrator: u8,

    pressed_for: u16,
    released_for: u16,

    last_event: ButtonEvent,
}

impl Button {
    pub fn create(input: Pin<mode::Input>, active_high: bool) -> Button {
        Button {
            input,
            active_high,
            last_active: false,
            active: false,
            integrator: 0,
            pressed_for: 0,
            released_for: 0,
            last_event: ButtonEvent::None
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

        let last_active = self.active;
        self.active = self.pressed();

        if !last_active && self.active {
            self.pressed_for = 0;
        } else if last_active && !self.active {
            self.released_for = 0;
        }

        if self.active {
            if self.pressed_for < PRESSED_FOR_MAX {
                self.pressed_for += 1;
            }
        }
        else {
            if self.released_for < RELEASED_FOR_MAX {
                self.released_for += 1;
            }
        }

        // just pressed and was released just for time it could be double click
        // fire double click right away
        // that is different to normal click. Normal clicks fire after release,
        // double click fires right after the second press
        if self.active && self.pressed_for == 1 && self.released_for < RELEASED_FOR_DOUBLE_CLICK {
            self.last_event = ButtonEvent::DoubleClick;
            self.pressed_for += 1;
        }

        // is a long click, fire event until released
        if self.active && self.pressed_for == PRESSED_FOR_LONG {
            self.last_event = ButtonEvent::LongClick;
        }
        else if self.active && self.pressed_for % PRESSED_FOR_LONG == 0 && self.last_event == ButtonEvent::None {
            self.last_event = ButtonEvent::LongClickContinuous;
        }

        // was not long click and period for double click is over
        if self.pressed_for < PRESSED_FOR_LONG && self.pressed_for > 1 && self.released_for == RELEASED_FOR_DOUBLE_CLICK {
            self.last_event = ButtonEvent::Click;
            self.released_for += 1;
        }
    }

    pub fn event(&mut self) -> ButtonEvent {
        let last_event = self.last_event;
        self.last_event = ButtonEvent::None;

        last_event
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
