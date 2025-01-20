#![no_std]
#![no_main]

use core::str::FromStr;

use esp_backtrace as _;
use esp_hal::{ delay::Delay, gpio::{ Level, Output}, main, rng::Rng, time::{self, Duration}, timer::timg::TimerGroup};
extern crate alloc;
use smoltcp::iface::{SocketSet, SocketStorage};

#[main]
fn main() -> ! {
    
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    let delay = Delay::new();
    
    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();
    
    
    let timg0 = TimerGroup::new(peripherals.TIMG1);
    
    let mut random = Rng::new(peripherals.RNG);
    let wifi_init = esp_wifi::init(timg0.timer0, random, peripherals.RADIO_CLK).expect("error init");
    

    esp_wifi::wifi_set_log_verbose();

    log::info!("wifi_controller.wifi {:?}", wifi_init.wifi());
    
    

     let (iface, wifi_device, mut wifi_controller) = esp_wifi::wifi::utils::create_network_interface
     (&wifi_init,  peripherals.WIFI, esp_wifi::wifi::WifiApDevice)
    .unwrap();
    let now = || time::now().duration_since_epoch().to_millis();
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let socket_set = SocketSet::new(&mut socket_set_entries[..]);
    
    let mut stack = blocking_network_stack::Stack::new(iface, wifi_device, socket_set, now, random.random());
    
    stack
        .set_iface_configuration(&blocking_network_stack::ipv4::Configuration::Client(
            blocking_network_stack::ipv4::ClientConfiguration::Fixed(
                blocking_network_stack::ipv4::ClientSettings {
                    ip: blocking_network_stack::ipv4::Ipv4Addr::from(parse_ip("192.168.2.1")),
                    subnet: blocking_network_stack::ipv4::Subnet {
                        gateway: blocking_network_stack::ipv4::Ipv4Addr::from(parse_ip(
                            "192.168.2.1",
                        )),
                        mask: blocking_network_stack::ipv4::Mask(24),
                    },
                    dns: Some( blocking_network_stack::ipv4::Ipv4Addr::from(parse_ip("1.1.1.1"))),
                    secondary_dns: None,
                },
            ),
        ))
        .unwrap();


    let config_ap: esp_wifi::wifi::AccessPointConfiguration = esp_wifi::wifi::AccessPointConfiguration {
        ssid:  heapless::String::from_str("esp32_test").unwrap(),
        ssid_hidden: false,
        password: heapless::String::from_str("StrongPassword123!").unwrap(),
        auth_method: esp_wifi::wifi::AuthMethod::WPA2WPA3Personal,
        channel: 2,
        ..Default::default()
    };
    

    wifi_controller.set_configuration(  &esp_wifi::wifi::Configuration::AccessPoint(config_ap) ).expect("Error set config");
    let mut led =Output::new(peripherals.GPIO2, Level::High);
    
    wifi_controller.start().expect("error when start");
    log::info!(" is_started {:?} ", wifi_controller.is_started());
    log::info!(" is_ap_enabled {:?} ", wifi_controller.is_ap_enabled());
   
    loop {
        led.toggle();

        delay.delay(Duration::millis(1000));
    }
}

fn parse_ip(ip: &str) -> [u8; 4] {
    let mut result = [0u8; 4];
    for (idx, octet) in ip.split(".").into_iter().enumerate() {
        result[idx] = u8::from_str_radix(octet, 10).unwrap();
    }
    result
}