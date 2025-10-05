use std::time::Duration;

use crate::{error, info};

#[derive(Debug, Clone)]
pub struct TimeFormat {
    name: String,
    description: String,
    initial: Duration,
    increment: Duration,
}

impl TimeFormat {
    pub fn formats() -> Vec<TimeFormat> {
        vec![
            TimeFormat {
                name: "m5;ps3;".to_string(),
                description: "5min game with a 3sec increment each turn".to_string(),
                initial: Duration::from_secs(5 * 60),
                increment: Duration::from_secs(3),
            },
            TimeFormat {
                name: "m10;".to_string(),
                description: "10min game".to_string(),
                initial: Duration::from_secs(10 * 60),
                increment: Duration::from_secs(0),
            },
        ]
    }

    fn get_number(str: &str) -> u64 {
        let num: Vec<&str> = str.split(";").collect();

        if let Some(num) = num.first() {
            let pnum = num.parse::<u64>();
            if let Ok(num) = pnum {
                return num;
            } else {
                panic!(" -> failed to parse number: {num:?}");
            }
        }

        return 0;
    }

    pub fn from(time_format: &str) -> TimeFormat {
        let mut seconds = 0;
        let mut plus_seconds = 0;

        let hour: Vec<&str> = time_format.split("H").collect();
        if let Some(hour) = hour.iter().nth(1) {
            let hour = TimeFormat::get_number(hour);

            seconds += hour * 60 * 60;
        }

        let minute: Vec<&str> = time_format.split("M").collect();
        if let Some(minute) = minute.iter().nth(1) {
            let minute = TimeFormat::get_number(minute);

            seconds += minute * 60
        }

        let second: Vec<&str> = time_format.split("S").collect();
        if let Some(second) = second.iter().nth(1) {
            let second = TimeFormat::get_number(second);

            seconds += second;
        }

        let plus_hour: Vec<&str> = time_format.split("h").collect();
        if let Some(hour) = plus_hour.iter().nth(1) {
            let hour = TimeFormat::get_number(hour);

            plus_seconds += hour * 60 * 60;
        }

        let plus_minute: Vec<&str> = time_format.split("m").collect();
        if let Some(minute) = plus_minute.iter().nth(1) {
            let minute = TimeFormat::get_number(minute);

            plus_seconds += minute * 60;
        }

        let plus_second: Vec<&str> = time_format.split("s").collect();
        if let Some(second) = plus_second.iter().nth(1) {
            let second = TimeFormat::get_number(second);
            plus_seconds += second;
        }

        let format = TimeFormat {
            name: "custom".into(),
            description: "custom time format made by the player".into(),
            initial: Duration::from_secs(seconds),
            increment: Duration::from_secs(plus_seconds),
        };


        info!(" -> input: {time_format}\ntime format: {format:#?}");


        format
    }

    pub fn default() -> TimeFormat {
        // TimeFormat::formats()[0]
        TimeFormat {
            name: ";M5;S3;".to_string(),
            description: "5min game with a 3sec increment each turn".to_string(),
            initial: Duration::from_secs(5 * 60),
            increment: Duration::from_secs(5),
        }
    }
}
