use chrono::{Local, Timelike};
use notify_rust::Notification;
use std::{
    fs, process::{self, Command, Stdio}, thread, time::Duration
};
use std::fmt::Display;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Time {
    Day,
    Afternoon,
    Night,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Time::Day => "Day",
            Time::Afternoon => "Afternoon",
            Time::Night => "Night",
        };

        write!(f, "{s}")
    }
}
#[derive(Serialize, Deserialize)]
struct Config {
    day_k: u32,
    afternoon_k: u32,
    night_k: u32,
    interval: u64,
    day_start: u32,
    afternoon_start: u32,
    night_start: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            day_k: 7000,
            afternoon_k: 6500,
            night_k: 5000,
            interval: 30,
            day_start: 7,
            afternoon_start: 12,
            night_start: 21,
        }
    }
}


impl Config {
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .expect("could not find config directory")
            .join("tempd")
            .join("config.toml")

    }
    fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = Config::config_path();
        if !config_path.exists() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let toml = toml::to_string_pretty(&Config::default())?;
            fs::write(&config_path, toml)?;
        }
        let toml = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&toml)?;
        Ok(config)
    }
}

impl Time {
    fn from_hour(hour: u32, config: &Config) -> Time {
        if hour >= config.night_start || hour < config.day_start {
            Time::Night
        } else if hour >= config.afternoon_start {
            Time::Afternoon
        } else {
            Time::Day
        }
    }
}

fn set_temperature(temp: u32) {
    Command::new("hyprsunset").stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap_or_else(|e| {
        eprintln!("failed to run hyprsunset: {e}");
        process::exit(1);
    });
    let status = Command::new("hyprctl")
        .args(["hyprsunset", "temperature", &temp.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
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

struct State {
    current_temp: u32,
    current_time: Time,
}

fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("failed to generate ({e})");
            process::exit(1);
        }
    };

    let mut current = None;
    let mut state = State {
        current_temp: config.day_k,
        current_time: Time::Day
    };
    println!("program is started!");
    loop {
        let hour = Local::now().hour();
        let time = Time::from_hour(hour, &config);

        if current != Some(time) {
            current = Some(time);

            match time {
                Time::Day => {
                    set_temperature(config.day_k);
                    state.current_time = Time::Day;
                    state.current_temp = config.day_k;
                    notify(format!("Switched to Day mode").as_str());
                }
                Time::Afternoon => {
                    set_temperature(config.afternoon_k);
                    state.current_time = Time::Afternoon;
                    state.current_temp = config.afternoon_k;
                    notify(format!("Switched to Afternoon mode").as_str());
                }
                Time::Night => {
                    set_temperature(config.night_k);
                    state.current_time = Time::Night;
                    state.current_temp = config.night_k;
                    notify(format!("Switched to Night mode").as_str());
                }
            }
        }
        println!("Currently it is the {}", state.current_time);
        println!("Screen temperature is {}°K", state.current_temp);
        thread::sleep(Duration::from_secs(config.interval.clone()));
    }
}
