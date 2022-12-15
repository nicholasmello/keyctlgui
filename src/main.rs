#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs, io::Write, io::Error};
use std::{num::ParseIntError};

use eframe::{egui, epaint::vec2};

// Location tuxedo keyboard driver installs driver files
const PATH: &str = "/sys/devices/platform/tuxedo_keyboard";

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(vec2(280.0, 160.0));
    eframe::run_native(
        "Keyboard Controller",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    hex: String,
    red: u8,
    green: u8,
    blue: u8,
    brightness: u8,
}

fn write_file(text: &String, file_name: &str) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(format!("{}/{}", PATH, file_name))?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

impl MyApp {
    fn update_color(&mut self) {
        let res = write_file(&format!("0x{:02x}{:02x}{:02x}", self.red, self.green, self.blue), "color_left");
        match res {
            Ok(_) => self.hex = format!("0x{:02x}{:02x}{:02x}", self.red, self.green, self.blue),
            Err(e) => println!("Set color failed with error message: {}\n0x{:02x}{:02x}{:02x}", e, self.red, self.green, self.blue)
        }
    }

    fn update_brightness(&mut self) {
        let res = write_file(&format!("{}", self.brightness), "brightness");
        match res {
            Ok(_) => (),
            Err(e) => println!("Set brightness failed with error message: {}", e)
        }
    }
}

fn get_color() -> Result<String, Error> {
    fs::read_to_string(format!("{}/color_left", PATH))
}

fn get_brightness() -> Result<String, Error> {
    fs::read_to_string(format!("{}/brightness", PATH))
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

impl Default for MyApp {
    fn default() -> Self {
        // Get initial colors
        let hex = get_color().expect("Color file not found, check clevo keyboard drivers are installed");
        let hex_slice: &str = &hex[..6];
        let vals = decode_hex(&hex_slice);
        let mut red: u8 = 255;
        let mut green: u8 = 255;
        let mut blue: u8 = 255;
        match vals {
            Ok(v) => {
                red = v[0];
                green = v[1];
                blue = v[2];
            },
            Err(e) => println!("Hex decoding failed with error message: {}", e)
        }

        // Get initial brightness
        let bright = get_brightness().expect("Brightness file not found, check clevo keyboard drivers are installed");
        let brightness = bright.trim().parse::<u8>().unwrap();

        Self {
            hex: format!("0x{:02x}{:02x}{:02x}", red, green, blue).to_owned(),
            red: red,
            green: green,
            blue: blue,
            brightness: brightness,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Keyboard Controller GUI");
            if ui.add(egui::Slider::new(&mut self.red, 0..=255).text("red")).changed() 
            | ui.add(egui::Slider::new(&mut self.green, 0..=255).text("green")).changed()
            | ui.add(egui::Slider::new(&mut self.blue, 0..=255).text("blue")).changed() {
                self.update_color();
            }
            if ui.add(egui::Slider::new(&mut self.brightness, 0..=255).text("brightness")).changed() {
                self.update_brightness();
            }
            if ui.text_edit_singleline(&mut self.hex).changed() {
                if self.hex.len() == 8 {
                    let vals = decode_hex(&self.hex[2..]);
                    match vals {
                        Ok(v) => {
                            self.red = v[0];
                            self.green = v[1];
                            self.blue = v[2];
                            self.update_color();
                        },
                        Err(e) => println!("Hex decoding failed with error message: {}", e)
                    }
                }
            }
        });
    }
}