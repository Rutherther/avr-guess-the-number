use atmega_hal::port::{Pin, mode};

pub struct LEDMatrix {
    width: u8,
    height: u8,
    data: u8,
    anodes: [Option<Pin<mode::Output>>; 8],
    cathodes: [Option<Pin<mode::Output>>; 8],
    anodes_count: usize,
    cathodes_count: usize,
    update_step: usize
}

impl LEDMatrix {
    pub fn create(width: u8, height: u8) -> LEDMatrix {
        LEDMatrix {
            width,
            height,
            data: 0,
            anodes: [None, None, None, None, None, None, None, None],
            cathodes: [None, None, None, None, None, None, None, None],
            anodes_count: 0,
            cathodes_count: 0,
            update_step: 0
        }
    }

    #[inline]
    fn get_position(width: u8, x: u8, y: u8) -> u8 {
        return width*y + x;
    }

    #[inline]
    pub fn data(&self) -> u8 {
        self.data
    }

    #[inline]
    pub fn set_data(&mut self, data: u8) {
        self.data = data;
    }

    pub fn set(&mut self, x: u8, y: u8, value: bool) {
        if x >= self.width || y >= self.height {
            return
        }

        let mask = 1 << Self::get_position(self.width, x, y);
        if value {
            self.data |= mask;
        } else {
            self.data &= !mask;
        }
    }

    #[inline]
    pub fn add_anode(&mut self, anode: Pin<mode::Output>) {
        self.anodes[self.anodes_count] = Some(anode);
        self.anodes_count += 1;
    }

    #[inline]
    pub fn add_cathode(&mut self, cathode: Pin<mode::Output>) {
        self.cathodes[self.cathodes_count] = Some(cathode);
        self.cathodes_count += 1;
    }

    pub fn step(&mut self) -> bool {
        let update_unsigned: u8 = self.update_step.try_into().unwrap();
        let first_position: u8 = update_unsigned << 2; // update_unsigned * self.width ... does not work, WTF!?

        for x in 0..self.cathodes_count {
            let cathode = &mut self.cathodes[x];
            if let Some(cathode) = cathode {
                cathode.set_high();
            }
        }

        let mut any_anode = false;
        for x in 0..self.anodes_count {
            let anode = &mut self.anodes[x];
            if let Some(anode) = anode {
                let x_unsigned: u8 = x.try_into().unwrap();
                if self.data & (1 << (first_position + x_unsigned)) != 0 {
                    anode.set_high();
                    any_anode = true;
                } else {
                    anode.set_low();
                }
            }
        }

        if any_anode {
            if let Some(cathode) = &mut self.cathodes[self.update_step] {
                cathode.set_low();
            }
        }

        self.update_step += 1;
        if self.update_step >= self.height.into() {
            self.update_step = 0;
            return true;
        }

        return false;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data = 0;
    }
}
