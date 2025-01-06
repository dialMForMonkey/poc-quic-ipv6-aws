#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{ delay::Delay, gpio::{ Level, Output}, prelude::*, rng::Rng, timer::timg::TimerGroup};


extern crate alloc;


#[entry]
fn main() -> ! {
    #[allow(unused)]
    let peripherals = esp_hal::init(esp_hal::Config::default());
    
    let delay = Delay::new();
    
    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();
    
    
    let timg0 = TimerGroup::new(peripherals.TIMG1);
    
    
    let wifi_init = esp_wifi::init(timg0.timer0, Rng::new(peripherals.RNG), peripherals.RADIO_CLK).expect("error init");
    
    log::info!("wifi_controller.wifi {:?}", wifi_init.wifi());
    
    let (wifi_device,mut wifi_controller) = esp_wifi::wifi::new_with_mode(&wifi_init, peripherals.WIFI, esp_wifi::wifi::WifiStaDevice ).expect("error start mode");
    
    let mut led =Output::new(peripherals.GPIO2, Level::High);
    let mac =wifi_device.mac_address();
    wifi_controller.start().expect("error when start");
    log::info!(" mac {:?} ", mac);
    log::info!(" is_started {:?} ", wifi_controller.is_started());
    loop {
        
        let (wifi_list,_) = wifi_controller.scan_n::<32>().expect("error scan");
       
        wifi_list.iter().for_each(|a|{
            log::info!("ssid {:?}", a.ssid);
        });
        
        led.toggle();

        delay.delay(1000.millis());
    }
}