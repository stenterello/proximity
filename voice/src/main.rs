use esp_idf_sys as _;
use esp_idf_svc::wifi::{AccessPointConfiguration, Configuration, EspWifi};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::FreeRtos;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = EspWifi::new(peripherals.modem, sys_loop, Some(nvs))?;

    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "recall".try_into().unwrap(),
        ssid_hidden: false,
        channel: 1,
        ..Default::default()
    }))?;

    wifi.start()?;

    let mac: [u8; 6] = [0x4E, 0x45, 0x41, 0x52, 0x20, 0x55];
    unsafe {
        esp_idf_sys::esp_wifi_set_mac(esp_idf_sys::wifi_interface_t_WIFI_IF_AP, mac.as_ptr());
    }

    let mut actual_mac = [0u8; 6];
    unsafe {
        esp_idf_sys::esp_wifi_get_mac(esp_idf_sys::wifi_interface_t_WIFI_IF_AP, actual_mac.as_mut_ptr());
    }
    log::info!("MAC: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        actual_mac[0], actual_mac[1], actual_mac[2],
        actual_mac[3], actual_mac[4], actual_mac[5]);

    loop {
        FreeRtos::delay_ms(5000);
    }
}
