// BatWarn
// Low Battery Warning Utility
// Polls battery status and pops up warning when low.

#![allow(unstable)]
extern crate collections;
extern crate regex;
use std::io::IoResult;
use std::io::timer::sleep;
use std::io::fs::File;
use std::io::process::{Command, ProcessOutput};
use std::time::duration::Duration;
use std::num::from_str_radix;
use collections::string::String;
use regex::Regex;

static PERCENT_DANGER:     i32 = 20;
static PERCENT_CRITICAL:   i32 = 8;

#[derive(Debug)]
struct BatteryState {
    discharging: bool,
    percent: i32,
}

fn main() {
    // let poll_delay: Duration = Duration::minutes(5);
    let poll_delay: Duration = Duration::seconds(1);

    loop {
        // Kill existing warnings
        // TODO

        // Check battery status
        match acpi_battery_state() {
            Err(err) => {
                println!("ERROR: {}", err)
            },
            Ok(batstat) => {
                println!("{:?}", batstat);

                if !batstat.discharging {
                    // Do nothing
                } else if batstat.percent <= PERCENT_CRITICAL {
                    // Battery critically low.
                    // TODO
                } else if batstat.percent <= PERCENT_DANGER {
                    // Battery low.
                    // TODO
                } else {
                    // Battery discharging normally.
                }

            },
        }

        sleep(poll_delay);
    }
}

// Parse output from `acpi --battery`
fn acpi_battery_state() -> Result<BatteryState, String> {
    // ACPI output format: "Battery #{number}: #{state}, #{percent}%..."
    match acpi_battery_string() {
        Err(err) => Err(err),
        Ok(s) => {
            let re = Regex::new(r"Battery \d+: (\w+), (\d+)%.*").unwrap();
            match re.captures(s.as_slice()) {
                None => Err(String::from_str("malformed acpi output")),
                Some(captures) => {
                    let state = captures.at(1).unwrap();
                    let percent_str = captures.at(2).unwrap();
                    let percent = from_str_radix::<i32>(percent_str, 10).unwrap();
                    Ok(BatteryState {
                        discharging: state == "Discharging",
                        percent: percent,})
                }
            }
        },
    }
}

// Get output from `acpi --battery`
fn acpi_battery_string() -> Result<String, String> {
    let mut cmd = Command::new("acpi");
    cmd.arg("--battery");
    match cmd.output() {
        Err(err) => Err(String::from_str(err.desc)),
        Ok(ProcessOutput { status: exit, output: stdout, error: _ }) => {
            match exit.success() {
                false => Err(String::from_str("acpi returned with non-zero exit status")),
                true => {
                    let stdout = String::from_utf8(stdout).unwrap();
                    Ok(stdout)
                },
            }
        },
    }
}

// Read battery state from /proc.
#[allow(dead_code)]
fn read_battery_state_string() -> IoResult<String> {
    let path = Path::new("/proc/acpi/battery/BAT0/state");
    match File::open(&path) {
        Err(err) => Err(err),
        Ok(mut file) => file.read_to_string(),
    }
}

