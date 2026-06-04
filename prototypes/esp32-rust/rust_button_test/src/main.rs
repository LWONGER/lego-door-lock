use esp_idf_svc::hal::delay::FreeRtos;
use log::info;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("ESP32 Lego Door Lock Rust Test");

    loop {
        info!("Main loop running...");
        FreeRtos::delay_ms(1000);
    }
}