use super::sipo;

pub struct FilledSipo {
    shift_register: sipo::Sipo,
    data: u16,
    update_step: u8
}

impl FilledSipo {
    pub fn create(shift_register: sipo::Sipo) -> FilledSipo {
        FilledSipo {
            shift_register,
            data: 0,
            update_step: 0
        }
    }

    pub fn set_data(&mut self, data: u16) {
        self.data = data;
        self.reset();
    }

    #[inline]
    pub fn push_block(&mut self) {
        while !self.step() {
        }
    }

    pub fn step(&mut self) -> bool {
        self.shift_register.shift_value((self.data >> (15 - self.update_step)) & 1 == 1);

        if self.update_step >= 15 {
            self.update_step = 0;
            self.shift_register.show();
            return true;
        }

        self.update_step += 1;
        return false;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.update_step = 0;
    }

    pub fn clear(&mut self) {
        self.set_data(0);
        self.reset();
    }
}
