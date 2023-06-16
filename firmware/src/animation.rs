use super::filled_seven_segment;
use super::led_matrix;

const WIN_ANIMATION_MAX_LED_OUTER_STEP: u8 = 5; // max led_step
const WIN_ANIMATION_MAX_LED_STEP: u8 = 4; // add led_step
const WIN_ANIMATION_MAX_LED_INNER_STEP: u8 = 10; // multiplied by 256 internally // add led_quarter

const HELO_ANIMATION_MAX_INNER_STEP: u8 = 20; // multiplied by 256 internally
const HELO_ANIMATION_MAX_OUTER_STEP: u8 = 5;

const GUESS_ANIMATION_MAX_STEP: u8 = 12; // multiplied by 256 internally
const DIGIT_INCREMENT_ANIMATION_MAX_STEP: u8 = 4; // multiplied by 256 internally

#[derive(PartialEq, Eq)]
pub enum AnimationState {
    Running,
    End
}

pub trait Animation {
    fn step(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, led_matrix: &mut led_matrix::LEDMatrix) -> AnimationState;
    fn cleanup(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, led_matrix: &mut led_matrix::LEDMatrix);
    fn running(&self) -> bool;
}

pub struct HelloAnimation {
    pub inner_step: u8,
    pub outer_step: u8,
    pub hidden: bool,
    pub internal_step: u8,
}

pub struct WinAnimation {
    pub number: [u8; 4],
    pub led_step: u8,
    pub led_quarter: u8,
    pub led_inner: u8,
    pub hidden: bool,
    pub internal_step: u8,
}

pub struct GuessAnimation {
    pub step: u8,
    pub internal_step: u8,
}

pub struct DigitIncrementAnimation {
    pub digit_index: usize,
    pub step: u8,
    pub internal_step: u8,
}

impl DigitIncrementAnimation {
    pub fn create(digit_index: usize) -> DigitIncrementAnimation {
        DigitIncrementAnimation {
            step: 0,
            internal_step: 0,
            digit_index
        }
    }

    pub fn reset(&mut self, digit_index: usize) {
        self.digit_index = digit_index;
        self.step = 0;
    }
}

impl Animation for DigitIncrementAnimation {
    fn step(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, _: &mut led_matrix::LEDMatrix) -> AnimationState {
        if !self.running() {
            return AnimationState::End;
        }

        if self.step == 0 {
            seven_segment.hide_digit(self.digit_index);
        }

        self.internal_step += 1;
        if self.internal_step == 255 {
            self.step += 1;
            self.internal_step = 0;
        }

        AnimationState::Running
    }

    fn cleanup(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, _: &mut led_matrix::LEDMatrix) {
        seven_segment.show_digit(self.digit_index);
    }

    fn running(&self) -> bool {
        self.step < DIGIT_INCREMENT_ANIMATION_MAX_STEP
    }
}

impl GuessAnimation {
    pub fn create() -> GuessAnimation {
        GuessAnimation {
            step: 0,
            internal_step: 0
        }
    }

    pub fn reset(&mut self) {
        self.step = 0;
    }
}

impl Animation for GuessAnimation {
    fn step(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, _: &mut led_matrix::LEDMatrix) -> AnimationState {
        if !self.running() {
            return AnimationState::End;
        }

        if self.step == 0 {
            seven_segment.hide_all_digits();
        }

        self.internal_step += 1;
        if self.internal_step == 255 {
            self.step += 1;
            self.internal_step = 0;
        }

        AnimationState::Running
    }

    fn cleanup(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, _: &mut led_matrix::LEDMatrix) {
        seven_segment.show_all_digits();
    }

    fn running(&self) -> bool {
        self.step < GUESS_ANIMATION_MAX_STEP
    }
}

impl WinAnimation {
    pub fn create(number: [u8; 4]) -> WinAnimation {
        WinAnimation {
            number,
            led_inner: 0,
            led_quarter: 0,
            led_step: 0,
            hidden: true,
            internal_step: 0
        }
    }

    pub fn reset(&mut self, number: [u8; 4]) {
        self.number = number;
        self.led_step = 0;
        self.led_quarter = 0;
        self.led_inner = 0;
        self.hidden = true;
    }
}

impl Animation for WinAnimation {
    fn step(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, led_matrix: &mut led_matrix::LEDMatrix) -> AnimationState {
        if self.led_inner == 0 && self.led_quarter == 0 && self.led_step == 0 {
            seven_segment.set_number(self.number);
            led_matrix.clear();
        }

        if self.led_inner > WIN_ANIMATION_MAX_LED_INNER_STEP {
            self.led_inner = 0;
            self.led_quarter += 1;
        }

        if self.led_quarter >= WIN_ANIMATION_MAX_LED_STEP {
            self.led_quarter = 0;
            self.led_step += 1;
        }

        if self.led_step > WIN_ANIMATION_MAX_LED_OUTER_STEP {
            return AnimationState::End;
        }

        led_matrix.clear();

        if self.led_step < 2 {
            led_matrix.set(self.led_quarter, self.led_step);
        } else if self.led_step == 2 {
            led_matrix.set(self.led_quarter, 0);
            led_matrix.set(self.led_quarter, 1);
        } else if self.led_step == 3 {
            led_matrix.set(3 - self.led_quarter, 0);
            led_matrix.set(3 - self.led_quarter, 1);
        } else {
            for i in 0..=self.led_quarter {
                led_matrix.set(i, 0);
                led_matrix.set(i, 1);
            }
        }

        if (self.led_quarter == 2 || self.led_quarter == 0) && self.led_inner == 0 {
            if self.hidden {
                seven_segment.show_all_digits();
            } else {
                seven_segment.hide_all_digits();
            }

            self.hidden = !self.hidden;
        }

        self.internal_step += 1;
        if self.internal_step == 255 {
            self.led_inner += 1;
            self.internal_step = 0;
        }
        AnimationState::Running
    }

    fn cleanup(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, led_matrix: &mut led_matrix::LEDMatrix) {
        led_matrix.set_data(0xFF);
        seven_segment.show_all_digits();
    }

    #[inline]
    fn running(&self) -> bool {
        self.led_step < WIN_ANIMATION_MAX_LED_OUTER_STEP
    }
}

impl HelloAnimation {
    pub fn create() -> HelloAnimation {
        HelloAnimation {
            inner_step: 0,
            outer_step: 0,
            internal_step: 0,
            hidden: false
        }
    }
}

impl Animation for HelloAnimation {

    fn step(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, led_matrix: &mut led_matrix::LEDMatrix) -> AnimationState {
        // Helo text
        if self.inner_step == 0 && self.outer_step == 0 {
            seven_segment.set_digit(3, Some(72)); // H
            seven_segment.set_digit(2, Some(69)); // E
            seven_segment.set_digit(1, Some(76)); // L
            seven_segment.set_digit(0, Some(79)); // O
            led_matrix.set_data(0xFF);
        }

        if self.inner_step >= HELO_ANIMATION_MAX_INNER_STEP {
            self.inner_step = 0;
            self.outer_step += 1;

            let matrix_data = led_matrix.data();
            led_matrix.set_data(!matrix_data);

            if self.hidden {
                seven_segment.show_all_digits();
            } else {
                seven_segment.hide_all_digits();
            }
            self.hidden = !self.hidden;
        }

        if self.outer_step == HELO_ANIMATION_MAX_OUTER_STEP {
            return AnimationState::End
        }

        self.internal_step += 1;
        if self.internal_step == 255 {
            self.inner_step += 1;
            self.internal_step = 0;
        }
        AnimationState::Running
    }

    fn cleanup(&mut self, seven_segment: &mut filled_seven_segment::FilledSevenSegment, led_matrix: &mut led_matrix::LEDMatrix) {
        led_matrix.clear();
        seven_segment.show_all_digits();
    }

    #[inline]
    fn running(&self) -> bool {
        self.outer_step < HELO_ANIMATION_MAX_OUTER_STEP
    }
}
