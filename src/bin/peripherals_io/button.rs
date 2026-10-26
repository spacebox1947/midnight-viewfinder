use embassy_time::{Duration, Instant, Timer};
use esp_hal::gpio::Input;

pub struct Button {
    pub button: Input<'static>,
    debounce: Duration,
    last: Instant,
    held: bool,
    is_high: bool,
    state: bool,
}

impl Button {
    pub fn new(button: Input<'static>, debounce: Duration) -> Self {
        Self {
            button,
            debounce,
            last: Instant::now(),
            held: false,
            is_high: true,
            state: false,
        }
    }

    pub fn new_override(button: Input<'static>, debounce: Duration, is_high: bool) -> Self {
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

    pub async fn wait_for_press(&mut self) {
        loop {
            self.button.wait_for_falling_edge().await;

            Timer::after(self.debounce).await;

            if self.button.is_low() {
                return;
            }
        }

    }
}
