use chrono::{Local, Timelike};
use notify_rust::Notification;
use std::{
    process::{self, Command}, thread, time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Time {
    Day,
    Afternoon,
    Night,
}

struct Config {
    day_k: u32,
    afternoon_k: u32,
    night_k: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            day_k: 7000,
            afternoon_k: 6500,
            night_k: 5000,
        }
    }
}

impl Time {
    fn from_hour(hour: u32) -> Self {
        match hour {
            7..=12 => Self::Day,
            13..=19 => Self::Afternoon,
            _ => Self::Night,
        }
    }
}

fn set_temperature(temp: u32) {
    Command::new("hyprsunset").spawn().unwrap_or_else(|e| {
        eprintln!("failed to run hyprsunset: {e}");
        process::exit(1);
    });
    let status = Command::new("hyprctl")
        .args(["hyprsunset", "temperature", &temp.to_string()])
        .status();

    if let Err(e) = status {
        eprintln!("Failed to run hyprctl: {e}");
    }
}

fn notify(msg: &str) {
    let _ = Notification::new()
        .summary("HyprSunset")
        .body(msg)
        .show();
}

fn main() {
    let config = Config::default();

    let mut current = None;

    loop {
        let hour = Local::now().hour();
        let time = Time::from_hour(hour);

        if current != Some(time) {
            current = Some(time);

            match time {
                Time::Day => {
                    set_temperature(config.day_k);
                    notify(format!("Switched to Day mode {}K", config.day_k).as_str());
                }
                Time::Afternoon => {
                    set_temperature(config.afternoon_k);
                    notify(format!("Switched to Afternoon mode {}K", config.afternoon_k).as_str());
                }
                Time::Night => {
                    set_temperature(config.night_k);
                    notify(format!("Switched to Night mode {}K", config.night_k).as_str());
                }
            }
        }
        thread::sleep(Duration::from_secs(60));
    }
}
