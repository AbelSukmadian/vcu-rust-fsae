#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

// =================================================================
// FUNGSI KALIBRASI SOFTWARE (Zero-Offset Compensation)
// Mengonversi raw (1960 - 4095) menjadi persentase (0% - 100%)
// =================================================================
fn map_adc(raw: u16) -> f32 {
    let min_raw = 1960.0; // Angka dasar (offset) dari ESP32-S3 Anda
    let max_raw = 4095.0; // Angka puncak
    let mut val = raw as f32;
    
    // Safety clamp: memastikan nilai di bawah 1960 dibulatkan menjadi 0%
    if val < min_raw { 
        val = min_raw; 
    }
    // Safety clamp: memastikan nilai di atas 4095 dibulatkan menjadi 100%
    if val > max_raw { 
        val = max_raw; 
    }
    
    // Rumus Interpolasi Linier (Mapping)
    ((val - min_raw) / (max_raw - min_raw)) * 100.0
}

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut adc1_config = AdcConfig::new();
    let mut apps1_pin = adc1_config.enable_pin(peripherals.GPIO4, Attenuation::_11dB);
    let mut apps2_pin = adc1_config.enable_pin(peripherals.GPIO5, Attenuation::_11dB);
    let mut brake_pin = adc1_config.enable_pin(peripherals.GPIO6, Attenuation::_11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    println!("--------------------------------------------------");
    println!(" PROGRAM VCU HIL - DENGAN AUTO-KALIBRASI");
    println!(" Putar semua potensio ke KIRI MENTOK (0%) sekarang!");
    
    for i in (1..=5).rev() {
        println!("Merekam data dalam {} detik...", i);
        delay.delay_millis(1000);
    }
    
    println!("MULAI REKAM DATA SEKARANG!");
    println!("--------------------------------------------------");
    println!("waktu_ms,apps_1_pct,apps_2_pct,brake_pct,torque_out_pct");
    
    let mut time_ms: u32 = 0;

    loop {
        let raw1: u16 = loop { if let Ok(val) = adc1.read_oneshot(&mut apps1_pin) { break val; } };
        let raw2: u16 = loop { if let Ok(val) = adc1.read_oneshot(&mut apps2_pin) { break val; } };
        let raw3: u16 = loop { if let Ok(val) = adc1.read_oneshot(&mut brake_pin) { break val; } };

        // Melewati raw data ke fungsi kompensasi
        let apps1_pct = map_adc(raw1);
        let apps2_pct = map_adc(raw2);
        let brake_pct = map_adc(raw3);
        
        let apps_avg = (apps1_pct + apps2_pct) / 2.0;
        let deviasi = (apps1_pct - apps2_pct).abs();
        let torque_out: f32;

        // Algoritma Safety FSAE (EV.5.5 dan EV.5.6)
        if deviasi > 10.0 {
            torque_out = 0.0; // Kabel putus/Sensor rusak
        } else if brake_pct > 10.0 && apps_avg > 25.0 {
            torque_out = 0.0; // Panic Braking
        } else {
            torque_out = apps_avg; // Normal
        }

        println!("{},{:.1},{:.1},{:.1},{:.1}", time_ms, apps1_pct, apps2_pct, brake_pct, torque_out);

        delay.delay_millis(50);
        time_ms += 50;
        
        if time_ms > 20000 {
            println!("--------------------------------------------------");
            println!("SIMULASI HARDWARE SELESAI! Silakan copy data.");
            loop {} 
        }
    }
}