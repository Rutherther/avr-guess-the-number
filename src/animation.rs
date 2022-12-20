use super::filled_seven_segment;
use super::led_matrix;

const WIN_ANIMATION_MAX_LED_OUTER_STEP: u8 = 5; // max led_step
const WIN_ANIMATION_MAX_LED_STEP: u8 = 4; // add led_step
const WIN_ANIMATION_MAX_LED_INNER_STEP: u16 = 2500; // 10000; // add led_quarter

const HELO_ANIMATION_MAX_INNER_STEP: u16 = 5000; // 20000;
const HELO_ANIMATION_MAX_OUTER_STEP: u16 = 5;

const GUESS_ANIMATION_MAX_STEP: u16 = 3000;
const DIGIT_INCREMENT_ANIMATION_MAX_STEP: u16 = 1000;

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
    pub inner_step: u16,
    pub outer_step: u16,
    pub hidden: bool
}

pub struct WinAnimation {
    pub number: u16,
    pub led_step: u8,
    pub led_quarter: u8,
    pub led_inner: u16,
    pub hidden: bool
}

pub struct GuessAnimation {
    pub step: u16
}

pub struct DigitIncrementAnimation {
    pub digit_index: usize,
    pub step: u16
}

impl DigitIncrementAnimation {
    pub fn create(digit_index: usize) -> DigitIncrementAnimation {
        DigitIncrementAnimation {
            step: 0,
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
        self.step += 1;

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
            step: 0
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
        self.step += 1;

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
    pub fn create(number: u16) -> WinAnimation {
        WinAnimation {
            number,
            led_inner: 0,
            led_quarter: 0,
            led_step: 0,
            hidden: true
        }
    }

    pub fn reset(&mut self, number: u16) {
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
            led_matrix.set(self.led_quarter, self.led_step, true);
        } else if self.led_step == 2 {
            led_matrix.set(self.led_quarter, 0, true);
            led_matrix.set(self.led_quarter, 1, true);
        } else if self.led_step == 3 {
            led_matrix.set(3 - self.led_quarter, 0, true);
            led_matrix.set(3 - self.led_quarter, 1, true);
        } else {
            for i in 0..=self.led_quarter {
                led_matrix.set(i, 0, true);
                led_matrix.set(i, 1, true);
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

        self.led_inner += 1;
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

        self.inner_step += 1;
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
