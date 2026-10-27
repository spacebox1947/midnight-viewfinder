use embassy_time::{Duration, Instant, Timer};
use esp_hal::gpio::Input;

pub struct Button {
    button: Input<'static>,
    debounce: Duration,
    pull_high: bool,
}

impl Button {
    pub fn default(button: Input<'static>, debounce: Duration) -> Self {
        Self {
            button,
            debounce,
            pull_high: false,
        }
    }

    pub fn new(button: Input<'static>, debounce: Duration, pull_high: bool) -> Self {
        Self {
            button,
            debounce,
            pull_high
        }
    }

    pub fn set_debounce(&mut self, debounce: Duration) {
        self.debounce = debounce;
    }

    pub fn get_debounce(&self) -> Duration {
        self.debounce
    }

    pub fn set_pull_high(&mut self, pull_high: bool) {
        self.pull_high = pull_high;
    }

    pub async fn wait_for_press(&mut self) {
        loop {
            match self.pull_high {
                true => {
                    self.button.wait_for_low().await;
                },
                false => {
                    self.button.wait_for_high().await;
                }
            };
            Timer::after(self.debounce).await;
        }
    }

    pub async fn wait_for_release(&mut self) {
        loop {
            match self.pull_high {
                true => {
                    self.button.wait_for_high().await;
                },
                false => {
                    self.button.wait_for_low().await;
                }
            };
            Timer::after(self.debounce).await;
        }
    }
}
