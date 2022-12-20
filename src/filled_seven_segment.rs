use super::seven_segment;
use super::filled_sipo;

pub struct FilledSevenSegment {
    seven_segment: seven_segment::SevenSegment,
    sipo: filled_sipo::FilledSipo,
    digits: [Option<u8>; 4],
    hide: u8,
    update_step: usize
}

impl FilledSevenSegment {
    pub fn create(seven_segment: seven_segment::SevenSegment, sipo: filled_sipo::FilledSipo) -> FilledSevenSegment {
        FilledSevenSegment {
            seven_segment,
            sipo,
            digits: [None, None, None, None],
            hide: 0,
            update_step: 0
        }
    }

    fn get_digit(digit: u16, digit_index: usize) -> u8 {
        let mut digit = digit;
        for _ in 0..digit_index {
            digit /= 10;
        }

        return (digit % 10).try_into().unwrap();
    }

    #[inline]
    pub fn hide_digit(&mut self, digit_index: usize) {
        self.hide |= 1 << digit_index;
    }

    #[inline]
    pub fn show_digit(&mut self, digit_index: usize) {
        self.hide &= !(1 << digit_index);
    }

    #[inline]
    pub fn hide_all_digits(&mut self) {
        self.hide = 0xFF;
    }

    #[inline]
    pub fn show_all_digits(&mut self) {
        self.hide = 0;
    }

    pub fn set_digit(&mut self, digit_index: usize, digit: Option<u8>) {
        if digit_index < 4 {
            self.digits[digit_index] = digit;
        }
    }

    pub fn set_number(&mut self, number: u16) {
        for i in 0..4_usize {
            let digit = Self::get_digit(number, i);
            self.digits[i] = Some(digit);
        }
    }

    #[inline]
    pub fn show_number_block(&mut self) {
        while !self.step() {}
    }

    fn fill_digit(&mut self, digit_index: usize) {
        if digit_index > 3 {
            return
        }

        if (self.hide & (1 << digit_index)) != 0 {
            self.sipo.clear();
            return
        }

        if let Some(digit) = self.digits[digit_index] {
            self.seven_segment.fill_digit(&mut self.sipo, digit, digit_index);
        } else {
            self.sipo.clear();
        }
    }

    pub fn step(&mut self) -> bool {
        if self.update_step == 0 {
            self.fill_digit(0);
            self.update_step = 1;
        }

        if self.sipo.step() {
            self.update_step += 1;

            if self.update_step <= self.seven_segment.digits().into() {
                self.fill_digit(self.update_step - 1);
            }
        }

        if self.update_step > self.seven_segment.digits().into() {
            self.update_step = 0;
            return true;
        }

        return false;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.update_step = 0;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.digits = [None, None, None, None];
    }
}
