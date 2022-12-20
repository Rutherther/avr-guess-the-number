use atmega_hal::port::{Pin, mode};

pub struct Sipo {
    srclk : Pin<mode::Output>,
    srclr : Pin<mode::Output>,
    ser : Pin<mode::Output>,
    rclk : Pin<mode::Output>,
}

impl Sipo {
    pub fn create(srclk: Pin<mode::Output>, srclr: Pin<mode::Output>, ser: Pin<mode::Output>, rclk: Pin<mode::Output>) -> Sipo {
        let mut sipo = Sipo {
            srclk,
            srclr,
            ser,
            rclk
        };

        sipo.setup();
        return sipo;
    }

    pub fn setup(&mut self) {
        self.clear();
        self.show();
    }

    pub fn set(&mut self, value: bool) {
        if value {
            self.ser.set_high();
        } else {
            self.ser.set_low();
        }
    }

    pub fn shift(&mut self) {
        self.srclk.set_low();
        self.srclk.set_high();
        self.srclk.set_low();
    }

    pub fn shift_value(&mut self, value: bool) {
        self.set(value);
        self.shift();
    }

    pub fn show(&mut self) {
        self.rclk.set_low();
        self.rclk.set_high();
        self.rclk.set_low();
    }

    pub fn clear(&mut self) {
        self.srclr.set_low();
        self.srclr.set_high();
    }
}
