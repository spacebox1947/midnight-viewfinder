use crate::peripherals_io::button::Button;
use embassy_time::{Duration, Instant, Timer};
use esp_hal::gpio::Input;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

#[derive(PartialEq, Clone, Copy, Debug, defmt::Format)]
enum ButtonEvent {
    Pressed,
    Held,
    Cleared,
}

struct InputEvent {
    input: Button,
    event: Signal<CriticalSectionRawMutex, ButtonEvent>,
}

impl InputEvent {
    pub fn new(input: Button, event: Signal<CriticalSectionRawMutex, ButtonEvent>) -> Self {
        Self {
            input, 
            event
        }
    }
}
