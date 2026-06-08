use anyhow::Result;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{Input, PinDriver, Pull};
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
const WIFI_SSID: &str = "CHANGE ME";
const WIFI_PASSWORD: &str = "CHANGE ME";

// LAPTOP SERVER DETAILS
// laptop runs server.py
// use ipconfig on powershell to find the wifi ipv4 address
const LAPTOP_IP: &str = "CHANGE ME";
const LAPTOP_PORT: u16 = 12345;

fn setup() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("code version: tcp test 2");
    info!("---------------------");
    info!("laptop runs server.py");
    info!("esp32 sends ping");
    info!("press access button to send stop");
}

fn main() -> Result<()> {
    setup();

    let peripherals = Peripherals::take()?;

    // access button on gpio42
    // external pull-down resistor, so floating is fine
    let mut access_button = PinDriver::input(peripherals.pins.gpio42, Pull::Floating)?;

    info!("access button ready");

    let modem = peripherals.modem;

    // keep wifi stored so the connection stays alive
    let _wifi = connect_to_wifi(modem)?;

    loop {
        info!("connecting to laptop tcp server...");

        match run_tcp_client(&mut access_button) {
            Ok(()) => {
                info!("tcp test stopped");
                break;
            }
            Err(error) => {
                error!("tcp client error: {:?}", error);
                warn!("trying again in 3 seconds...");
                FreeRtos::delay_ms(3000);
            }
        }
    }

    info!("stop sent. press reset to run it again");

    loop {
        FreeRtos::delay_ms(1000);
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

    info!("connecting to wifi...");
    wifi.connect()?;

    info!("waiting for wifi network...");
    wifi.wait_netif_up()?;

    info!("wifi connected");

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("esp32 ip: {}", ip_info.ip);

    Ok(wifi)
}

fn run_tcp_client(access_button: &mut PinDriver<'_, Input>) -> Result<()> {
    let server_address = format!("{}:{}", LAPTOP_IP, LAPTOP_PORT);

    info!("server address: {}", server_address);

    let mut stream = TcpStream::connect(server_address)?;

    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;

    info!("connected to laptop server");
    info!("sending ping every 2 seconds");
    info!("press access button to send stop");

    loop {
        if access_button.is_high() {
            info!("access button pressed");
            info!("sending stop...");

            stream.write_all(b"stop\n")?;
            stream.flush()?;

            read_server_reply(&mut stream)?;

            info!("closing tcp connection");
            break;
        }

        info!("sending ping...");

        stream.write_all(b"Ping\n")?;
        stream.flush()?;

        read_server_reply(&mut stream)?;

        // wait 2 seconds, but keep checking the button
        for _ in 0..20 {
            if access_button.is_high() {
                info!("access button pressed");
                info!("sending stop...");

                stream.write_all(b"stop\n")?;
                stream.flush()?;

                read_server_reply(&mut stream)?;

                info!("closing tcp connection");
                return Ok(());
            }

            FreeRtos::delay_ms(100);
        }
    }

    Ok(())
}

fn read_server_reply(stream: &mut TcpStream) -> Result<()> {
    let mut buffer = [0_u8; 64];

    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        warn!("server closed the connection");
        return Ok(());
    }

    let reply = std::str::from_utf8(&buffer[..bytes_read]).unwrap_or("");
    let reply = reply.trim();

    info!("from server: {}", reply);

    if reply == "ACK" {
        info!("esp: ack");
    } else if reply == "Stopping connection" {
        info!("server accepted stop");
    } else {
        warn!("unexpected reply from server");
    }

    Ok(())
}