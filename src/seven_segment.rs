use super::filled_sipo;

pub struct SevenSegment {
    digits: u8,
    dp: bool,
    common_cathode: bool
}

impl SevenSegment {
    pub fn create(digits: u8, dp: bool, common_cathode: bool) -> SevenSegment {
        SevenSegment {
            digits,
            dp,
            common_cathode
        }
    }

    fn get_digit_segments(digit: u8) -> u8 {
        match digit {
            //     HGFEDCBA
            0 => 0b00111111,   // 0
            1 => 0b00000110,   // 1
            2 => 0b01011011,   // 2
            3 => 0b01001111,   // 3
            4 => 0b01100110,   // 4
            5 => 0b01101101,   // 5
            6 => 0b01111101,   // 6
            7 => 0b00000111,   // 7
            8 => 0b01111111,   // 8
            9 => 0b01101111,   // 9
            65 => 0b01110111,  // A
            98 => 0b01111100,  // b
            67 => 0b00111001,  // C
            99 => 0b01011000,  // c
            100 => 0b01011110, // d
            69 => 0b01111001,  // E
            70 => 0b01110001,  // F
            71 => 0b00111101,  // G
            103 => 0b01101111, // g
            72 => 0b011110110, // H
            104 => 0b01110100, // h
            73 => 0b00000110,  // I
            76 => 0b00111000,  // L
            108 => 0b00110000, // l
            110 => 0b01010100, // n
            78 => 0b00110111, // N
            79 => 0b00111111,  // O
            111 => 0b01011100, // o
            80 => 0b01110011,  // P
            81 => 0b01100111,  // Q
            114 => 0b01010000, // r
            83 => 0b01101101,  // S
            116 => 0b01111000, // t
            85 => 0b00111110,  // U
            _ => 0b00000000,  // nothing
        }
    }

    #[inline]
    pub fn digits(&self) -> u8 {
        return self.digits;
    }

    #[inline]
    pub fn dp(&self) -> bool {
        return self.dp;
    }

    #[inline]
    pub fn common_cathode(&self) -> bool {
        return self.common_cathode;
    }

    #[inline]
    fn get_digit_selector(digit_count: u8, digit_index: usize) -> u8 {
        let digit_count: usize = digit_count.into();
        return 1 << (digit_count - 1 - digit_index);
    }

    pub fn fill_digit(&self, sipo: &mut filled_sipo::FilledSipo, digit: u8, digit_index: usize) -> bool {
        if digit_index >= self.digits.into() {
            return false;
        }

        let mut segments = SevenSegment::get_digit_segments(digit);
        let mut digit_selector = SevenSegment::get_digit_selector(self.digits, digit_index);

        if self.common_cathode {
            digit_selector = !digit_selector;
        } else {
            segments = !segments;
        }

        let segments: u16 = segments.into();
        let digit_selector: u16 = digit_selector.into();

        if self.dp {
            sipo.set_data(digit_selector << 8 | segments);
        } else {
            sipo.set_data(digit_selector << 7 | (segments & 0x7F));
        }
        return true;
    }
}
