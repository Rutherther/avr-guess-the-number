#![no_std]
#![no_main]

mod sipo;
mod filled_sipo;
mod seven_segment;
mod filled_seven_segment;
mod led_matrix;
mod animation;
mod button;
mod rng;

use panic_halt as _;

const DIGITS: usize = 4;
const LED_MATRIX_CORRECT_ROW: u8 = 0;
const LED_MATRIX_INCORRECT_POSITION_ROW: u8 = 1;

static mut HELLO_ANIMATION: animation::HelloAnimation = animation::HelloAnimation { inner_step: 0, outer_step: 0, hidden: false };
static mut WIN_ANIMATION: animation::WinAnimation = animation::WinAnimation { number: 0, led_step: 0, led_quarter: 0, led_inner: 0, hidden: true } ;

#[atmega_hal::entry]
fn main() -> ! {
    // PERIPHERALS
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let srclr = pins.pc2.into_output().downgrade();
    let srclk = pins.pc3.into_output().downgrade();
    let rclk = pins.pc4.into_output().downgrade();
    let ser = pins.pc5.into_output().downgrade();

    let shift_register = sipo::Sipo::create(srclk, srclr, ser, rclk);

    let shift_register = filled_sipo::FilledSipo::create(shift_register);
    let seven_segment = seven_segment::SevenSegment::create(4, true, false);
    let seven_segment = filled_seven_segment::FilledSevenSegment::create(seven_segment, shift_register);

    let mut matrix = led_matrix::LEDMatrix::create(4, 2);
    matrix.add_anode(pins.pd3.into_output().downgrade());
    matrix.add_anode(pins.pd2.into_output().downgrade());
    matrix.add_anode(pins.pd1.into_output().downgrade());
    matrix.add_anode(pins.pd0.into_output().downgrade());

    matrix.add_cathode(pins.pd5.into_output().downgrade());
    matrix.add_cathode(pins.pd6.into_output().downgrade());

    let in_1 = pins.pc1.into_pull_up_input().downgrade().forget_imode();
    let in_2 = pins.pb2.into_pull_up_input().downgrade().forget_imode();
    let in_3 = pins.pb1.into_pull_up_input().downgrade().forget_imode();
    let in_4 = pins.pd7.into_pull_up_input().downgrade().forget_imode();
    let in_confirm = pins.pb0.into_pull_up_input().downgrade().forget_imode();

    let btn_1 = button::Button::create(in_1, false);
    let btn_2 = button::Button::create(in_2, false);
    let btn_3 = button::Button::create(in_3, false);
    let btn_4 = button::Button::create(in_4, false);
    let btn_confirm = button::Button::create(in_confirm, false);
    // PERIPHERALS END

    // RNG
    let rng = rng::Rng::init(123, 111, 45);

    // GAME
    let mut game = Game {
        seven_segment,
        led_matrix: matrix,
        state: GameState::Start,
        guessing_number: None,
        current_number: None,
        animation: None,
        rng,
        buttons: [btn_1, btn_2, btn_3, btn_4],
        confirm: btn_confirm
    };

    unsafe {
        HELLO_ANIMATION = animation::HelloAnimation::create();
        WIN_ANIMATION = animation::WinAnimation::create(0);
        game.animation = Some(&mut HELLO_ANIMATION);
    }

    let mut step: u64 = 0;
    loop {
        // Show seven segment, matrix data
        step += 1;
        game.seven_segment.step();
        if step > 50 {
            game.led_matrix.step();
            step = 0;
        }

        for button in game.buttons.iter_mut() {
            button.step();
        }
        game.confirm.step();

        // Animation logic
        if let Some(animation) = &mut game.animation {
            if animation.running() {
                let state = animation.step(&mut game.seven_segment, &mut game.led_matrix);

                if state == animation::AnimationState::End {
                    animation.cleanup(&mut game.seven_segment, &mut game.led_matrix);
                    game.animation = None;
                }
            } else {
                animation.cleanup(&mut game.seven_segment, &mut game.led_matrix);
                game.animation = None;
            }
        }

        game.step();
    }
}

pub struct Game {
    seven_segment: filled_seven_segment::FilledSevenSegment,
    led_matrix: led_matrix::LEDMatrix,
    state: GameState,
    guessing_number: Option<u16>,
    current_number: Option<u16>,
    animation: Option<&'static mut dyn animation::Animation>,
    rng: rng::Rng,
    buttons: [button::Button; 4],
    confirm: button::Button
}

pub enum GameState {
    Start,
    Play,
    Won
}

impl Game {
    pub fn step(&mut self) {
        match self.state {
            GameState::Start | GameState::Won => {
                if self.any_button_pressed() {
                    self.start_new_game();
                }
            },
            GameState::Play => {
                if self.confirm.state() == button::ButtonState::Pressed {
                    if self.current_number == self.guessing_number {
                        self.end_current_game();
                        return;
                    }

                    self.update_led_matrix();
                }

                let mut btns_pressed: [bool; DIGITS] = [false; DIGITS];
                for (i, button) in self.buttons.iter().enumerate() {
                    let state = button.state();
                    btns_pressed[i] = state == button::ButtonState::Pressed;
                }

                for (i, pressed) in btns_pressed.iter().enumerate() {
                    if *pressed {
                        self.increase_digit(DIGITS - 1 - i);
                    }
                }
            }
        }
    }

    fn get_digit(number: u16, digit_index: usize) -> u8 {
        let mut digit = number;
        for _ in 0..digit_index {
            digit /= 10;
        }

        (digit % 10).try_into().unwrap()
    }

    fn update_led_matrix(&mut self) {
        self.led_matrix.clear();
        let current_number = self.current_number.unwrap();
        let guessing_number = self.guessing_number.unwrap();

        let mut current_digits: [u8; DIGITS] = [0, 0, 0, 0];
        let mut guessing_digits: [u8; DIGITS] = [0, 0, 0, 0];


        for i in 0..DIGITS {
            current_digits[i] = Self::get_digit(current_number, i);
            guessing_digits[i] = Self::get_digit(guessing_number, i);
        }

        for j in 0..2 {
            self.led_matrix.set(
                0,
                1,
                false
            );
        }

        for i in 0..DIGITS {
            if current_digits[i] == guessing_digits[i] {
                self.led_matrix.set(
                    i.try_into().unwrap(),
                    0,
                    true
                );
            }
            else {
                for j in 0..DIGITS {
                    if current_digits[j] != guessing_digits[j] && current_digits[i] == guessing_digits[j] {
                        /*self.led_matrix.set(
                            i.try_into().unwrap(),
                            1,
                            true
                        );*/
                    }
                }

            }
        }
    }

    fn increase_digit(&mut self, digit_index: usize) {
        let current_number = self.current_number.unwrap();
        let mut order = 1;
        for _ in 0..digit_index {
            order *= 10;
        }

        let current_digit = Self::get_digit(current_number, digit_index);
        let new_digit: u16 = ((current_digit + 1) % 10).into();

        let trimmed_number = current_number % order;
        let mut new_number = current_number - (current_number % (order*10));
        new_number += new_digit * order + trimmed_number;

        self.current_number = Some(new_number);
        self.seven_segment.set_number(new_number);
    }

    fn end_current_game(&mut self) {
        // TODO: win animation
        //self.animation = None;
        unsafe {
            WIN_ANIMATION.reset(self.guessing_number.unwrap());
            self.animation = Some(&mut WIN_ANIMATION);
        }
        self.cleanup_current_game();
        self.state = GameState::Won;
    }

    fn cleanup_current_game(&mut self) {
        self.guessing_number = None;
        self.current_number = None;
    }

    fn start_new_game(&mut self) {
        if let Some(animation) = &mut self.animation {
            animation.cleanup(&mut self.seven_segment, &mut self.led_matrix);
            self.animation = None;
        }

        self.guessing_number = Some(self.rng.take_u16() % 10000);
        self.current_number = Some(self.guessing_number.unwrap());
        self.seven_segment.set_number(self.current_number.unwrap());
        self.led_matrix.clear();

        self.state = GameState::Play;
    }

    fn any_button_pressed(&mut self) -> bool {
        for btn in self.buttons.iter() {
            let state = btn.state();

            if state == button::ButtonState::Pressed {
                return true;
            }
        }

        self.confirm.state() == button::ButtonState::Pressed
    }
}
