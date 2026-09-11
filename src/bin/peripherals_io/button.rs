use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Instant};
use esp_hal::gpio::Input;

pub struct Button<'a> {
    pub button: &'a Input<'a>,
    debounce: Duration,
    last: Instant,
    held: bool,
    is_high: bool,
    state: bool,
}

impl<'a> Button<'a> {
    pub fn new(button: &'a Input, debounce: Duration) -> Self {
        Self {
            button,
            debounce,
            last: Instant::now(),
            held: false,
            is_high: true,
            state: false,
        }
    }

    pub fn new_override(button: &'a Input, debounce: Duration, is_high: bool) -> Self {
        // Potentially useful for hiding the fact you can invert button logic.
        Self {
            button,
            debounce,
            last: Instant::now(),
            held: false,
            is_high,
            state: false,
        }
    }

    pub fn set_debounce(&mut self, debounce: Duration) {
        self.debounce = debounce;
    }

    pub fn get_debounce(&self) -> Duration {
        self.debounce
    }

    fn set_held(&mut self, held: bool) {
        self.held = held;
    }

    pub fn get_held(&self) -> bool {
        self.held
    }

    pub fn update(&mut self) {
        self.last = Instant::now();
    }

    pub fn valid_press(&self) -> bool {
        self.last.elapsed() > self.debounce
    }

    fn set_button_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn check_button(&mut self) -> bool {
        CriticalSectionRawMutex
        self.set_button_state(self.button.is_high());

        if self.is_high {
            self.set_button_state(true);
            self.button.is_high()
        } else {
            self.set_button_state(false);
            self.button.is_low()
        }
    }
}
