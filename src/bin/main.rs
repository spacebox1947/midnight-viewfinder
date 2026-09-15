#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

mod peripherals_io;

use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;

use peripherals_io::button;

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

enum ButtonEvent {
    Pressed,
    Held,
    Cleared,
}

static BUTTON_SIG: Signal<CriticalSectionRawMutex, ButtonEvent> = Signal::new();
// ---- ---- ---- [main] ---- ---- ----
#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o unstable-hal -o alloc -o wifi -o embassy -o defmt -o esp-backtrace -o neovim -o stable-x86_64-unknown-linux-gnu

    const DEBOUNCE_MS: u64 = 80;

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let (mut _wifi_controller, _interfaces) =
        esp_radio::wifi::new(peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    // ---- ---- [InputConfiguration] ---- ----
    let input_config = InputConfig::default().with_pull(Pull::Up);
    let button_left = Input::new(peripherals.GPIO10, input_config);
    let button_right = Input::new(peripherals.GPIO11, input_config);
    let encoder_a = Input::new(peripherals.GPIO1, input_config);
    let encoder_b = Input::new(peripherals.GPIO0, input_config);

    // ---- ---- [InputConfiguration] ---- ----

    // ---- ---- [OutputConfiguration] ---- ----
    let led_config = OutputConfig::default().with_pull(Pull::Up);

    let mut led_left = Output::new(peripherals.GPIO2, Level::Low, led_config);
    let led_right = Output::new(peripherals.GPIO3, Level::Low, led_config);

    led_left.set_low();

    // spawner.spawn():
    // - unwrap is okay
    // - should `panic!` if task cannot spawn
    // TODO: research more
    spawner.spawn(encoder_task(encoder_a, encoder_b).unwrap());
    for button in buttons {
        spawner.spawn(button_struct_task(button, led_left).unwrap());
    }
    // spawner.spawn(button_task(button_left, led_left).unwrap());
    // spawner.spawn(button_task(button_right, led_right).unwrap());

    loop {
        // TODO: Replace with sane loop
        // What is convention if all tasks are async, and there's nothing for the loop{}
        Timer::after_secs(1).await;
    }
}

fn clamp(mut x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        x = min;
    } else if x > max {
        x = max;
    }
    x
}

fn clamp_usize(mut x: usize, min: usize, max: usize) -> usize {
    if x < min {
        x = min;
    } else if x > max {
        x = max;
    }
    x
}

// TODO: Impliment generic clamp<T> for usize or f32, or whatever I want it for
// - NOTE: will need some form of PartialEq Trait, most likely.

#[embassy_executor::task]
async fn encoder_task(mut encoder_a: Input<'static>, encoder_b: Input<'static>) {
    let mut delta: f32 = 0.0;
    loop {
        encoder_a.wait_for_any_edge().await;

        let a = encoder_a.is_high();
        let b = encoder_b.is_high();

        delta += if a == b { -0.0001 } else { 0.0001 };

        delta = clamp(delta, -1.0, 1.0);

        info!("Changing Speed! {}", delta);
        //Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn button_struct_task(mut button: button::Button<'static>, mut led: Output<'static>) {
    loop {
        // any button change
        let mut button_sig = BUTTON_SIG.lock().await;
        *shared = shared.call(button::input_state.update_state());

        info!("Button State updated: ");

        // button release
        Timer::after_millis(button.get_debounce().as_millis()).await;
    }
}


#[embassy_executor::task]
async fn button_event(mut button_event: ButtonEvent, button: Input<'static>, mut led: Output<'static>) {
    loop {
        // wait for any edge on the interrupt
        button.wait_for_any_edge().await;

        // await a &mut to ButtonEvent
        // there is no lock on this Signal<CSRMutex, ButtonEvent>
        let mut button_sig = BUTTON_SIG.lock().await;

        match button.is_high() {
            true => {
                match button_sig {
                    ButtonEvent::Pressed => {
                        button_sig = ButtonEvent::Held
                    },
                    ButtonEvent::Held => {
                        button_sig = ButtonEvent::Held
                    },
                    ButtonEvent::Cleared => {
                        button_sig = ButtonEvent::Pressed
                    }
                }
            },
            false => {
                button_sig = ButtonEvent::Cleared;
            }
        }


    }
}

#[embassy_executor::task]
async fn button_task(mut button: Input<'static>, mut led: Output<'static>) {
    let mut result;
    let mut once = true;

    loop {
        button.wait_for_falling_edge().await;

        // button press
        result = button.is_high();
        led.set_low();
        if once {
            info!("Button has been released! {}", result);
            once = false;
        }
        Timer::after_millis(70).await;

        // button release
        button.wait_for_rising_edge().await;
        result = button.is_low();
        led.set_high();
        if !once {
            info!("Button has been pressed! {}", result);
            once = true;
        }
        Timer::after_millis(70).await;
    }
}
