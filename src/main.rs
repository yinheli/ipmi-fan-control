#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate anyhow;

use args::Command;
use chrono::Local;
use clap::Parser;
use ipmi::{Cmd, Ipmi, IpmiTool};
use log::{error, info};
use std::{io::Write, ops::RangeInclusive};
use tokio::time::{self, Duration};

mod args;
mod ipmi;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = args::Args::parse();

    let mut level = log::LevelFilter::Debug;

    if args.verbose {
        level = log::LevelFilter::Trace;
    }

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter_level(level)
        .init();

    let tool = IpmiTool::new(Box::new(Cmd::new()));

    match args.command {
        Command::Auto(a) => {
            let mut interval = a.interval;
            if !RangeInclusive::new(5, 120).contains(&interval) {
                interval = 5;
                info!("invalid interval, interval set to 5");
            }

            let mut threshold = a.threshold;
            if !RangeInclusive::new(60, 100).contains(&threshold) {
                threshold = 75;
                info!("invalid threshold, threshold set to {}", threshold);
            }

            info!(
                "auto mode start, interval: {}, threshold: {}",
                interval, threshold
            );

            let mut interval = time::interval(Duration::from_secs(interval));

            let mut last_speed = 0xff;

            loop {
                interval.tick().await;

                if let Ok(temperature) = tool.get_cpu_temperature() {
                    // transfer temperature to fan speed
                    let mut speed = match temperature {
                        0..=40 => 0,
                        41..=50 => 2,
                        51..=55 => 5,
                        56..=60 => 10,
                        61..=62 => 20,
                        63..=65 => 30,
                        66..=70 => 40,
                        71..=75 => 50,
                        76..=80 => 80,
                        81.. => 100,
                    };

                    if temperature >= threshold {
                        speed = 100;
                        info!("temperature reach threshold {}", temperature);
                    }

                    if last_speed != speed {
                        match tool.set_fan_speed(speed) {
                            Ok(_) => {
                                last_speed = speed;
                                info!("temperature: {}, set fan speed to {}", temperature, speed);
                            }
                            Err(e) => error!("failed to set fan speed: {}", e),
                        }
                    }
                } else {
                    error!("failed to get cpu temperature");
                }
            }
        }
        Command::Fixed { value } => {
            let mut v = value;
            if v > 100 {
                v = 100;
            }
            info!("fixed mode, set fan speed to {}", v);
            if let Err(e) = tool.set_fan_speed(v) {
                error!("set fan speed, error: {}", e);
            }
        }
        Command::Info => match tool.get_info_fan_temp() {
            Ok(info) => {
                println!("{}", info);
            }
            Err(err) => {
                error!("get info error: {}", err);
            }
        },
    }
}
