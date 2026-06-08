// in short this code basically works as Fail (Red LED) until training is pressed. Actual version will have a way to detect if a lego is known or not

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{PinDriver, Pull};
use esp_idf_svc::hal::peripherals::Peripherals;
use log::info;

//trying to keep arduino like setup function, pin declerations were to hard to do inside a function
fn setup() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("ESP32 Lego Door Lock Test");
    info!("-------------------------");
    info!("Press train button to simulate training");
    info!("Press access button to test access");
}

fn main() {
    setup();

    let peripherals = Peripherals::take().unwrap();

    //pin definitions
    let train_button = PinDriver::input(peripherals.pins.gpio41, Pull::Floating).unwrap(); // need pull specification because of input pin
    let access_button = PinDriver::input(peripherals.pins.gpio42, Pull::Floating).unwrap(); // in my circuit we have external pull downs so specified .Floating

    let mut pass_led = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let mut fail_led = PinDriver::output(peripherals.pins.gpio1).unwrap();

    pass_led.set_low().unwrap();
    fail_led.set_low().unwrap();
    //pin definition end

    let mut trained: bool = false;

    let mut last_train_button_state: bool = false;
    let mut last_access_button_state: bool = false;

    loop {
        let train_button_state: bool = train_button.is_high();
        let access_button_state: bool = access_button.is_high();

    // train mode
        if train_button_state && !last_train_button_state {
            info!("");
            info!("TRAIN button pressed.");
            info!("Simulating training...");

            trained = true;

            // Training should not leave an LED on
            pass_led.set_low().unwrap();
            fail_led.set_low().unwrap();

            info!("Training complete.");
            info!("A figure is now trained.");
        }

        // ACCESS mode (accept and deny)
        if access_button_state && !last_access_button_state {
            info!("");
            info!("ACCESS button pressed.");
            info!("Checking access...");

            if trained {
                info!("Access granted.");
                info!("PASS LED stays on.");

                pass_led.set_high().unwrap();
                fail_led.set_low().unwrap();
            } else {
                info!("Access denied.");
                info!("No figure trained yet.");
                info!("FAIL LED stays on.");

                pass_led.set_low().unwrap();
                fail_led.set_high().unwrap();
            }
        }

        last_train_button_state = train_button_state;
        last_access_button_state = access_button_state;

        FreeRtos::delay_ms(50);
    }
}