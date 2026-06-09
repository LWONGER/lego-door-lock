use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Input, Output, PinDriver, Pull};
use esp_idf_svc::hal::modem::Modem;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{
    AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi,
};
use log::{error, info, warn};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

// WIFI DETAILS
// change these before running

const WIFI_SSID: &str = "CHANGEME";
const WIFI_PASSWORD: &str = "CHANGEME";

// LAPTOP SERVER DETAILS
// use ipconfig on powershell to find ipv4 address

const LAPTOP_IP: &str = "CHANGEME";
const LAPTOP_PORT: u16 = 12345;

fn setup() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("lego door lock tcp red detection test");
    info!("------------------------------");
    info!("press access button to send ACCESS");
    info!("if the camera sees red then the Green LED will light if it doesnt then the Red LED will light");
}

fn main() -> Result<()> {
    setup();

    let peripherals = Peripherals::take()?;

    // access button on gpio42
    // external pull-down resistor, so floating is fine
    let mut access_button = PinDriver::input(peripherals.pins.gpio42, Pull::Floating)?;

    // led outputs
    let mut pass_led = PinDriver::output(peripherals.pins.gpio2)?;
    let mut fail_led = PinDriver::output(peripherals.pins.gpio1)?;

    // low by default
    pass_led.set_low()?;
    fail_led.set_low()?;

    info!("access button ready");
    info!("pass led ready");
    info!("fail led ready");

    let modem = peripherals.modem;

    // keep wifi stored so the connection stays alive
    let _wifi = connect_to_wifi(modem)?;

    loop {
        info!("connecting to laptop tcp server...");

        match run_tcp_client(&mut access_button, &mut pass_led, &mut fail_led) {
            Ok(()) => {
                warn!("tcp connection ended");
                warn!("trying again in 3 seconds...");
                FreeRtos::delay_ms(3000);
            }
            Err(error) => {
                error!("tcp client error: {:?}", error);
                warn!("trying again in 3 seconds...");
                FreeRtos::delay_ms(3000);
            }
        }
    }
}

fn connect_to_wifi<'d>(modem: Modem<'d>) -> Result<BlockingWifi<EspWifi<'d>>> {
    let system_event_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let wifi_driver = EspWifi::new(modem, system_event_loop.clone(), Some(nvs))?;
    let mut wifi = BlockingWifi::wrap(wifi_driver, system_event_loop)?;

    info!("setting wifi details...");

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap(),
        password: WIFI_PASSWORD.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    info!("starting wifi...");
    wifi.start()?;

    let _ = wifi.disconnect();
    loop {
        info!("connecting to wifi...");

        match wifi.connect() {
            Ok(_) => {
                info!("wifi connect command sent");
            }
            Err(error) => {
                error!("wifi connect failed: {:?}", error);
            }
        }

        info!("waiting for wifi network...");

        match wifi.wait_netif_up() {
            Ok(_) => {
                info!("wifi connected");

                let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
                info!("esp32 ip: {}", ip_info.ip);

                return Ok(wifi);
            }
            Err(error) => {
                error!("wifi wait failed: {:?}", error);
                warn!("trying wifi again in 3 seconds...");

                let _ = wifi.disconnect();
                FreeRtos::delay_ms(3000);
            }
        }
    }
}

fn run_tcp_client(
    access_button: &mut PinDriver<'_, Input>,
    pass_led: &mut PinDriver<'_, Output>,
    fail_led: &mut PinDriver<'_, Output>,
) -> Result<()> {
    let server_address = format!("{}:{}", LAPTOP_IP, LAPTOP_PORT);

    info!("server address: {}", server_address);

    let mut stream = TcpStream::connect(server_address)?;

    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    info!("connected to laptop server");
    info!("waiting for access button...");

    let mut last_access_button_state = false;

    loop {
        let access_button_state = access_button.is_high();

        // only send once when button changes from not pressed to pressed
        if access_button_state && !last_access_button_state {
            info!("access button pressed");
            info!("sending ACCESS...");

            stream.write_all(b"ACCESS\n")?;
            stream.flush()?;

            let reply = read_server_reply(&mut stream)?;

            if reply == "OPEN" {
                info!("access granted");
                info!("pass led on");

                pass_led.set_high()?;
                fail_led.set_low()?;
            } else if reply == "REJECT" {
                info!("access rejected");
                info!("fail led on");

                pass_led.set_low()?;
                fail_led.set_high()?;
            } else {
                warn!("unexpected server reply");

                pass_led.set_low()?;
                fail_led.set_high()?;
            }
        }

        last_access_button_state = access_button_state;

        FreeRtos::delay_ms(50);
    }
}

fn read_server_reply(stream: &mut TcpStream) -> Result<String> {
    let mut buffer = [0_u8; 64];

    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        warn!("server closed the connection");
        return Ok(String::from(""));
    }

    let reply = std::str::from_utf8(&buffer[..bytes_read]).unwrap_or("");
    let reply = reply.trim().to_string();

    info!("from server: {}", reply);

    Ok(reply)
}